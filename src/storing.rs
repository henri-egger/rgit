use crate::Paths;
use serde::Deserialize;
use std::fs;

pub trait Storable {
    fn store(&self);
}

pub trait Object {
    fn new_from_object_file<'de>(sha1: String) -> Self
    where
        Self: Deserialize<'de>,
    {
        let file = fs::File::open(Paths::objects() + "/" + &sha1)
            .expect(&format!("Failed to open {} to create object", sha1));

        let object = ciborium::de::from_reader(file).expect("Failed to read data from object");

        object
    }
}
