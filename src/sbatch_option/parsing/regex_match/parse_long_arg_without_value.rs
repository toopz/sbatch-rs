use regex::Regex;
use super::RegexMatch;

pub fn parse(input: &str) -> Option<RegexMatch> {
    Regex::new(r"^--(\w[\w-]+)$")
        .ok()?
        .captures(input.trim())?
        .get(1)
        .map(|m| RegexMatch::LongArg(m.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("--option", "option")]
    #[case("--option-with-dash", "option-with-dash")]
    fn test_parse(#[case] input: &str, #[case] key: &str) {
        let result = parse(input);
        assert!(result.is_some());

        let result_key = match result.unwrap() {
            RegexMatch::LongArg(key) => key,
            _ => panic!("Expected LongArg"),
        };
        assert_eq!(result_key, key);
    }

    #[rstest]
    #[case("invalid")]
    #[case("invalid=")]
    #[case("invalid value")]
    #[case("invalid value=")]
    #[case("--invalid=x")]
    #[case("--invalid x")]
    #[case("--invalid x=")]
    #[case("--i")]
    #[case("--i=value")]
    #[case("--i value")]
    #[case("--i=")]
    #[case("--i  ")]
    #[case("--i =")]
    #[case("")]
    #[case("-i")]
    #[case("-i=value")]
    #[case("-i value")]
    fn test_parse_invalid(#[case] input: &str) {
        let result = parse(input);
        assert!(result.is_none());
    }
}