//! Modelled after docs published at: <https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file>

// exports
pub use {legacy::LegacyToolchainFile, toml::RustToolchainToml};

pub mod legacy;
pub mod toml;

#[cfg(test)]
mod tests;

/// Model of a Rust toolchain file, which can be used to pin a specific toolchain to a Rust project.
#[derive(Debug, PartialEq)]
pub enum ToolchainFile {
    /// The legacy variant of the toolchain file only specifies the name of a toolchain
    Legacy(LegacyToolchainFile),
    /// A specification of a toolchain file, which builds on top of the TOML format.
    Toml(RustToolchainToml),
}

/// Variants which may be used to identify supported rust-toolchain variants.
#[derive(Debug, Copy, Clone, PartialEq)]
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

/// Option to determine whether only to parse one rust-toolchain variant (TOML, or legacy), or
/// to fallback to another variant.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ParseStrategy {
    Only(Variant),
    Fallback {
        first: Variant,
        fallback_to: Variant,
    },
}

/// A combined parser for the legacy and TOML toolchain file formats.
pub struct Parser<'content> {
    content: &'content str,

    parse_option: ParseStrategy,
}

impl<'content> Parser<'content> {
    pub fn new(content: &'content str, parse_option: ParseStrategy) -> Self {
        Self {
            content,
            parse_option,
        }
    }
}

impl Parser<'_> {
    pub fn parse(&self) -> Result<ToolchainFile, ParserError> {
        match self.parse_option {
            ParseStrategy::Only(v) => v.parse_with(self.content),
            ParseStrategy::Fallback { first, fallback_to } => {
                first.parse_with(self.content).or_else(|original_err| {
                    fallback_to
                        .parse_with(self.content)
                        .map_err(|fallback_err| {
                            ParserError::FallbackError(FallbackError {
                                first: Box::new(original_err),
                                fallback_to: Box::new(fallback_err),
                            })
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

    #[error("Both original and fallback parse attempts failed: {0}")]
    FallbackError(FallbackError),
}

#[derive(Debug, PartialEq, thiserror::Error)]
#[error("Failed to parse: '{first}' and failed to fallback on '{fallback_to}'")]
pub struct FallbackError {
    first: Box<ParserError>,
    fallback_to: Box<ParserError>,
}

impl FallbackError {
    pub fn first(&self) -> &ParserError {
        self.first.as_ref()
    }

    pub fn fallback_to(&self) -> &ParserError {
        self.fallback_to.as_ref()
    }
}
