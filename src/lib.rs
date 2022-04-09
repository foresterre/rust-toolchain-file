//! Modelled after docs published at: <https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file>

// exports
pub use {legacy::LegacyToolchainFile, toml::RustToolchainToml};

pub mod legacy;
pub mod toml;

/// Model of a Rust toolchain file, which can be used to pin a specific toolchain to a Rust project.
pub enum ToolchainFile {
    /// The legacy variant of the toolchain file only specifies the name of a toolchain
    Legacy(LegacyToolchainFile),
    /// A specification of a toolchain file, which builds on top of the TOML format.
    Toml(RustToolchainToml),
}

#[derive(Debug, Copy, Clone)]
pub enum Variant {
    Legacy,
    Toml,
}

impl Variant {
    fn parse_with(&self, content: &str) -> Result<ToolchainFile, ParserError> {
        match *self {
            Self::Legacy => legacy::Parser::new(content)
                .parse()
                .map(ToolchainFile::Legacy)
                .map_err(From::from),
            Self::Toml => toml::Parser::new(content)
                .parse()
                .map(ToolchainFile::Toml)
                .map_err(From::from),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ParseOption {
    Only(Variant),
    Fallback(Variant, Variant),
}

/// A combined parser for the legacy and TOML toolchain file formats.
pub struct Parser<'content> {
    content: &'content str,

    parse_option: ParseOption,
}

impl<'content> Parser<'content> {
    pub fn new(content: &'content str, parse_option: ParseOption) -> Self {
        Self {
            content,
            parse_option,
        }
    }
}

impl Parser<'_> {
    pub fn parse(&self) -> Result<ToolchainFile, ParserError> {
        match self.parse_option {
            ParseOption::Only(v) => v.parse_with(self.content),
            ParseOption::Fallback(lhs, rhs) => {
                lhs.parse_with(self.content).or_else(|original_err| {
                    rhs.parse_with(self.content).map_err(|fallback_err| {
                        ParserError::FallbackError(Box::new(original_err), Box::new(fallback_err))
                    })
                })
            }
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ParserError {
    #[error("Failed to parse legacy toolchain-file variant: {0}")]
    LegacyParseError(#[from] legacy::ParserError),

    #[error("Failed to parse TOML toolchain-file variant: {0}")]
    TomlParseError(#[from] toml::ParserError),

    #[error("Both original and fallback parse attempts failed. Original error: {0}. Fallback error: {1}")]
    FallbackError(Box<ParserError>, Box<ParserError>),
}
