use crate::Paths;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs, io};

#[derive(Serialize, Deserialize)]
pub struct Index {
    entry_count: u32,
    entries: Vec<Entry>,
    sha1: String,
}

impl Index {
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    fn write_to_file(&self) -> Result<(), io::Error> {
        fs::write(Paths::index(), self.to_json_string()?)
    }
}

#[derive(Serialize, Deserialize)]
struct Entry {
    ctime: u64,
    mtime: u64,
    dev: u32,
    ino: u32,
    mode: u32,
    uid: u32,
    gid: u32,
    file_size: u32,
    sha1: String,
    file_path: String,
}
