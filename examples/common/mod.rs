//! Shared code between the examples.

use cogwheel::Configuration;
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
