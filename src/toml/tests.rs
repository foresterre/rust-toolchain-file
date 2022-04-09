use crate::toml::Parser;
use crate::RustToolchainToml;
use camino::Utf8Path;

const RUSTUP_BOOK_LAYOUT: &'static str =
    include_str!("../../tests/fixtures/rustup-book-layout/rust-toolchain.toml");

#[test]
fn parse_rustup_book_layout() {
    let rust_toolchain: RustToolchainToml = toml_edit::de::from_str(RUSTUP_BOOK_LAYOUT).unwrap();

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

const RUSTUP_BOOK_LOCAL_TOOLCHAIN: &'static str =
    include_str!("../../tests/fixtures/rustup-book-local-toolchain/rust-toolchain.toml");

#[test]
fn parse_rustup_book_layout_with_parser() {
    let p = Parser::new(RUSTUP_BOOK_LAYOUT);

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

    dbg!(&rust_toolchain);

    let toolchain = rust_toolchain.toolchain();
    let path = toolchain.path();

    assert!(path.is_some());
    let path = path.unwrap().path();

    assert_eq!(path, Utf8Path::new("/path/to/local/toolchain"));
}
