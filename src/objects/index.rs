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

    fn query_by_path<T>(&self, path: T) -> Option<usize>
    where
        T: AsRef<path::Path> + fmt::Display,
    {
        let entry = self
            .entries
            .iter()
            .enumerate()
            .find(|(_, x)| x.value().path.eq(&path.to_string()));

        match entry {
            Some((i, _)) => return Some(i),
            None => None,
        }
    }

    pub fn add(&mut self, path: impl AsRef<path::Path> + fmt::Display) {
        let new_maybe_entry = Entry::try_new(&path);
        let existing_maybe_i = self.query_by_path(&path);

        match new_maybe_entry {
            None => match existing_maybe_i {
                None => panic!("File {} not found", path),
                Some(existing_i) => {
                    self.entries.remove(existing_i);
                }
            },

            Some(new_entry) => {
                if let Some(existing_i) = existing_maybe_i {
                    self.entries.remove(existing_i);
                }

                self.entries.push(Stored::new(new_entry));
            }
        }

        self.update_index_file();
    }

    pub fn from_index_file() -> Index {
        let mut json_string =
            fs::read_to_string(Paths::index()).expect(&format!("Failed to read index file"));

        if json_string.is_empty() {
            Index {
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

    fn try_new(path: impl AsRef<path::Path> + fmt::Display) -> Option<Entry> {
        if !path::Path::try_exists(path.as_ref()).unwrap() {
            return None;
        }

        Some(Entry::new(path))
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
