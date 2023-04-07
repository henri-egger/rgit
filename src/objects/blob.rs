use sha1_smol;
use std::{fs, io, path};

pub struct Blob {
    bytes: Vec<u8>,
    sha1: String,
}

impl Blob {
    fn new(path: impl AsRef<path::Path>) -> Result<Blob, io::Error> {
        let bytes = fs::read(path)?;

        let mut hasher = sha1_smol::Sha1::new();
        hasher.update(&bytes[..]);

        let sha1 = hasher.digest().to_string();

        Ok(Blob { bytes, sha1 })
    }
}
