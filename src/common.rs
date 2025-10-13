//! Functionality shared between features

pub fn format_time(seconds: u64) -> String {
    let hours = (seconds / 60 / 60) % 24;
    let minutes = (seconds / 60) % 60;
    let seconds = seconds % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_seconds() {
        let formatted = format_time(12);

        assert_eq!("00:00:12", formatted);
    }

    #[test]
    fn format_zero_seconds() {
        let formatted = format_time(0);

        assert_eq!("00:00:00", formatted);
    }

    #[test]
    fn format_minutes() {
        let formatted = format_time(12 * 60 + 25);

        assert_eq!("00:12:25", formatted);
    }

    #[test]
    fn format_hours() {
        let formatted = format_time(11 * 60 * 60 + 25 * 60 + 47);

        assert_eq!("11:25:47", formatted);
    }
}