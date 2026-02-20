pub use normalization_macros::*;

#[derive(Debug, PartialEq)]
pub enum NormalizationError {
    MissingField,
    InvalidFormat,
    ParseFailure,
}

pub trait Serializable {
    fn serialize(&self) -> String;
}

pub trait Deserializable: Sized {
    fn deserialize(input: &str) -> Result<Self, NormalizationError>;
    // fn deserialize(input: &str) -> Result<Self, &'static str>;
}
