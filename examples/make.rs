//! Barebones example on making a configuration file using the `make_*` methods.

mod common;

use anyhow::Result;
use cogwheel::Configuration;
#[allow(clippy::wildcard_imports)]
use common::*;

fn main() -> Result<()> {
    let config_struct: SomeBasicConfig = SomeBasicConfig {
        some_string: "Hello, world!".to_string(),
        some_bool: true,
        some_nest: SomeBasicNestedConfig {
            some_int: -4,
            some_float: 3.14_159_26,
            some_unsigned: 2_147_483_648,
        },
    };

    SomeBasicConfig::builder()
        .make("./examples/imadeitmyself.json", &config_struct, None)?
        .build()?;
    SomeBasicConfig::builder()
        .make_default("./examples/acomputermadethis.json", None)?
        .build()?;

    let config_struct: SomeBasicConfig = SomeBasicConfig {
        some_string: "Goodbye, world!".to_string(),
        some_bool: false,
        some_nest: SomeBasicNestedConfig {
            some_int: 4,
            some_float: 64.54,
            some_unsigned: 1337,
        },
    };

    SomeBasicConfig::builder()
        .make_override("./examples/imadeitmyself.json", &config_struct, None)?
        .build()?;
    SomeBasicConfig::builder()
        .make_default_override("./examples/acomputermadethis.json", None)?
        .build()?;

    Ok(())
}
