use super::Parser;
use crate::legacy::{LegacyChannel, ParserError};
use crate::LegacyToolchainFile;
use std::path::{Path, PathBuf};
use yare::parameterized;

yare::ide!();

fn sample_path() -> &'static str {
    #[cfg(target_family = "windows")]
    {
        "C:/test"
    }
    #[cfg(target_family = "unix")]
    {
        "/test"
    }
}

#[parameterized(
    docs_example = { "nightly-2021-01-21" },
    nightly = { "nightly" },
    stable_version = { "1.37.0" },
    untrimmed_pre = { " nightly-2021-01-21" },
    untrimmed_post = { "nightly-2021-01-21 " },
)]
fn parse_ok_spec(content: &str) {
    let parser = Parser::new(content);

    let parsed = parser.parse();
    assert!(parsed.is_ok());

    let legacy_toolchain_file = parsed.unwrap();
    let spec = legacy_toolchain_file.spec();
    assert!(spec.is_some());

    let spec = spec.unwrap();

    assert_eq!(spec, content.trim());
}

#[test]
fn parse_ok_path() {
    let content = sample_path();
    let parser = Parser::new(content);

    let parsed = parser.parse();
    assert!(parsed.is_ok());

    let legacy_toolchain_file = parsed.unwrap();
    let path = legacy_toolchain_file.path();
    assert!(path.is_some());

    let spec = path.unwrap();

    assert_eq!(spec, Path::new(content));
}

#[test]
fn parse_ok_ascii() {
    let content = "hello";

    // Assert that a lenient parser would pass
    let lenient = Parser::new(content);

    let lenient_result = lenient.parse();
    assert!(lenient_result.is_ok());

    // Assert that the strict parser however does not
    let strict = Parser::strict(content);
    let strict_result = strict.parse();

    assert!(strict_result.is_ok());
}

#[parameterized(
    heart_emoji = { "❤️" },
    utf8_bom = { "a\u{FEFF}" },
    definitely_not_ascii = { &char::MAX.to_string() }
)]
fn parse_err_strictness(content: &str) {
    // Assert that a lenient parser would pass
    let lenient = Parser::new(content);

    let lenient_result = lenient.parse();
    assert!(lenient_result.is_ok());

    // Assert that the strict parser however does not
    let strict = Parser::strict(content);
    let strict_result = strict.parse();

    assert!(strict_result.is_err());
    assert_eq!(
        strict_result.unwrap_err(),
        ParserError::InvalidEncodingStrict
    )
}

#[parameterized(
    lenient = { |content: &str| Parser::new(content) },
    strict = { |content: &str| Parser::strict(content) },
)]
fn parse_err_empty(parser: fn(&str) -> Parser) {
    let content = "";

    let parser = parser(content);

    let result = parser.parse();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ParserError::IsEmpty);
}

#[parameterized(
    lenient = { |content: &str| Parser::new(content) },
    strict = { |content: &str| Parser::strict(content) },
)]
fn parse_err_line_count(parser: fn(&str) -> Parser) {
    let content = "a\nb";

    let parser = parser(content);

    let result = parser.parse();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ParserError::TooManyLines(2));
}

#[parameterized(
    lenient_path = { sample_path(), |content: &str| Parser::new(content), |content: &str| LegacyChannel::Path(PathBuf::from(content))  },
    strict_path = { sample_path(), |content: &str| Parser::strict(content), |content: &str| LegacyChannel::Path(PathBuf::from(content)) },
    lenient_spec = { "channel", |content: &str| Parser::new(content), |content: &str| LegacyChannel::Spec(String::from(content))  },
    strict_spec = { "channel", |content: &str| Parser::strict(content), |content: &str| LegacyChannel::Spec(String::from(content)) },
)]
fn get_channel(content: &str, parser: fn(&str) -> Parser, expected: fn(&str) -> LegacyChannel) {
    let parser = parser(content);
    let result = parser.parse();

    assert!(result.is_ok());

    let expected = expected(content);
    assert_eq!(result.unwrap().channel(), &expected);
}

#[parameterized(
    lenient = { |content: &str| Parser::new(content) },
    strict = { |content: &str| Parser::strict(content) },
)]
fn get_path(parser: fn(&str) -> Parser) {
    let content = sample_path();
    let parser = parser(content);
    let result = parser.parse();

    assert!(result.is_ok());
    let legacy_toolchain_file = result.unwrap();
    assert_eq!(legacy_toolchain_file.path(), Some(Path::new(content)));
    assert_eq!(legacy_toolchain_file.spec(), None);
}

#[parameterized(
    lenient = { |content: &str| Parser::new(content) },
    strict = { |content: &str| Parser::strict(content) },
)]
fn get_spec(parser: fn(&str) -> Parser) {
    let content = "channel";
    let parser = parser(content);
    let result = parser.parse();

    assert!(result.is_ok());
    let legacy_toolchain_file = result.unwrap();
    assert_eq!(legacy_toolchain_file.spec(), Some(content));
    assert_eq!(legacy_toolchain_file.path(), None);
}

#[parameterized(
    path = { sample_path(), |content: &str| Ok(LegacyToolchainFile { channel: LegacyChannel::Path(PathBuf::from(content)) }) },
    spec = { "channel", |content: &str| Ok(LegacyToolchainFile { channel: LegacyChannel::Spec(String::from(content)) }) },
    spec_lenient = { "😉", |content: &str| Ok(LegacyToolchainFile { channel: LegacyChannel::Spec(String::from(content)) }) },
    spec_leniet = { "a\nb", |_content: &str| Err(ParserError::TooManyLines(2)) },
)]
fn legacy_toolchain_file_from_str(
    content: &str,
    expected: fn(&str) -> Result<LegacyToolchainFile, ParserError>,
) {
    use std::str::FromStr;

    let result = LegacyToolchainFile::from_str(content);
    let expected = expected(content);

    assert_eq!(result, expected);
}

#[parameterized(
    heart_emoji = { "❤️" },
    utf8_bom = { "a\u{FEFF}" },
    definitely_not_ascii = { &char::MAX.to_string() }
)]
fn legacy_toolchain_file_from_str_is_lenient(content: &str) {
    use std::str::FromStr;

    let result = LegacyToolchainFile::from_str(content);
    let strict_parser = Parser::strict(content);
    let strict_result = strict_parser.parse();

    assert!(result.is_ok());
    assert!(strict_result.is_err());
}

#[parameterized(
    is_empty = { "", "Unable to parse legacy toolchain file: toolchain file was empty" },
    invalid_encoding_strict = { "\u{FFFF}", "Encountered invalid encoding while parsing legacy rust-toolchain file. The expected encoding to be US-ASCII, and lenient encoding was disabled." },
    too_many_lines = { "a\nb\nc", "Expected a single line containing the toolchain specifier but found '3' lines." },
)]
fn error_message(content: &str, error_message: &str) {
    let parser = Parser::strict(content);
    let result = parser.parse();

    assert!(result.is_err());

    let error = result.unwrap_err();
    let message = format!("{}", error);

    assert_eq!(&message, error_message);
}
