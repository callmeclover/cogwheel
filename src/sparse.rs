use serde::{Deserialize, Serialize};

/// A trait representing a version of a struct with only optional fields.
/// You shouldn't implement this yourself, and should instead use its macro,
/// `#[with_sparse]`, which will generate a struct like `<Struct Name>Sparse`.
pub trait Sparse: Serialize + for<'de> Deserialize<'de> {}