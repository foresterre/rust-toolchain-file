#[cfg(test)]
mod tests;

use camino::{Utf8Path, Utf8PathBuf};

/// A parser for the TOML based toolchain file format.
pub struct Parser<'content> {
    content: &'content [u8],
}

impl<'content> Parser<'content> {
    /// Initialize a parser for the `&str` content.
    pub fn new(content: &'content str) -> Self {
        Self {
            content: content.as_bytes(),
        }
    }

    /// Initialize a parser.
    pub fn from_slice(content: &'content [u8]) -> Self {
        Self { content }
    }
}

impl Parser<'_> {
    pub fn parse(&self) -> Result<RustToolchainToml, ParserError> {
        toml_edit::de::from_slice(self.content).map_err(ParserError::TomlParse)
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ParserError {
    #[error("Unable to parse toolchain file: {0}")]
    TomlParse(toml_edit::de::Error),
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RustToolchainToml {
    toolchain: ToolchainSection,
}

impl RustToolchainToml {
    pub fn toolchain(&self) -> &ToolchainSection {
        &self.toolchain
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ToolchainSection {
    Path(ToolchainPath),
    Spec(ToolchainSpec),
}

impl ToolchainSection {
    pub fn path(&self) -> Option<&ToolchainPath> {
        match self {
            Self::Path(p) => Some(p),
            _ => None,
        }
    }

    pub fn spec(&self) -> Option<&ToolchainSpec> {
        match self {
            Self::Spec(ts) => Some(ts),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToolchainSpec {
    channel: Option<Channel>,
    components: Option<Vec<Component>>,
    targets: Option<Vec<Target>>,
    profile: Option<Profile>,
}

impl ToolchainSpec {
    pub fn channel(&self) -> Option<&Channel> {
        self.channel.as_ref()
    }

    pub fn components(&self) -> Option<&[Component]> {
        self.components.as_deref()
    }

    pub fn targets(&self) -> Option<&[Target]> {
        self.targets.as_deref()
    }

    pub fn profile(&self) -> Option<&Profile> {
        self.profile.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToolchainPath {
    path: Utf8PathBuf,
}

impl ToolchainPath {
    pub fn path(&self) -> &Utf8Path {
        &self.path
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Channel(String);

impl Channel {
    pub fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Component(String);

impl Component {
    pub fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target(String);

impl Target {
    pub fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Profile(String);

impl Profile {
    pub fn name(&self) -> &str {
        &self.0
    }
}
