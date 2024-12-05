use cogwheel::{config::ConfigurationVariant, with_sparse, Configuration};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[with_sparse]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Configuration)]
/// Very barebones config struct.
struct SomeBasicConfig {
    some_string: String,
    some_bool: bool,
    some_nest: SomeBasicNestedConfig,
}

#[derive(Debug, Default, Configuration, Serialize, Deserialize, Clone)]
/// A very barebones nested config struct.
struct SomeBasicNestedConfig {
    some_int: i32,
    some_float: f32,
    some_unsigned: u32,
}

#[test]
fn using_sparse() -> Result<()> {
    let file: &str = r#"
    some_string = "Hello, world!"
    some_bool = true

    [some_nest]
    some_int = -4
    some_float = 3.14159265
    some_unsigned = 2147483648
    "#;

    let file_replacement: &str = r#"some_string = "Goodbye, world!""#;

    let config: SomeBasicConfig = SomeBasicConfig::builder()
        .use_str(file, ConfigurationVariant::Toml)?
        .replace::<SomeBasicConfigSparse>(file_replacement, vec!["some_string".to_string()], ConfigurationVariant::Toml)?
        .build()?;

    assert_eq!(config.some_string, "Hello, world!");
    assert!(config.some_bool);
    assert_eq!(config.some_nest.some_int, -4_i32);
    assert_eq!(config.some_nest.some_float, 3.14_159_265);
    assert_eq!(config.some_nest.some_unsigned, 2_147_483_648_u32);
    
    Ok(())
}