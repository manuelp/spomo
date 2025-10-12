use error_stack::{Report, ResultExt};
use regex::Regex;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
#[error("a duration parsing error occured")]
pub struct ParsingError;

pub type DurationParsingResult<T> = Result<T, Report<ParsingError>>;

fn match_duration(input: &str, regex: &str) -> DurationParsingResult<Option<u64>> {
    let r = Regex::new(regex)
        .change_context(ParsingError)
        .attach("Invalid regex")?;
    Ok(r.captures(input).and_then(|capture| match capture.get(1) {
        Some(v) => v.as_str().parse::<u64>().ok(),
        None => None,
    }))
}

pub fn parse_duration(input: &str) -> DurationParsingResult<Duration> {
    let maybe_hours = match_duration(input, r"^(\d+)h$")?
        .map(|num_hours| Duration::from_secs(num_hours * 60 * 60));
    let maybe_minutes = match_duration(input, r"^(\d+)m$")?
        .map(|num_minutes| Duration::from_secs(num_minutes * 60));
    let maybe_seconds =
        match_duration(input, r"^(\d+)s$")?.map(|num_seconds| Duration::from_secs(num_seconds));

    maybe_hours
        .or(maybe_minutes)
        .or(maybe_seconds)
        .ok_or(Report::new(ParsingError).attach("invalid duration"))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn parsing_1h_duration_should_succeed() {
        let duration = parse_duration("1h").expect("should be parsed correctly");

        assert_eq!(Duration::from_secs(1 * 60 * 60), duration);
    }

    #[test]
    fn parsing_2h_duration_should_succeed() {
        let duration = parse_duration("2h").expect("should be parsed correctly");

        assert_eq!(Duration::from_secs(2 * 60 * 60), duration);
    }

    #[test]
    fn parsing_a_plain_number_should_fail() {
        let duration = parse_duration("15");

        assert!(duration.is_err());
    }

    #[test]
    fn parsing_25m_duration_should_succeed() {
        let duration = parse_duration("25m").expect("should be parsed correctly");

        assert_eq!(Duration::from_secs(25 * 60), duration);
    }

    #[test]
    fn parsing_30s_duration_should_succeed() {
        let duration = parse_duration("30s").expect("should be parsed correctly");

        assert_eq!(Duration::from_secs(30), duration);
    }

    #[test]
    fn parsing_an_empty_string_should_fail() {
        let duration = parse_duration("");

        assert!(duration.is_err());
    }
}
