use anyhow::Result;
use cogwheel::{config::ConfigurationVariant, Configuration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Configuration, Serialize, Deserialize)]
/// Very barebones config struct.
struct SomeBasicConfig {
    some_string: String,
    some_bool: bool,
    some_nest: SomeBasicNestedConfig,
}

#[derive(Debug, Default, Configuration, Serialize, Deserialize)]
/// A very barebones nested config struct.
struct SomeBasicNestedConfig {
    some_int: i32,
    some_float: f32,
    some_unsigned: u32,
}

#[test]
#[cfg(feature = "toml")]
fn deserialize_configuration_toml() -> Result<()> {
    let file: &str = r#"
    some_string = "Hello, world!"
    some_bool = true

    [some_nest]
    some_int = -4
    some_float = 3.14159265
    some_unsigned = 2147483648
    "#;

    let config: SomeBasicConfig = SomeBasicConfig::builder()
        .use_str(file, ConfigurationVariant::Toml)?
        .build()?;

    assert_eq!(config.some_string, "Hello, world!");
    assert!(config.some_bool);
    assert_eq!(config.some_nest.some_int, -4_i32);
    assert_eq!(config.some_nest.some_float, 3.14_159_265);
    assert_eq!(config.some_nest.some_unsigned, 2_147_483_648_u32);

    Ok(())
}

#[test]
#[cfg(feature = "json")]
fn deserialize_configuration_json() -> Result<()> {
    let file: &str = r#"
    {
        "some_string": "Hello, world!",
        "some_bool": true,
        "some_nest": {
            "some_int": -4,
            "some_float": 3.14159265,
            "some_unsigned": 2147483648
        }
    }
    "#;

    let config: SomeBasicConfig = SomeBasicConfig::builder()
        .use_str(file, ConfigurationVariant::Json)?
        .build()?;

    assert_eq!(config.some_string, "Hello, world!");
    assert!(config.some_bool);
    assert_eq!(config.some_nest.some_int, -4_i32);
    assert_eq!(config.some_nest.some_float, 3.14_159_265);
    assert_eq!(config.some_nest.some_unsigned, 2_147_483_648_u32);

    Ok(())
}

#[test]
#[cfg(feature = "yaml")]
fn deserialize_configuration_yaml() -> Result<()> {
    let file: &str = r#"
    some_string: "Hello, world!"
    some_bool: true

    some_nest:
        some_int: -4
        some_float: 3.14159265
        some_unsigned: 2147483648
    "#;

    let config: SomeBasicConfig = SomeBasicConfig::builder()
        .use_str(file, ConfigurationVariant::Yaml)?
        .build()?;

    assert_eq!(config.some_string, "Hello, world!");
    assert!(config.some_bool);
    assert_eq!(config.some_nest.some_int, -4_i32);
    assert_eq!(config.some_nest.some_float, 3.14_159_265);
    assert_eq!(config.some_nest.some_unsigned, 2_147_483_648_u32);

    Ok(())
}
