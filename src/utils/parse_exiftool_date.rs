use snafu::prelude::*;
use time::OffsetDateTime;
use time::macros::format_description;

pub fn parse_time_to_exiftool_format(time: &OffsetDateTime) -> Result<String, ExiftoolDateError> {
    let exif_format = format_description!(
        "[year]:[month]:[day] [hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]"
    );

    Ok(time.format(&exif_format).unwrap())
}

pub fn parse_to_exiftool_format(date_str: &str) -> Result<String, ExiftoolDateError> {
    let datetime = parse_custom_format(date_str)?;

    let exif_format = format_description!(
        "[year]:[month]:[day] [hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]"
    );

    Ok(datetime.format(&exif_format).unwrap())
}

fn parse_custom_format(date_str: &str) -> Result<OffsetDateTime, ExiftoolDateError> {
    let format = format_description!(
        "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour][offset_minute]"
    );

    OffsetDateTime::parse(date_str, &format).context(ParseSnafu)
}

#[derive(Snafu, Debug)]
pub enum ExiftoolDateError {
    #[snafu(display("Failed to parse date: {}", source))]
    Parse { source: time::error::Parse },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let input = "2025-11-16T21:17:25+0100";
        let result = parse_to_exiftool_format(input).unwrap();
        assert_eq!(result, "2025:11:16 21:17:25+01:00");
    }

    #[test]
    fn test_negative_timezone() {
        let input = "2025-10-23T20:06:34-0500";
        let result = parse_to_exiftool_format(input).unwrap();
        assert_eq!(result, "2025:10:23 20:06:34-05:00");
    }

    #[test]
    fn test_utc_timezone() {
        let input = "2025-11-16T21:17:25+0000";
        let result = parse_to_exiftool_format(input).unwrap();
        assert_eq!(result, "2025:11:16 21:17:25+00:00");
    }
}
