use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    /// Creates a new clock from hours and minutes
    /// Handles any integer input (negative or positive)
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Convert hours to minutes and add input minutes
        let total_minutes = hours * 60 + minutes;
        let (hours, minutes) = Self::normalize_time(total_minutes);
        Self { hours, minutes }
    }

    /// Adds (or subtracts) minutes to the current time
    /// Returns a new Clock instance
    pub fn add_minutes(&self, minutes: i32) -> Self {
        // Convert current time to minutes and add new minutes
        let total_minutes = self.hours * 60 + self.minutes + minutes;
        let (hours, minutes) = Self::normalize_time(total_minutes);
        Self { hours, minutes }
    }

    /// Private helper method to handle time conversion and normalization
    /// Takes total minutes and returns a tuple of normalized (hours, minutes)
    ///
    /// Examples:
    /// - 150 minutes -> (2, 30) // 2 hours and 30 minutes
    /// - -5 hours (-300 minutes) -> (19, 0) // 19 hours and 0 minutes
    /// - 25 hours (1500 minutes) -> (1, 0) // 1 hour and 0 minutes
    fn normalize_time(total_minutes: i32) -> (i32, i32) {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format with zero padding to ensure 2 digits
        // Example: 2:5 becomes "02:05"
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
