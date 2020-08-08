use crate::error::LoadError;
use crate::parser;
use crate::types::TimeScale;

pub struct VCD {
    pub date: String,
    pub version: String,
    pub timescale: TimeScale,
    pub comments: Vec<String>,
}

impl VCD {
    pub fn new() -> VCD {
        VCD { date: String::new(), version: String::new(), timescale: TimeScale::new(), comments: Vec::new() }
    }
}

pub struct VCDLoader;

impl VCDLoader {
    pub fn load_from_str(s: &str) -> Result<VCD, LoadError> {
        let vcd = parser::parse(s)?;
        Ok(vcd)
    }

    #[allow(unused_variables)]
    pub fn load_from_file(f: &str) -> Result<VCD, LoadError> {
        Ok(VCD::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TimeUnit;

    #[test]
    fn date_command() {
        let contents = "$date Date text $end";
        let vcd = VCDLoader::load_from_str(&contents).unwrap();
        assert_eq!(vcd.date, "Date text".to_string());
    }

    #[test]
    fn date_command_newline() {
        let contents = r#"$date
            Date text
        $end"#;
        let vcd = VCDLoader::load_from_str(&contents).unwrap();
        assert_eq!(vcd.date, "Date text".to_string());
    }

    #[test]
    #[should_panic(expected = "$date missing an $end")]
    fn date_command_with_no_end_throws_load_error() {
        let contents = r#"$date
            Date text"#;
        VCDLoader::load_from_str(&contents).unwrap();
    }

    #[test]
    #[should_panic(expected = "$date missing an $end")]
    fn date_command_with_no_end_and_new_command_begins_throws_load_error() {
        let contents = r#"$date
            Date text
        $version
            The version is 1.0
        $end"#;
        VCDLoader::load_from_str(&contents).unwrap();
    }

    #[test]
    fn version_command_multiple_newlines() {
        let contents = r#"$version

            The version number is 1.1

        $end"#;
        let vcd = VCDLoader::load_from_str(&contents).unwrap();
        assert_eq!(vcd.version, "The version number is 1.1");
    }

    #[test]
    fn version_command() {
        let contents = r#"$version This version number is 2.0 $end"#;
        let vcd = VCDLoader::load_from_str(&contents).unwrap();
        assert_eq!(vcd.version, "This version number is 2.0");
    }

    #[test]
    #[should_panic(expected = "$version missing an $end")]
    fn version_command_with_no_end_throws_load_error() {
        let contents = r#"$version
            This version has no end"#;
        VCDLoader::load_from_str(&contents).unwrap();
    }

    #[test]
    #[should_panic(expected = "Multiple $version commands is invalid")]
    fn vcd_file_with_multiple_versions_throws_error() {
        let contents = r#"$version
            Version 1.0
        $end
        $version
            Version 2.0. Which version is the right version?
        $end"#;
        VCDLoader::load_from_str(&contents).unwrap();
    }

    #[test]
    #[should_panic(expected = "Multiple $date commands is invalid")]
    fn vcd_file_with_multiple_dates_throws_error() {
        let contents = r#"$date
            May 31st, 2020
        $end
        $date
            August 9th, 2020. Which is the correct date?
        $end"#;
        VCDLoader::load_from_str(&contents).unwrap();
    }

    #[test]
    fn timescale_command() {
        let contents = "$timescale 1ps $end";
        let vcd = VCDLoader::load_from_str(&contents).unwrap();
        assert_eq!(vcd.timescale, TimeScale{ value: 1, unit: TimeUnit::PS });
    }

    #[test]
    fn comment_command_with_one_comment() {
        let contents = "$comment this is a comment $end";
        let vcd = VCDLoader::load_from_str(&contents).unwrap();
        assert_eq!(vcd.comments, vec!["this is a comment"]);
    }

    #[test]
    fn comment_command_with_multiple_comments() {
        let contents = r#"$comment
            This is comment 1
        $end
        $comment
            This is comment 2
        $end"#;
        let vcd = VCDLoader::load_from_str(&contents).unwrap();
        assert_eq!(vcd.comments, vec!["This is comment 1", "This is comment 2"]);
    }

    #[test]
    #[should_panic(expected = "$comment missing an $end")]
    fn comment_command_with_no_end_throws_load_error() {
        let contents = "$comment This comment is missing an end";
        VCDLoader::load_from_str(&contents).unwrap();
    }

}
