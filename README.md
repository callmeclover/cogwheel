# Cogwheel

*A customizable and unopinionated configuration library.*

If you have any questions, make an issue or e-mail me!
I may be busy, so I may be slow to respond.

Please make feature requests only in issues.

## Example

```rust
//! ./examples/deserialize.rs
//! 
//! Barebones example on deserializing from a file with Cogwheel and using the result.
//!
//! Note that Rust starts rounding after 8 decimal points in `f32`s
//! (`3.14159265_f32` becomes `3.1415927_f32`, it's cursed, I know,)
//! so modifying beyond that becomes messy.
//! A "workaround" for this is using higher bits,
//! e.g. an `f64` or `f128` for really precise values.

use anyhow::Result;
use cogwheel::{config::ConfigurationVariant, Configuration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Configuration, Serialize, Deserialize, PartialEq)]
/// Very barebones config struct.
pub struct SomeBasicConfig {
    pub some_string: String,
    pub some_bool: bool,
    pub some_nest: SomeBasicNestedConfig,
}

#[derive(Debug, Default, Configuration, Serialize, Deserialize, PartialEq)]
/// A very barebones nested config struct.
pub struct SomeBasicNestedConfig {
    pub some_int: i32,
    pub some_float: f32,
    pub some_unsigned: u32,
}

fn main() -> Result<()> {
    let json_config: SomeBasicConfig = SomeBasicConfig::builder()
        .use_file(
            "./examples/somebasicconfig.json",
            ConfigurationVariant::Json,
        )?
        .build()?;

    let toml_config: SomeBasicConfig = SomeBasicConfig::builder()
        .use_file(
            "./examples/somebasicconfig.toml",
            ConfigurationVariant::Toml,
        )?
        .build()?;

    let yaml_config: SomeBasicConfig = SomeBasicConfig::builder()
        .use_file(
            "./examples/somebasicconfig.yaml",
            ConfigurationVariant::Yaml,
        )?
        .build()?;

    println!("JSON:\n{json_config:#?}");
    println!("TOML:\n{toml_config:#?}");
    println!("YAML:\n{yaml_config:#?}");

    // Should output "They are equal!"
    println!(
        "They {} equal!",
        if json_config == toml_config && json_config == yaml_config {
            "are"
        } else {
            "aren't"
        }
    );

    Ok(())
}
```
