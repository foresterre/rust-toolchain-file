use crate::toml::{Channel, Parser, ParserError, ToolchainSpec};
use crate::RustToolchainToml;
use camino::Utf8Path;

const RUSTUP_BOOK_SPEC: &'static str =
    include_str!("../../tests/fixtures/rustup-book-layout/rust-toolchain.toml");

const RUSTUP_BOOK_LOCAL_TOOLCHAIN: &'static str =
    include_str!("../../tests/fixtures/rustup-book-local-toolchain/rust-toolchain.toml");

mod complete_file {
    use super::*;

    #[test]
    fn parse_rustup_book_layout() {
        let rust_toolchain: RustToolchainToml = toml_edit::de::from_str(RUSTUP_BOOK_SPEC).unwrap();

        let toolchain = rust_toolchain.toolchain();
        assert!(toolchain.path().is_none());

        let spec = toolchain.spec();
        assert!(spec.is_some());

        let spec = spec.unwrap();

        let channel = spec.channel();
        assert!(channel.is_some());
        assert_eq!(channel.unwrap().name(), "nightly-2020-07-10");

        let components = spec.components();
        assert!(components.is_some());
        assert_eq!(
            components
                .unwrap()
                .iter()
                .map(|c| c.name())
                .collect::<Vec<_>>(),
            vec!["rustfmt", "rustc-dev"]
        );

        let targets = spec.targets();
        assert!(targets.is_some());
        assert_eq!(
            targets
                .unwrap()
                .iter()
                .map(|c| c.name())
                .collect::<Vec<_>>(),
            vec!["wasm32-unknown-unknown", "thumbv2-none-eabi"]
        );

        let profile = spec.profile();
        assert!(profile.is_some());
        assert_eq!(profile.unwrap().name(), "minimal");
    }

    #[test]
    fn parse_rustup_book_layout_with_parser() {
        let p = Parser::new(RUSTUP_BOOK_SPEC);

        let result = p.parse();
        assert!(result.is_ok());

        let toolchain_file = result.unwrap();
        let toolchain = toolchain_file.toolchain();

        let spec = toolchain.spec();
        assert!(spec.is_some());

        let spec = spec.unwrap();

        let channel = spec.channel();
        assert!(channel.is_some());
        assert_eq!(channel.unwrap().name(), "nightly-2020-07-10");

        let components = spec.components();
        assert!(components.is_some());
        assert_eq!(
            components
                .unwrap()
                .iter()
                .map(|c| c.name())
                .collect::<Vec<_>>(),
            vec!["rustfmt", "rustc-dev"]
        );

        let targets = spec.targets();
        assert!(targets.is_some());
        assert_eq!(
            targets
                .unwrap()
                .iter()
                .map(|c| c.name())
                .collect::<Vec<_>>(),
            vec!["wasm32-unknown-unknown", "thumbv2-none-eabi"]
        );

        let profile = spec.profile();
        assert!(profile.is_some());
        assert_eq!(profile.unwrap().name(), "minimal");
    }

    #[test]
    fn parse_rustup_book_local_toolchain() {
        let rust_toolchain: RustToolchainToml =
            toml_edit::de::from_str(RUSTUP_BOOK_LOCAL_TOOLCHAIN).unwrap();

        let toolchain = rust_toolchain.toolchain();
        let path = toolchain.path();

        assert!(path.is_some());
        let path = path.unwrap().path();

        assert_eq!(path, Utf8Path::new("/path/to/local/toolchain"));
    }

    #[test]
    fn parse_rustup_book_local_toolchain_with_parser() {
        let parser = Parser::new(RUSTUP_BOOK_LOCAL_TOOLCHAIN);
        let result = parser.parse();
        assert!(result.is_ok());

        let rust_toolchain = result.unwrap();

        let toolchain = rust_toolchain.toolchain();
        let path = toolchain.path();

        assert!(path.is_some());
        let path = path.unwrap().path();

        assert_eq!(path, Utf8Path::new("/path/to/local/toolchain"));
    }
}

mod from_slice {
    use super::*;

    #[test]
    fn parse_with_parser_from_slice() {
        let parser = Parser::from_slice(RUSTUP_BOOK_SPEC.as_bytes());

        assert!(parser.parse().is_ok());
    }
}

mod getters {
    use super::*;

    #[test]
    fn toolchain_section_getters_path() {
        let parser = Parser::new(RUSTUP_BOOK_LOCAL_TOOLCHAIN);
        let toolchain = parser.parse().unwrap();

        let path = toolchain.toolchain().path();
        assert!(path.is_some());

        let spec = toolchain.toolchain().spec();
        assert!(spec.is_none());
    }

    #[test]
    fn toolchain_section_getters_spec() {
        let parser = Parser::new(RUSTUP_BOOK_SPEC);
        let toolchain = parser.parse().unwrap();

        let path = toolchain.toolchain().path();
        assert!(path.is_none());

        let spec = toolchain.toolchain().spec();
        assert!(spec.is_some());
    }

    #[test]
    fn parser_err() {
        let parser = Parser::new("...");
        let result = parser.parse();

        assert!(result.is_err());

        assert!(matches!(result.unwrap_err(), ParserError::TomlParse(_)))
    }

    mod toolchain_spec {
        use super::*;
        use crate::toml::{Component, Profile, Target};

        #[test]
        fn channel() {
            let value = Channel("hello".to_string());

            let spec = ToolchainSpec {
                channel: Some(value.clone()),
                components: None,
                targets: None,
                profile: None,
            };

            assert_eq!(spec.channel().unwrap(), &value);
            assert!(spec.components().is_none());
            assert!(spec.targets().is_none());
            assert!(spec.profile().is_none());
        }

        #[test]
        fn components() {
            let value = vec![
                Component("hello".to_string()),
                Component("chris".to_string()),
            ];

            let spec = ToolchainSpec {
                channel: None,
                components: Some(value.clone()),
                targets: None,
                profile: None,
            };

            assert!(spec.channel().is_none());
            assert_eq!(spec.components().unwrap(), &value);
            assert!(spec.targets().is_none());
            assert!(spec.profile().is_none());
        }

        #[test]
        fn targets() {
            let value = vec![Target("t1".to_string()), Target("t2".to_string())];

            let spec = ToolchainSpec {
                channel: None,
                components: None,
                targets: Some(value.clone()),
                profile: None,
            };

            assert!(spec.channel().is_none());
            assert!(spec.components().is_none());
            assert_eq!(spec.targets().unwrap(), &value);
            assert!(spec.profile().is_none());
        }

        #[test]
        fn profile() {
            let value = Profile("Name".to_string());

            let spec = ToolchainSpec {
                channel: None,
                components: None,
                targets: None,
                profile: Some(value.clone()),
            };

            assert!(spec.channel().is_none());
            assert!(spec.components().is_none());
            assert!(spec.targets().is_none());
            assert_eq!(spec.profile().unwrap(), &value);
        }

        #[test]
        fn values() {
            let channel_value = Channel("channel".to_string());
            let component_value = vec![Component("c1".to_string()), Component("c2".to_string())];
            let target_value = vec![Target("t1".to_string()), Target("t2".to_string())];
            let profile_value = Profile("Name".to_string());

            let spec = ToolchainSpec {
                channel: Some(channel_value.clone()),
                components: Some(component_value.clone()),
                targets: Some(target_value.clone()),
                profile: Some(profile_value.clone()),
            };

            assert_eq!(spec.channel.as_ref(), Some(&channel_value));
            assert_eq!(spec.channel(), Some(&channel_value));
            assert_eq!(spec.components.as_ref(), Some(&component_value));
            assert_eq!(spec.components(), Some(component_value.as_slice()));
            assert_eq!(spec.targets.as_ref(), Some(&target_value));
            assert_eq!(spec.targets(), Some(target_value.as_slice()));
            assert_eq!(spec.profile.as_ref(), Some(&profile_value));
            assert_eq!(spec.profile(), Some(&profile_value));
        }

        #[test]
        fn nones() {
            let spec = ToolchainSpec {
                channel: None,
                components: None,
                targets: None,
                profile: None,
            };

            assert!(spec.channel().is_none());
            assert!(spec.components().is_none());
            assert!(spec.targets().is_none());
            assert!(spec.profile().is_none());
        }
    }
}

mod toolchain_path {
    use crate::toml::ToolchainPath;
    use camino::Utf8Path;

    #[test]
    fn path() {
        let value = ToolchainPath {
            path: Utf8Path::new("/my/path").to_path_buf(),
        };

        assert_eq!(value.path(), Utf8Path::new("/my/path"));
        assert_eq!(value.path(), &value.path)
    }
}

mod channel {
    use crate::toml::Channel;

    #[test]
    fn name() {
        let name = "msvc";
        let value = Channel(name.to_string());

        assert_eq!(value.name(), name);
        assert_eq!(value.name(), &value.0)
    }
}

mod component {
    use crate::toml::Component;

    #[test]
    fn name() {
        let name = "msvc";
        let value = Component(name.to_string());

        assert_eq!(value.name(), name);
        assert_eq!(value.name(), &value.0)
    }
}

mod target {
    use crate::toml::Target;

    #[test]
    fn name() {
        let name = "msvc";
        let value = Target(name.to_string());

        assert_eq!(value.name(), name);
        assert_eq!(value.name(), &value.0)
    }
}

mod profile {
    use crate::toml::Profile;

    #[test]
    fn name() {
        let name = "empty";
        let profile = Profile(name.to_string());

        assert_eq!(profile.name(), name);
        assert_eq!(profile.name(), &profile.0)
    }
}

mod parser {
    use super::Parser;

    #[test]
    fn new() {
        let content = "test content";
        let parser = Parser::new(content);

        let expected = b"test content";
        assert_eq!(parser.content, expected);
    }

    #[test]
    fn new_and_from_slice_are_alike() {
        let content = "test content";
        let parser_new = Parser::new(content);

        let parser_from_slice = Parser::from_slice(content.as_bytes());

        let expected = b"test content";
        assert_eq!(parser_new.content, expected);
        assert_eq!(parser_from_slice.content, expected);
    }
}
