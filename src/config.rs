use std::{
    ffi::OsStr,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The representation of a configuration file type.
/// This can normally be guessed by functions that take paths.
pub enum ConfigurationVariant {
    #[cfg(feature = "json")]
    Json,
    #[cfg(feature = "toml")]
    Toml,
    #[cfg(feature = "yaml")]
    Yaml,
}

/// An implementable trait for configuration storage.
///
/// This should be used with it's builder `ConfigurationBuilder`:
/// ```
/// let file: &str = r#"
/// some_string = "Hello, world!"
/// some_bool = true
///
/// [some_nest]
/// some_int = -4
/// some_float = 3.14159265
/// some_unsigned = 2147483648
/// "#;
///
/// let config: SomeBasicConfig = SomeBasicConfig::builder()
///     .use_str(file, ConfigurationVariant::Toml)?
///     .build()?;
/// ```
pub trait Configuration
where
    Self: Serialize + for<'de> Deserialize<'de>,
{
    /// Creates a `ConfigurationBuilder` for this configuration.
    fn builder() -> ConfigurationBuilder<Self> {
        ConfigurationBuilder(None)
    }
}

/// A builder for a `Configuration` struct.
///
/// Either a `use_*` or `make_*` method must be called and succeed before any `with_*` method.
/// If `self.0` is `None`, then `build` or any `with_*` will fail.
/// Error handling is a must.
pub struct ConfigurationBuilder<T: Serialize + for<'de> Deserialize<'de>>(Option<T>);

impl<T: Serialize + for<'de> Deserialize<'de> + Configuration> ConfigurationBuilder<T> {
    /// Attempts to build a `Configuration` from this builder.
    ///
    /// ```
    /// let file: &str = r#"
    /// some_string = "Hello, world!"
    /// some_bool = true
    ///
    /// [some_nest]
    /// some_int = -4
    /// some_float = 3.14159265
    /// some_unsigned = 2147483648
    /// "#;
    ///
    /// let config: SomeBasicConfig = SomeBasicConfig::builder()
    ///     .use_str(file, ConfigurationVariant::Toml)?
    ///     .build()?;
    /// ```
    ///
    /// # Errors
    /// This will fail if:
    /// - A location (like `use_str`) hasn't been specified yet
    pub fn build(self) -> Result<T, Error> {
        self.0
            .map_or_else(|| Err(Error::NoConfigurationSpecified), |data: T| Ok(data))
    }

    /// Attempts to parse an `&str` `data` into a configuration struct, `T`.
    ///
    /// ```
    /// let file: &str = r#"
    /// some_string = "Hello, world!"
    /// some_bool = true
    ///
    /// [some_nest]
    /// some_int = -4
    /// some_float = 3.14159265
    /// some_unsigned = 2147483648
    /// "#;
    ///
    /// let config: SomeBasicConfig = SomeBasicConfig::builder()
    ///     .use_str(file, ConfigurationVariant::Toml)?
    ///     .build()?;
    /// ```
    ///
    /// # Errors
    /// This will fail if the string is:
    /// - Missing an entry that isn't marked with an `Option<_>`
    /// - Malformed (either of wrong variant or otherwise malformed/corrupt)
    pub fn use_str(mut self, data: &str, variant: ConfigurationVariant) -> Result<Self, Error> {
        self.0 = Some(Self::gen_from_str(data, variant)?);

        Ok(self)
    }

    /// Parses `&str` into `T`.
    fn gen_from_str(data: &str, variant: ConfigurationVariant) -> Result<T, Error> {
        match variant {
            #[cfg(feature = "json")]
            ConfigurationVariant::Json => Ok(serde_json::from_str::<T>(data)?),
            #[cfg(feature = "toml")]
            ConfigurationVariant::Toml => Ok(toml::from_str::<T>(data)?),
            #[cfg(feature = "yaml")]
            ConfigurationVariant::Yaml => Ok(serde_yml::from_str::<T>(data)?),
        }
    }

    /// Converts `T` into `String`.
    fn gen_to_string(data: &T, variant: ConfigurationVariant) -> Result<String, Error> {
        match variant {
            #[cfg(feature = "json")]
            ConfigurationVariant::Json => Ok(serde_json::to_string_pretty(&data)?),
            #[cfg(feature = "toml")]
            ConfigurationVariant::Toml => Ok(toml::to_string_pretty(&data)?),
            #[cfg(feature = "yaml")]
            ConfigurationVariant::Yaml => Ok(serde_yml::to_string(&data)?),
        }
    }

    /// Attempts to read a file at `path` to type `T`.
    /// This method can guess the variant based off the path if you specify `variant` as `None`.
    ///
    /// ```
    /// let config: SomeBasicConfig = SomeBasicConfig::builder()
    ///     .use_file("./config.toml", None)?
    ///     .build()?;
    /// ```
    ///
    /// # Errors
    /// This will fail if:
    /// - The file does not exist/is a directory
    /// - The file can't be read
    pub fn use_file<S: AsRef<Path> + ?Sized>(
        mut self,
        path: &S,
        variant: ConfigurationVariant,
    ) -> Result<Self, Error> {
        let mut output: String = String::new();
        File::open(path)?.read_to_string(&mut output)?;

        self.0 = Some(Self::gen_from_str(&output, variant)?);
        Ok(self)
    }

    fn guess_file_variant(path: &Path) -> Result<ConfigurationVariant, Error> {
        match path
            .extension()
            .map(|x: &OsStr| x.to_string_lossy().to_lowercase())
            .as_deref()
        {
            #[cfg(feature = "json")]
            Some("json") => Ok(ConfigurationVariant::Json),
            #[cfg(feature = "toml")]
            Some("toml") => Ok(ConfigurationVariant::Toml),
            #[cfg(feature = "yaml")]
            Some("yaml" | "yml") => Ok(ConfigurationVariant::Yaml),
            None | Some(_) => Err(Error::FileIsDirectory),
        }
    }

    /// Attempts to make a configuration file of type `variant` at `path` with the specified data from `data`.
    /// If you want to overwrite an already existing file, you should use `make_override` instead.
    /// This method can guess the variant based off the path if you specify `variant` as `None`.
    ///
    /// ```
    /// let data: SomeBasicConfig = SomeBasicConfig {
    ///     some_string: "Hello, world!".to_string(),
    ///     some_bool: true,
    ///     some_nest: SomeBasicNestedConfig {
    ///         some_int: -4,
    ///         some_float: 3.14_159_26,
    ///         some_unsigned: 2_147_483_648,
    ///     },
    /// }
    ///
    /// let config: SomeBasicConfig = SomeBasicConfig::builder()
    ///     .make("./config.toml", data, ConfigurationVariant::Toml)?
    ///     .build()?;
    /// ```
    ///
    /// # Errors
    /// This will fail if:
    /// - The file already exists
    /// - `path` is a directory
    /// - The data was corrupt/malformed after the write
    pub fn make<S: AsRef<Path> + ?Sized>(
        mut self,
        path: &S,
        data: &T,
        variant: Option<ConfigurationVariant>,
    ) -> Result<Self, Error> {
        let variant: ConfigurationVariant = variant
            .or_else(|| Self::guess_file_variant(path.as_ref()).ok())
            .ok_or(Error::CouldNotGuess)?;
        let mut file: File = File::create_new(path)?;
        file.write_all(Self::gen_to_string(data, variant)?.as_bytes())?;
        self = self.use_file(path, variant)?;
        Ok(self)
    }

    /// Attempts to make a configuration file of type `variant` at `path` with the default data for `T`.
    /// If you want to overwrite an already existing file, you should use `make_default_override` instead.
    /// This method can guess the variant based off the path if you specify `variant` as `None`.
    ///
    /// ```
    /// let config: SomeBasicConfig = SomeBasicConfig::builder()
    ///     .make_default("./config.toml", None)?
    ///     .build()?;
    /// ```
    ///
    /// # Errors
    /// This will fail if:
    /// - The file already exists
    /// - `path` is a directory
    /// - The data was corrupt/malformed after the write
    pub fn make_default<S: AsRef<Path> + ?Sized>(
        mut self,
        path: &S,
        variant: Option<ConfigurationVariant>,
    ) -> Result<Self, Error>
    where
        T: Default,
    {
        let variant: ConfigurationVariant = variant
            .or_else(|| Self::guess_file_variant(path.as_ref()).ok())
            .ok_or(Error::CouldNotGuess)?;
        let mut file: File = File::create_new(path)?;
        file.write_all(Self::gen_to_string(&T::default(), variant)?.as_bytes())?;
        self = self.use_file(path, variant)?;
        Ok(self)
    }

    /// Attempts to make a configuration file of type `variant` at `path` with the specified data from `data`.
    /// If you want to overwrite an already existing file, you should use `make_override` instead.
    /// This method can guess the variant based off the path if you specify `variant` as `None`.
    ///
    /// ```
    /// let data: SomeBasicConfig = SomeBasicConfig {
    ///     some_string: "Hello, world!".to_string(),
    ///     some_bool: true,
    ///     some_nest: SomeBasicNestedConfig {
    ///         some_int: -4,
    ///         some_float: 3.14_159_26,
    ///         some_unsigned: 2_147_483_648,
    ///     },
    /// }
    ///
    /// let config: SomeBasicConfig = SomeBasicConfig::builder()
    ///     .make_override("./config.toml", data, ConfigurationVariant::Toml)?
    ///     .build()?;
    /// ```
    ///
    /// # Errors
    /// This will fail if:
    /// - `path` is a directory
    /// - The data was corrupt/malformed after the write
    pub fn make_override<S: AsRef<Path> + ?Sized>(
        mut self,
        path: &S,
        data: &T,
        variant: Option<ConfigurationVariant>,
    ) -> Result<Self, Error> {
        let variant: ConfigurationVariant = variant
            .or_else(|| Self::guess_file_variant(path.as_ref()).ok())
            .ok_or(Error::CouldNotGuess)?;
        let mut file: File = File::create(path)?;
        file.write_all(Self::gen_to_string(data, variant)?.as_bytes())?;
        self = self.use_file(path, variant)?;
        Ok(self)
    }

    /// Attempts to make a configuration file of type `variant` at `path` with the default data for `T`.
    /// If you want to overwrite an already existing file, you should use `make_default_override` instead.
    /// This method can guess the variant based off the path if you specify `variant` as `None`.
    ///
    /// ```
    /// let config: SomeBasicConfig = SomeBasicConfig::builder()
    ///     .make_default_override("./config.toml", None)?
    ///     .build()?;
    /// ```
    ///
    /// # Errors
    /// This will fail if:
    /// - `path` is a directory
    /// - The data was corrupt/malformed after the write
    pub fn make_default_override<S: AsRef<Path> + ?Sized>(
        mut self,
        path: &S,
        variant: Option<ConfigurationVariant>,
    ) -> Result<Self, Error>
    where
        T: Default,
    {
        let variant: ConfigurationVariant = variant
            .or_else(|| Self::guess_file_variant(path.as_ref()).ok())
            .ok_or(Error::CouldNotGuess)?;
        let mut file: File = File::create(path)?;
        file.write_all(Self::gen_to_string(&T::default(), variant)?.as_bytes())?;
        self = self.use_file(path, variant)?;
        Ok(self)
    }
}
