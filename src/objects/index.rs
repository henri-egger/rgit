use crate::{objects, storing::Storable, IgnoreFilter, Paths};
use glob;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashSet, fmt, fs, hash::Hash, io, os::unix::prelude::PermissionsExt, path};

#[derive(Serialize, Deserialize)]
pub struct Index {
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

    fn query_by_path<T>(&self, path: T) -> Option<usize>
    where
        T: AsRef<path::Path> + fmt::Display,
    {
        let entry = self
            .entries
            .iter()
            .enumerate()
            .find(|(_, x)| x.path.eq(&path.to_string()));

        match entry {
            Some((i, _)) => return Some(i),
            None => None,
        }
    }

    // TODO: Work with glob
    pub fn add(&mut self, mut path: String) {
        let ignore_filter = IgnoreFilter::new(Paths::ignore());

        let paths: Vec<String> = if path::Path::is_dir(&path::Path::new(&path)) {
            if !path.ends_with("/") {
                path.push_str("/");
            }

            path.push_str("**/*.*");

            glob::glob(&path)
                .unwrap()
                .map(|x| x.unwrap())
                .map(|x| x.to_string_lossy().to_string())
                .filter(|x| ignore_filter.validate(x))
                .collect()
        } else {
            if !ignore_filter.validate(&path) {
                panic!("Path is included in gitignore");
            }

            vec![path]
        };

        for path in paths.iter() {
            let result = self.add_entry_from_path(path);
            if let Err(err) = result {
                eprintln!("{}", err);
            }
        }
    }

    fn add_entry_from_path<T>(&mut self, path: T) -> Result<(), io::Error>
    where
        T: AsRef<path::Path> + fmt::Display,
    {
        let new_maybe_entry = Entry::try_new(&path);
        let existing_maybe_i = self.query_by_path(&path);

        match new_maybe_entry {
            None => match existing_maybe_i {
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("File {} not found", path),
                    ))
                }
                Some(existing_i) => {
                    self.entries.remove(existing_i);
                }
            },

            Some(new_entry) => {
                if let Some(existing_i) = existing_maybe_i {
                    self.entries.remove(existing_i);
                }

                self.entries.push(new_entry);
            }
        }

        Ok(())
    }

    pub fn new_from_index_file() -> Index {
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

    pub fn status(&self) {
        let ignore_filter = IgnoreFilter::new(Paths::ignore());

        let wd_entries: HashSet<Entry> = glob::glob("**/*.*")
            .expect("Failed to read glob pattern")
            .map(|x| x.unwrap())
            .map(|x| x.to_string_lossy().to_string())
            .filter(|x| ignore_filter.validate(x))
            .map(|x| Entry::new_from_path(x))
            .collect();

        let index_entries: HashSet<Entry> = self.entries.iter().cloned().collect();

        let delta = &wd_entries - &index_entries;

        // TODO: Sort diffrences
        dbg!(&delta);
    }
}

impl Storable for Index {
    fn store(&self) {
        for entry in self.entries.iter() {
            entry.store();
        }

        self.update_index_file()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
struct Entry {
    mode: u32,
    path: String,
    sha1: String,
}

impl Entry {
    fn new_from_path(path: impl AsRef<path::Path> + fmt::Display) -> Entry {
        let mode = fs::File::open(&path)
            .expect(&format!("Failed to open {} to retrieve metadata", path))
            .metadata()
            .expect(&format!("Failed to retrieve metadata for {}", path))
            .permissions()
            .mode();

        let sha1 = objects::Blob::new_from_wd_file(&path).sha1().into();

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

        Some(Entry::new_from_path(path))
    }
}

impl Storable for Entry {
    fn store(&self) {
        objects::Blob::new_from_wd_file(&self.path).store();
    }
}
