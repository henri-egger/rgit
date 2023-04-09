use crate::{objects, Paths};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fmt, fs, os::unix::prelude::PermissionsExt, path};

#[derive(Serialize, Deserialize)]
pub struct Index {
    entry_count: usize,
    entries: Vec<Entry>,
}

impl Index {
    fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_json_string(string: String) -> Index {
        serde_json::from_str(&string).unwrap()
    }

    fn update_index_file(&self) {
        fs::write(Paths::index(), self.to_json_string()).expect("Failed to update index file")
    }

    pub fn add_entry(&mut self, path: impl AsRef<path::Path> + fmt::Display) {
        let new_entry = Entry::new(&path);

        match self
            .entries
            .iter_mut()
            .find(|x| x.path.eq(&path.to_string()))
        {
            // If an entry with this path already exists in the index and the sha1s and modes match
            // then return, else update it with the new data
            Some(existng_entry) => {
                if existng_entry.sha1.eq(&new_entry.sha1()) && existng_entry.mode == new_entry.mode
                {
                    return;
                }

                existng_entry.update(path);
            }
            None => {
                new_entry.store_object_file();
                self.entries.push(new_entry);
                self.entry_count = self.entries.len();
            }
        }

        // If changes have been made, update the index file
        self.update_index_file();
    }

    pub fn from_index_file() -> Index {
        let mut json_string =
            fs::read_to_string(Paths::index()).expect(&format!("Failed to read index file"));

        if json_string.is_empty() {
            Index {
                entry_count: 0,
                entries: Vec::new(),
            }
            .update_index_file();

            json_string =
                fs::read_to_string(Paths::index()).expect(&format!("Failed to read index file"));
        }

        let index = Index::from_json_string(json_string);

        index
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
struct Entry {
    mode: u32,
    path: String,
    sha1: String,
}

impl Entry {
    fn new(path: impl AsRef<path::Path> + fmt::Display) -> Entry {
        let mode = fs::File::open(&path)
            .expect(&format!("Failed to open {} to retrieve metadata", path))
            .metadata()
            .expect(&format!("Failed to retrieve metadata for {}", path))
            .permissions()
            .mode();

        let sha1 = objects::Blob::from_wd_file(&path).sha1().into();

        Entry {
            mode,
            path: path.to_string(),
            sha1,
        }
    }

    fn update(&mut self, path: impl AsRef<path::Path> + fmt::Display) {
        let mode = fs::File::open(&path)
            .expect(&format!("Failed to open {} to retrieve metadata", path))
            .metadata()
            .expect(&format!("Failed to retrieve metadata for {}", path))
            .permissions()
            .mode();

        let sha1 = objects::Blob::from_wd_file(&path).sha1().into();

        self.mode = mode;
        self.sha1 = sha1;

        self.store_object_file();
    }

    fn sha1(&self) -> &str {
        &self.sha1
    }

    fn store_object_file(&self) {
        objects::Blob::from_wd_file(&self.path).store_object_file();
    }
}
