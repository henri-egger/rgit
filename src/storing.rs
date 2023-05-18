use serde::{Deserialize, Serialize};

pub trait Storable {
    fn store(&self);
}

pub trait Object {
    fn new_from_object_file(sha: &str, name: Option<String>) -> Self;

    fn serialize(&self) -> Vec<u8>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Finalized<T>
where
    T: Storable,
{
    value: T,
}

impl<T> Finalized<T>
where
    T: Storable,
{
    pub fn new(value: T) -> Finalized<T> {
        Finalized { value }
    }
}

impl<T> Storable for Finalized<T>
where
    T: Storable,
{
    fn store(&self) {
        self.value.store()
    }
}
