//! Barebones example on deserializing from a file with Cogwheel and using the result.
//!
//! Note that Rust starts rounding after 8 decimal points in `f32`s
//! (`3.14159265_f32` becomes `3.1415927_f32`, it's cursed, I know,)
//! so modifying beyond that becomes messy.
//! A "workaround" for this is using higher bits,
//! e.g. an `f64` or `f128` for really precise values.

mod common;

use anyhow::Result;
use cogwheel::{config::ConfigurationVariant, Configuration};
#[allow(clippy::wildcard_imports)]
use common::*;

fn main() -> Result<()> {
    let json_config: SomeBasicConfig = SomeBasicConfig::builder()
        .use_file(
            "./examples/common/somebasicconfig.json",
            ConfigurationVariant::Json,
        )?
        .build()?;

    let toml_config: SomeBasicConfig = SomeBasicConfig::builder()
        .use_file(
            "./examples/common/somebasicconfig.toml",
            ConfigurationVariant::Toml,
        )?
        .build()?;

    let yaml_config: SomeBasicConfig = SomeBasicConfig::builder()
        .use_file(
            "./examples/common/somebasicconfig.yaml",
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
