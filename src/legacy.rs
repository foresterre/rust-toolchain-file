use std::path::{Path, PathBuf};
use std::str::FromStr;

#[cfg(test)]
mod tests;

/// A parser for the legacy toolchain file format.
pub struct Parser<'content> {
    content: &'content str,

    /// According to the rustup book, the legacy format must be encoded as US-ASCII without BOM.
    ///
    /// Setting this field to true, we'll be more lenient, and allow the input to be encoded as UTF-8.
    strict: bool,
}

impl<'content> Parser<'content> {
    /// Initialize a parser which leniently accepts the US-ASCII compatible UTF-8 encoding.
    pub fn new(content: &'content str) -> Self {
        Self {
            content,
            strict: false,
        }
    }

    /// Initialize a parser, which strictly only accepts US-ASCII encoded content.
    pub fn strict(content: &'content str) -> Self {
        Self {
            content,
            strict: true,
        }
    }
}

impl Parser<'_> {
    pub fn parse(&self) -> Result<LegacyToolchainFile, ParserError> {
        // Verify the required encoding.
        if self.strict && !self.content.is_ascii() {
            return Err(ParserError::InvalidEncodingStrict);
        }

        let content = self.content.trim();

        // Verify, that there is content
        if content.is_empty() {
            return Err(ParserError::IsEmpty);
        }

        // Verify the contents consist of one specifier or path, on a single line
        let line_count = content.lines().count();

        if line_count != 1 {
            return Err(ParserError::TooManyLines(line_count));
        }

        // Set the channel type
        let channel = if Path::new(content).is_absolute() {
            LegacyChannel::Path(content.into())
        } else {
            LegacyChannel::Spec(content.to_string())
        };

        Ok(LegacyToolchainFile { channel })
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ParserError {
    #[error("Unable to parse legacy toolchain file: toolchain file was empty")]
    IsEmpty,

    #[error("Encountered invalid encoding while parsing legacy rust-toolchain file. The expected encoding to be US-ASCII, and lenient encoding was disabled.")]
    InvalidEncodingStrict,

    #[error("Expected a single line containing the toolchain specifier but found '{0}' lines.")]
    TooManyLines(usize),
}

/// The legacy toolchain file variant
#[derive(Debug, PartialEq)]
pub struct LegacyToolchainFile {
    channel: LegacyChannel,
}

impl FromStr for LegacyToolchainFile {
    type Err = ParserError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let parser = Parser::new(content);

        parser.parse()
    }
}

impl LegacyToolchainFile {
    pub fn channel(&self) -> &LegacyChannel {
        &self.channel
    }

    /// Return the toolchain path, given that the toolchain-file contents
    /// consists of a path and not a channel specification.
    pub fn path(&self) -> Option<&Path> {
        match self.channel {
            LegacyChannel::Path(ref p) => Some(p.as_path()),
            _ => None,
        }
    }

    /// Return the channel specification specified in the toolchain file, given that the toolchain-file
    /// contents consists of a channel spec and not a path.
    pub fn spec(&self) -> Option<&str> {
        match self.channel {
            LegacyChannel::Spec(ref p) => Some(p.as_str()),
            _ => None,
        }
    }
}

/// The channel specified within the legacy toolchain file.
#[derive(Debug, PartialEq)]
pub enum LegacyChannel {
    Path(PathBuf),
    Spec(String),
}
