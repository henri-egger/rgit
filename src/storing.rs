pub trait Storable {
    fn store(&self);
}

pub trait Object {
    fn new_from_object_file(sha: &str, name: Option<String>) -> Self;

    fn serialize(&self) -> Vec<u8>;
}
