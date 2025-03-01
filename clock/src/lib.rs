use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    /// Creates a new Clock instance from the given hours and minutes.
    /// Normalizes any input values (positive or negative) to valid 24-hour time.
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = Self::normalize_time(hours, minutes);
        Self { hours, minutes }
    }

    /// Adds (or subtracts if negative) minutes to the current time.
    /// Returns a new normalized Clock instance.
    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours, minutes) = Self::normalize_time(self.hours, self.minutes + minutes);
        Self { hours, minutes }
    }

    /// Normalizes any time input to valid 24-hour clock values.
    ///
    /// Returns a tuple of (hours, minutes) where:
    /// - hours is in range 0-23
    /// - minutes is in range 0-59
    ///
    /// Examples:
    /// normalize_time(25, 0)   returns (1, 0)    // Wraps 25 hours to 1:00
    /// normalize_time(-1, -30) returns (22, 30)  // Converts negative time to 22:30
    /// normalize_time(0, 150)  returns (2, 30)   // Converts 150 minutes to 2:30
    fn normalize_time(hours: i32, minutes: i32) -> (i32, i32) {
        // Convert hours to minutes and add input minutes
        let total_minutes = hours * 60 + minutes;
        // Use rem_euclid to handle negative numbers and overflow
        // 24 * 60 = 1440 minutes in a day
        let normalized_minutes = total_minutes.rem_euclid(24 * 60);
        (
            normalized_minutes / 60, // Convert to hours (0-23)
            normalized_minutes % 60, // Get remaining minutes (0-59)
        )
    }
}

impl Display for Clock {
    /// Formats the time as HH:MM with leading zeros.
    ///
    /// Example:
    /// - 2:5 is formatted as "02:05"
    /// - 15:30 is formatted as "15:30"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format with zero padding to ensure 2 digits
        // Example: 2:5 becomes "02:05"
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
