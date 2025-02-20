//! Regex match for parsing sbatch options.

mod parse_long_arg_with_value;
mod parse_long_arg_without_value;
mod parse_short_arg_with_value;
mod parse_short_arg_without_value;

/// Represents a regex match for parsing sbatch options.
///
/// The `RegexMatch` enum represents a regex match for parsing sbatch options.
pub enum RegexMatch<'a> {
    /// Long argument without a value, for example: `--option`
    LongArg(&'a str),
    /// Long argument with a value, for example: `--option=value`
    LongArgWithValue(&'a str, &'a str),
    /// Short argument without a value, for example: `-o`
    ShortArg(&'a str),
    /// Short argument with a value, for example: `-o value`
    ShortArgWithValue(&'a str, &'a str),
}

impl<'a> RegexMatch<'a> {
    /// Returns the key of the regex match.
    ///
    /// For example:
    /// - `--option` would return `option`.
    /// - `--option=value` would return `option`.
    /// - `-o` would return `o`.
    /// - `-o value` would return `o`.
    ///
    /// # Arguments
    ///
    /// * `self` - The `RegexMatch` enum
    ///
    /// # Returns
    ///
    /// The key of the regex match as a string.
    pub fn key(&self) -> &'a str {
        match self {
            RegexMatch::LongArg(key) => key,
            RegexMatch::LongArgWithValue(key, _) => key,
            RegexMatch::ShortArg(key) => key,
            RegexMatch::ShortArgWithValue(key, _) => key,
        }
    }

    /// Returns the value of the regex match.
    ///
    /// For example:
    /// - `--option` would return `None`.
    /// - `--option=value` would return `Some("value")`.
    /// - `-o` would return `None`.
    /// - `-o value` would return `Some("value")`.
    ///
    /// # Arguments
    ///
    /// * `self` - The `RegexMatch` enum
    ///
    /// # Returns
    ///
    /// The value of the regex match as an optional string.
    /// Returns `None` if the regex match does not have a value.
    pub fn value(&self) -> Option<&'a str> {
        match self {
            RegexMatch::LongArg(_) => None,
            RegexMatch::LongArgWithValue(_, value) => Some(value),
            RegexMatch::ShortArg(_) => None,
            RegexMatch::ShortArgWithValue(_, value) => Some(value),
        }
    }
}

impl<'a> RegexMatch<'a> {
    /// Parses a string into a `RegexMatch`.
    ///
    /// The input string is parsed using regex patterns to determine the type of match.
    /// - RegexMatch::LongArg if the input string is a long argument without a value, for example: `--option`.
    /// - RegexMatch::LongArgWithValue if the input string is a long argument with a value, for example: `--option=value`.
    /// - RegexMatch::ShortArg if the input string is a short argument without a value, for example: `-o`.
    /// - RegexMatch::ShortArgWithValue if the input string is a short argument with a value, for example: `-o value`.
    ///
    /// # Arguments
    ///
    /// * `value` - The input string to parse
    ///
    /// # Returns
    ///
    /// A `RegexMatch` enum if the input string matches a regex pattern, otherwise `None`.
    pub fn from_str(value: &'a str) -> Option<Self> {
        parse_long_arg_with_value::parse(value)
            .or_else(|| parse_long_arg_without_value::parse(value))
            .or_else(|| parse_short_arg_with_value::parse(value))
            .or_else(|| parse_short_arg_without_value::parse(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("--option=value")]
    #[case("--option value")]
    #[case("-o value")]
    #[case("-o")]
    #[case("--option-with-dash value")]
    fn test_from(#[case] value: &str) {
        let regex_match = RegexMatch::from_str(value);
        assert!(regex_match.is_some());
    }

    #[rstest]
    #[case("--option=value", "option", Some("value"))]
    #[case("--option value", "option", Some("value"))]
    #[case("-o value", "o", Some("value"))]
    #[case("-o", "o", None)]
    fn test_key_and_value(
        #[case] input_string: &str,
        #[case] key: &str,
        #[case] value: Option<&str>,
    ) {
        let regex_match = RegexMatch::from_str(input_string).unwrap();
        assert_eq!(regex_match.key(), key);
        assert_eq!(regex_match.value(), value);
    }

    #[rstest]
    #[case("invalid")]
    #[case("invalid=")]
    #[case("invalid value")]
    #[case("invalid value=")]
    fn test_from_invalid(#[case] value: &str) {
        let regex_match = RegexMatch::from_str(value);
        assert!(regex_match.is_none());
    }
}
