use crate::{
    identifiers,
    storing::{Object, Storable},
    Paths,
};
use sha1_smol;
use std::{fmt, fs, io::Write, path};

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

    pub fn sha(&self) -> String {
        let mut hasher = sha1_smol::Sha1::new();
        hasher.update(&self.bytes);
        let sha = hasher.digest().to_string();

        sha
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Storable for Blob {
    fn store(&self) {
        let mut file = fs::File::create(Paths::objects() + "/" + &self.sha())
            .expect("Failed to create file to store blob");

        let buf = self.serialize();

        file.write_all(&buf).unwrap();
    }
}

impl Object for Blob {
    fn new_from_object_file(sha: &str, _: Option<String>) -> Self {
        let buf = fs::read(Paths::objects() + "/" + sha).unwrap();
        let null_i = buf.iter().position(|x| *x == b'\0').unwrap();

        let bytes = Vec::from(&buf[null_i + 1..]);

        Blob { bytes }
    }

    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.extend(identifiers::BLOB.bytes());
        buf.push(b' ');
        buf.extend(
            // self.bytes
            //     .len()
            //     .to_be_bytes()
            //     .into_iter()
            //     .filter(|x| *x != b'\0'),
            self.bytes.len().to_string().bytes(),
        );
        buf.push(b'\0');
        buf.extend_from_slice(&self.bytes);

        buf
    }
}
