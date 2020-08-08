use crate::error::LoadError;
use crate::types::TimeScale;
use crate::vcd::VCD;

pub fn parse(s: &str) -> Result<VCD, LoadError> {
    let vcd = VCD {
        date: get_date_from_lines(s).unwrap(),
        version: get_version_from_lines(s).unwrap(),
        timescale: get_timescale_from_lines(s).unwrap(),
        comments: get_comments_from_lines(s).unwrap(),
    };
    Ok(vcd)
}

fn get_date_from_lines(lines: &str) -> Result<String, LoadError> {
    let mut parsed_date = CommandParser::new()
        .lines(lines)
        .command("$date")
        .enforce_only_one_of_command(true)
        .parse_command()
        .unwrap();
    Ok(parsed_date.remove(0))
}

fn get_version_from_lines(lines: &str) -> Result<String, LoadError> {
    let mut parsed_version = CommandParser::new()
        .lines(lines)
        .command("$version")
        .enforce_only_one_of_command(true)
        .parse_command()
        .unwrap();
    Ok(parsed_version.remove(0))
}

fn get_timescale_from_lines(lines: &str) -> Result<TimeScale, LoadError> {
    let timescale_str = CommandParser::new()
        .lines(lines)
        .command("$timescale")
        .enforce_only_one_of_command(true)
        .parse_command()
        .unwrap()
        .remove(0);
    Ok(TimeScale::load_from_str(timescale_str))
}

fn get_comments_from_lines(lines: &str) -> Result<Vec<String>, LoadError> {
    CommandParser::new()
        .lines(lines)
        .command("$comment")
        .enforce_only_one_of_command(false)
        .parse_command()
}

struct CommandParser<'a> {
    lines: &'a str,
    command: &'a str,
    enforce_only_one_of_command: bool,
}

impl<'a> CommandParser<'a> {
    pub fn new() -> CommandParser<'a> {
        CommandParser {
            lines: "",
            command: "",
            enforce_only_one_of_command: false,
        }
    }

    pub fn lines(&mut self, lines: &'a str) -> &'a mut CommandParser {
        self.lines = lines;
        self
    }

    pub fn command(&mut self, command_in: &'a str) -> &'a mut CommandParser {
        self.command = command_in;
        self
    }

    pub fn enforce_only_one_of_command(&mut self, enforcement: bool) -> &'a mut CommandParser {
        self.enforce_only_one_of_command = enforcement;
        self
    }

    fn parse_command(&self) -> Result<Vec<String>, LoadError> {
        let mut currently_parsing_command = false;
        let mut current_command_string = String::new();
        let mut command_vec = Vec::new();
        let mut line_num = 1;
        let words: Vec<_> = self.lines.split(" ").filter(|c| !c.is_empty()).collect();
        for word in words {
            let word_wo_newlines = word.replace("\n", "");

            if self.is_different_command(&word_wo_newlines, self.command)
                && currently_parsing_command
            {
                return Err(LoadError {
                    line: line_num,
                    info: format!("{} missing an $end", self.command),
                });
            }

            if self.is_end(&word_wo_newlines) && current_command_string.len() != 0 {
                currently_parsing_command = false;
                command_vec.push(current_command_string.trim().to_string());
                current_command_string = String::new();
            } else if currently_parsing_command {
                current_command_string = current_command_string + " " + &word_wo_newlines[..];
            } else if self.is_command(&word_wo_newlines, self.command) {
                if command_vec.len() != 0 && self.enforce_only_one_of_command {
                    return Err(LoadError {
                        line: line_num,
                        info: format!("Multiple {} commands is invalid", self.command),
                    });
                }
                currently_parsing_command = true;
            }

            if self.is_end_of_line(word) {
                line_num += 1;
            }
        }

        // Not finding any command in string is invalid
        if command_vec.len() == 0 {
            command_vec.push(String::new());
        }

        match currently_parsing_command {
            true => Err(LoadError {
                line: line_num,
                info: format!("{} missing an $end", self.command),
            }),
            false => Ok(command_vec),
        }
    }

    fn is_different_command(&self, word: &String, command: &str) -> bool {
        word.starts_with("$") && word != command && word != "$end"
    }

    fn is_end(&self, word: &String) -> bool {
        word == "$end"
    }

    fn is_command(&self, word: &String, command: &str) -> bool {
        word == command
    }

    fn is_end_of_line(&self, word: &str) -> bool {
        word.contains("\n")
    }
}
