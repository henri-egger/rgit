/// Trait for all Structs which have to do fs operations
pub trait Storable {
    fn store(&self);
}

/// Trait used for all Structs which represent actual objects in the .rgit/objects directory
pub trait Object {
    /// Retrieves the object from its serialized representation as a file in the objects directory
    fn new_from_object_file(sha: &str, name: Option<String>) -> Self;

    /// Serializes the object to bytes to be able to be stored
    fn serialize(&self) -> Vec<u8>;
}
