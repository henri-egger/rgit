use crate::{storing::Storable, Paths};
use ciborium;
use serde::{Deserialize, Serialize};
use sha1_smol;
use std::{fmt, fs, path};

const BLOB_IDENTIFIER: &str = "blob ";

#[derive(Serialize, Deserialize)]
pub struct Blob {
    bytes: Vec<u8>,
    sha1: String,
}

impl Blob {
    fn from_bytes(bytes: &[u8]) -> Blob {
        let mut hasher = sha1_smol::Sha1::new();
        hasher.update(&bytes);

        let sha1 = hasher.digest().to_string();

        Blob {
            bytes: bytes.into(),
            sha1,
        }
    }

    pub fn new_from_wd_file(path: impl AsRef<path::Path> + fmt::Display) -> Blob {
        let bytes = fs::read(&path).expect(&format!("Failed to read {} to create blob", path));
        let blob = Blob::from_bytes(&bytes);

        blob
    }

    pub fn new_from_object_file(sha1: String) -> Blob {
        let file = fs::File::open(Paths::objects() + "/" + &sha1)
            .expect(&format!("Failed to open {} to create blob", sha1));

        let buf: Vec<u8> = ciborium::de::from_reader(file).expect("Failed to read data from blob");

        let bytes = &buf[BLOB_IDENTIFIER.len()..];
        let blob = Blob::from_bytes(bytes);

        blob
    }

    pub fn sha1(&self) -> &str {
        &self.sha1
    }
}

impl Storable for Blob {
    fn store(&self) {
        let mut buf: Vec<u8> = BLOB_IDENTIFIER.into();
        buf.extend_from_slice(&self.bytes);

        let file = fs::File::create(Paths::objects() + "/" + &self.sha1)
            .expect("Failed to create file to store blob");

        ciborium::ser::into_writer(self, file).expect("Failed to write data to blob");
    }
}
