const LEGACY_ONLY: &'static str = include_str!("../tests/fixtures/legacy-only/rust-toolchain");

const TOML_LOCAL_PATH: &'static str =
    include_str!("../tests/fixtures/rustup-book-local-toolchain/rust-toolchain.toml");

const TOML_WITH_EXT: &'static str =
    include_str!("../tests/fixtures/rustup-book-layout/rust-toolchain.toml");

const TOML_WITHOUT_EXT: &'static str =
    include_str!("../tests/fixtures/toml-without-ext/rust-toolchain");

mod parser_new {
    use crate::{ParseStrategy, Parser, Variant};
    use yare::parameterized;

    #[parameterized(
        only_legacy = { ParseStrategy::Only(Variant::Legacy) },
        only_toml = { ParseStrategy::Only(Variant::Toml) },
        fallback_legacy_to_toml = {
            ParseStrategy::Fallback {
                first: Variant::Legacy,
                fallback_to: Variant::Toml,
            }
        },
        fallback_toml_to_legacy= {
            ParseStrategy::Fallback {
                first: Variant::Toml,
                fallback_to: Variant::Legacy,
            }
        },
    )]
    fn new(option: ParseStrategy) {
        let content = "hello-world";
        let parser = Parser::new(content, option);

        assert_eq!(parser.content, content);
        assert_eq!(parser.parse_option, option);
    }
}

mod legacy_only {
    use crate::legacy::LegacyChannel;
    use crate::tests::{LEGACY_ONLY, TOML_LOCAL_PATH};
    use crate::{legacy, LegacyToolchainFile, ToolchainFile};
    use crate::{ParseStrategy, Parser, ParserError, Variant};

    #[test]
    fn accept_legacy() {
        let option = ParseStrategy::Only(Variant::Legacy);
        let parser = Parser::new(LEGACY_ONLY, option);

        let result = parser.parse();
        assert_eq!(
            result.unwrap(),
            ToolchainFile::Legacy(LegacyToolchainFile::new(LegacyChannel::Spec(
                "nightly-2020-07-10".to_string()
            )))
        )
    }

    #[test]
    fn reject_toml() {
        let option = ParseStrategy::Only(Variant::Legacy);
        let parser = Parser::new(TOML_LOCAL_PATH, option);

        let result = parser.parse();
        assert_eq!(
            result.unwrap_err(),
            ParserError::LegacyParseError(legacy::ParserError::TooManyLines(2))
        )
    }
}

mod toml_only {
    use crate::tests::{LEGACY_ONLY, TOML_LOCAL_PATH, TOML_WITHOUT_EXT, TOML_WITH_EXT};
    use crate::toml;
    use crate::ToolchainFile;
    use crate::{ParseStrategy, Parser, ParserError, Variant};
    use yare::parameterized;

    #[parameterized(
        toml_local_path = { TOML_LOCAL_PATH },
        toml_with_ext = { TOML_WITH_EXT },
        toml_without_ext = { TOML_WITHOUT_EXT },
    )]
    fn accept_toml(content: &str) {
        let option = ParseStrategy::Only(Variant::Toml);
        let parser = Parser::new(content, option);

        let result = parser.parse();
        assert!(matches!(result.unwrap(), ToolchainFile::Toml(_)))
    }

    #[test]
    fn reject_legacy() {
        let option = ParseStrategy::Only(Variant::Toml);
        let parser = Parser::new(LEGACY_ONLY, option);

        let result = parser.parse();
        assert!(matches!(
            result.unwrap_err(),
            ParserError::TomlParseError(toml::ParserError::TomlParse(_))
        ));
    }
}

mod fallback {
    use crate::legacy;
    use crate::legacy::LegacyChannel;
    use crate::tests::{LEGACY_ONLY, TOML_WITHOUT_EXT};
    use crate::{LegacyToolchainFile, ParseStrategy, Parser, ParserError, ToolchainFile, Variant};

    #[test]
    fn accept_legacy_first_try() {
        let strategy = ParseStrategy::Fallback {
            first: Variant::Legacy,
            fallback_to: Variant::Toml,
        };

        let parser = Parser::new(LEGACY_ONLY, strategy);

        let result = parser.parse();
        assert_eq!(
            result.unwrap(),
            ToolchainFile::Legacy(LegacyToolchainFile::new(LegacyChannel::Spec(
                "nightly-2020-07-10".to_string()
            )))
        );
    }

    #[test]
    fn fallback_to_toml() {
        let strategy = ParseStrategy::Fallback {
            first: Variant::Legacy,
            fallback_to: Variant::Toml,
        };

        let parser = Parser::new(TOML_WITHOUT_EXT, strategy);

        let result = parser.parse();
        assert!(matches!(result.unwrap(), ToolchainFile::Toml(_)));
    }

    #[test]
    fn fail_to_parse() {
        let strategy = ParseStrategy::Fallback {
            first: Variant::Legacy,
            fallback_to: Variant::Toml,
        };

        let parser = Parser::new("", strategy);

        let result = parser.parse();
        let error = result.unwrap_err();

        assert!(matches!(error, ParserError::FallbackError(_)));

        if let ParserError::FallbackError(inner) = error {
            assert_eq!(
                inner.first(),
                &ParserError::LegacyParseError(legacy::ParserError::IsEmpty)
            );
            assert!(matches!(
                inner.fallback_to(),
                ParserError::TomlParseError(_)
            ));
        }
    }
}
