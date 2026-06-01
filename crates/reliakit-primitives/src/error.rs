use core::fmt;

/// Error returned when a primitive value fails validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveError {
    /// The value was empty or contained only whitespace.
    Empty,
    /// The value was shorter than the minimum allowed length.
    TooShort { min: usize, actual: usize },
    /// The value was longer than the maximum allowed length.
    TooLong { max: usize, actual: usize },
    /// The value was outside the inclusive allowed range.
    OutOfRange { min: u128, max: u128, actual: u128 },
}

/// Result alias used by Reliakit primitive constructors.
pub type PrimitiveResult<T> = Result<T, PrimitiveError>;

impl fmt::Display for PrimitiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("value must not be empty"),
            Self::TooShort { min, actual } => {
                write!(
                    f,
                    "value is too short: minimum is {min}, actual is {actual}"
                )
            }
            Self::TooLong { max, actual } => {
                write!(f, "value is too long: maximum is {max}, actual is {actual}")
            }
            Self::OutOfRange { min, max, actual } => {
                write!(
                    f,
                    "value is out of range: expected {min}..={max}, actual is {actual}"
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PrimitiveError {}

#[cfg(test)]
mod tests {
    use super::PrimitiveError;
    use alloc::string::ToString;

    #[test]
    fn display_empty() {
        assert_eq!(PrimitiveError::Empty.to_string(), "value must not be empty");
    }

    #[test]
    fn display_too_short() {
        assert_eq!(
            PrimitiveError::TooShort { min: 3, actual: 1 }.to_string(),
            "value is too short: minimum is 3, actual is 1"
        );
    }

    #[test]
    fn display_too_long() {
        assert_eq!(
            PrimitiveError::TooLong { max: 5, actual: 8 }.to_string(),
            "value is too long: maximum is 5, actual is 8"
        );
    }

    #[test]
    fn display_out_of_range() {
        assert_eq!(
            PrimitiveError::OutOfRange {
                min: 1,
                max: 100,
                actual: 200
            }
            .to_string(),
            "value is out of range: expected 1..=100, actual is 200"
        );
    }
}
