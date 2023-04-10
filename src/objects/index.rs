use crate::{
    objects,
    storing::{Storable, Stored},
    Paths,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fmt, fs, os::unix::prelude::PermissionsExt, path};

#[derive(Serialize, Deserialize)]
pub struct Index {
    entry_count: usize,
    entries: Vec<Stored<Entry>>,
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

        let new_entry = match self
            .entries
            .iter_mut()
            .enumerate()
            .find(|(_, x)| x.value().path.eq(&path.to_string()))
        {
            // If an entry with this path already exists in the index and the sha1s and modes match
            // then return, else update it with the new data
            Some((i, _)) => {
                let existing_entry = self.entries.remove(i);
                let mut existing_entry = existing_entry.into_value();

                if existing_entry.sha1.eq(&new_entry.sha1())
                    && existing_entry.mode == new_entry.mode
                {
                    return;
                }

                existing_entry.update(path);

                existing_entry
            }
            None => new_entry,
        };

        let stored_new_entry = Stored::new(new_entry);

        self.entries.push(stored_new_entry);
        self.entry_count = self.entries.len();

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
    }

    fn sha1(&self) -> &str {
        &self.sha1
    }
}

impl Storable for Entry {
    fn store(&self) {
        objects::Blob::from_wd_file(&self.path).store();
    }
}
