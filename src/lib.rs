//! *A customizable and unopinionated configuration library.*

pub mod config;
mod sparse;

use std::io;

#[allow(clippy::wildcard_imports)]
pub use cogwheel_macro::*;
pub use config::Configuration;
pub use sparse::Sparse;

#[derive(Debug, thiserror::Error)]
/// The representation of an error from Cogwheel.
pub enum Error {
    #[error("unknown or missing configuration variant. are you missing a feature?")]
    UnknownConfigurationVariant,
    #[error("file does not exist, use `make_with` or `make_default`")]
    FileNotExists,
    #[error("argument was meant to be a file, instead found directory")]
    FileIsDirectory,
    #[error("could not guess configuration variant, maybe specify it or check the file extension")]
    CouldNotGuess,
    #[error("no configuration location specified, use something like `use_*`, or `make_*`")]
    NoConfigurationSpecified,

    #[error("error while reading or writing file")]
    FileError(#[from] io::Error),

    #[cfg(feature = "json")]
    #[error("error while de/serializing JSON")]
    JsonError(#[from] serde_json::Error),

    #[cfg(feature = "toml")]
    #[error("error while deserializing TOML")]
    TomlDeError(#[from] toml::de::Error),
    #[cfg(feature = "toml")]
    #[error("error while serializing TOML")]
    TomlSerError(#[from] toml::ser::Error),

    #[cfg(feature = "yaml")]
    #[error("error while de/serializing YAML")]
    YamlError(#[from] serde_yml::Error),
}
