use crate::Paths;
use sha1_smol;
use std::{fmt, fs, path};

const BLOB_IDENTIFIER: &str = "blob ";

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

    pub fn from_wd_file(path: impl AsRef<path::Path> + fmt::Display) -> Blob {
        let bytes = fs::read(&path).expect(&format!("Failed to read {} to create blob", path));
        let blob = Blob::from_bytes(&bytes);

        blob
    }

    pub fn from_object_file(sha1: String) -> Blob {
        let buf = fs::read(Paths::objects() + &sha1)
            .expect(&format!("Failed to read {} to create blob", sha1));

        let bytes = &buf[BLOB_IDENTIFIER.len()..];
        let blob = Blob::from_bytes(bytes);

        if blob.sha1().ne(&sha1) {
            panic!("Filename sha != content sha");
        }

        blob
    }

    pub fn store_object_file(&self) {
        let mut buf: Vec<u8> = BLOB_IDENTIFIER.into();
        buf.extend_from_slice(&self.bytes);

        fs::write(Paths::objects() + "/" + &self.sha1, &buf)
            .expect(&format!("Failed to store blob"));
    }

    pub fn sha1(&self) -> &str {
        &self.sha1
    }
}
