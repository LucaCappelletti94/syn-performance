#![doc = include_str!("../README.md")]

/// A generic attribute.
pub struct Attribute {
    /// The name of the attribute.
    pub name: String,
    /// The type of the attribute.
    pub r#type: String,
    /// Whether the attribute should be wrapped in an `Option`.
    pub optional: bool,
}

/// A generic struct.
pub struct Struct {
    /// The name of the struct.
    pub name: String,
    /// The attributes of the struct.
    pub attributes: Vec<Attribute>,
}

impl Struct {
    /// Returns the number of attributes in this struct.
    pub fn number_of_attributes(&self) -> usize {
        self.attributes.len()
    }

    /// Returns the number of optional fields in this struct.
    pub fn number_of_optional_fields(&self) -> usize {
        self.attributes.iter().filter(|a| a.optional).count()
    }
}

/// Trait to measure the performance of syn parsing
pub trait Foo {
    /// Returns the number of attributes in the struct
    fn number_of_attributes() -> usize;
    /// Returns the number of fields in the struct
    fn number_of_optional_fields() -> usize;
}
