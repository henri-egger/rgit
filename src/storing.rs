use crate::Paths;
use serde::{Deserialize, Serialize};
use std::fs;

pub trait Storable {
    fn store(&self);
}

pub trait Object {
    fn new_from_object_file<'de>(sha1: &str) -> Self
    where
        Self: Deserialize<'de>,
    {
        let file = fs::File::open(Paths::objects() + "/" + sha1)
            .expect(&format!("Failed to open {} to create object", sha1));

        let object = ciborium::de::from_reader(file).expect("Failed to read data from object");

        object
    }
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
