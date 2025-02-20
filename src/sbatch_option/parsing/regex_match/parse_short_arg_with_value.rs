use regex::Regex;
use super::RegexMatch;

pub fn parse(input: &str) -> Option<RegexMatch> {
    Regex::new(r"^-(\w)\s(.+)$")
        .ok()?
        .captures(input.trim())
        .and_then(|captures| {
            Some(RegexMatch::ShortArgWithValue(
                captures.get(1)?.as_str(),
                captures.get(2)?.as_str(),
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("-o value", "o", "value")]
    #[case("-o v", "o", "v")]
    fn test_parse(#[case] input: &str, #[case] key: &str, #[case] value: &str) {
        let result = parse(input);
        assert!(result.is_some());

        let (result_key, result_value) = match result.unwrap() {
            RegexMatch::ShortArgWithValue(key, value) => (key, value),
            _ => panic!("Expected ShortArgWithValue"),
        };
        assert_eq!(result_key, key);
        assert_eq!(result_value, value);
    }

    #[rstest]
    #[case("invalid")]
    #[case("invalid=")]
    #[case("invalid value")]
    #[case("invalid value=")]
    #[case("--invalid")]
    #[case("--invalid=")]
    #[case("--invalid  ")]
    #[case("--invalid=  ")]
    #[case("---invalid =")]
    #[case("--i")]
    #[case("--i=value")]
    #[case("--i value")]
    #[case("--i=")]
    #[case("--i  ")]
    #[case("--i =")]
    #[case("")]
    #[case("-i")]
    #[case("-i=value")]
    fn test_parse_invalid(#[case] input: &str) {
        let result = parse(input);
        assert!(result.is_none());
    }
}