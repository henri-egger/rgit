use crate::{
    storing::{Object, Storable},
    Paths,
};
use ciborium;
use serde::{Deserialize, Serialize};
use sha1_smol;
use std::{fmt, fs, path};

#[derive(Serialize, Deserialize)]
pub struct Blob {
    bytes: Vec<u8>,
}

impl Blob {
    fn new_from_bytes(bytes: Vec<u8>) -> Blob {
        Blob { bytes }
    }

    pub fn new_from_wd_file(path: impl AsRef<path::Path> + fmt::Display) -> Blob {
        let bytes = fs::read(&path).expect(&format!("Failed to read {} to create blob", path));
        let blob = Blob::new_from_bytes(bytes);

        blob
    }

    pub fn sha1(&self) -> String {
        let mut hasher = sha1_smol::Sha1::new();
        hasher.update(&self.bytes);
        let sha1 = hasher.digest().to_string();

        sha1
    }
}

impl Storable for Blob {
    fn store(&self) {
        let file = fs::File::create(Paths::objects() + "/" + &self.sha1())
            .expect("Failed to create file to store blob");

        ciborium::ser::into_writer(self, file).expect("Failed to write data to blob");
    }
}

impl Object for Blob {}
