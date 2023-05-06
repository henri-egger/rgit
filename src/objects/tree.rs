use crate::{
    objects::{index, Index},
    storing::{Object, Storable},
    Paths,
};
use ciborium;
use serde::{Deserialize, Serialize};
use sha1_smol::Sha1;
use std::{fs, path};

#[derive(Serialize, Deserialize, Debug, Clone)]
enum EntryType {
    Tree(Tree),
    Blob(Entry),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tree {
    entries: Vec<EntryType>,
    name: String,
    sha1: Option<String>,
}

impl Tree {
    pub fn new(name: &str, entries: Vec<index::Entry>) -> Tree {
        let mut tree = Tree {
            entries: Vec::new(),
            name: String::from(name),
            sha1: None,
        };

        tree.add_index_entries(entries);
        tree.generate_shas();

        tree
    }

    fn add_index_entries(&mut self, mut entries: Vec<index::Entry>) {
        // To avoid cloning and having multiple instances of entries which
        // have to be syncronized, the indicies of the affected items are collected
        // and then reversed, so that they can then be removed from the original entries
        // array. This might be simpler if entries was just a hashset but it is what it is rn :)

        entries
            .iter()
            .enumerate()
            .filter(|(_, i_entry)| i_entry.is_top_level())
            .rev()
            .fold(&mut Vec::new(), |vec, (i, _)| {
                vec.push(i);
                vec
            })
            .iter()
            .map(|i| entries.remove(*i))
            .map(|i_entry| Entry::from(i_entry))
            .map(|t_entry| EntryType::Blob(t_entry))
            .for_each(|blob| self.entries.push(blob));

        while entries.len() > 0 {
            let path = entries[0].path().to_owned();
            let dir = &path[0..path.find("/").unwrap()];

            let entries_with_dir = entries
                .iter()
                .enumerate()
                .filter(|(_, i_entry)| i_entry.path().starts_with(dir))
                .rev()
                .fold(&mut Vec::new(), |vec, (i, _)| {
                    vec.push(i);
                    vec
                })
                .iter()
                .map(|i| entries.remove(*i))
                .map(|mut i_entry| {
                    let slash_offset = i_entry.path().find("/").unwrap();
                    i_entry.path_mut().replace_range(..=slash_offset, "");
                    i_entry
                })
                .collect();

            let tree = Tree::new(dir, entries_with_dir);
            self.entries.push(EntryType::Tree(tree));
        }
    }

    fn generate_sha1(&self) -> String {
        let mut bytes = Vec::new();
        ciborium::ser::into_writer(self, &mut bytes)
            .expect("Failed to serialize tree to generate hash");

        let mut hasher = Sha1::new();
        hasher.update(&bytes);
        let sha1 = hasher.digest().to_string();

        sha1
    }

    fn generate_shas(&mut self) {
        self.get_trees_mut()
            .for_each(|(_, tree)| tree.generate_shas());

        self.sha1 = Some(self.generate_sha1());
    }

    fn get_trees(&self) -> impl Iterator<Item = (usize, &Tree)> {
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, entry_type)| match entry_type {
                EntryType::Tree(_) => true,
                EntryType::Blob(_) => false,
            })
            .map(|(i, entry_type)| match entry_type {
                EntryType::Tree(tree) => (i, tree),
                EntryType::Blob(_) => panic!("Blob after tree filtering"),
            })
            .rev()
    }

    fn get_trees_mut(&mut self) -> impl Iterator<Item = (usize, &mut Tree)> {
        self.entries
            .iter_mut()
            .enumerate()
            .filter(|(_, entry_type)| match entry_type {
                EntryType::Tree(_) => true,
                EntryType::Blob(_) => false,
            })
            .map(|(i, entry_type)| match entry_type {
                EntryType::Tree(tree) => (i, tree),
                EntryType::Blob(_) => panic!("Blob after tree filtering"),
            })
            .rev()
    }

    pub fn sha1(&self) -> Option<&str> {
        match &self.sha1 {
            Some(sha1) => Some(sha1),
            None => None,
        }
    }
}

// TODO: Serialize blobs and trees individually, probably not possible with ciborium
impl Storable for Tree {
    fn store(&self) {
        // We need to create a copy of the data to remove inner trees and entries while storing
        let mut tree = self.to_owned();

        tree.entries.iter().for_each(|entry_type| match entry_type {
            EntryType::Tree(tree) => tree.store(),
            EntryType::Blob(entry) => entry.store(),
        });

        tree.get_trees_mut()
            .for_each(|(_, tree)| tree.entries = Vec::new());

        let file = fs::File::create(Paths::objects() + "/" + tree.sha1().unwrap())
            .expect("Failed to create file to store tree");

        ciborium::ser::into_writer(&tree, file).expect("Failed to write data to tree");
    }
}

impl From<Index> for Tree {
    fn from(index: Index) -> Self {
        let entries = index.entries().to_owned();
        let tree = Tree::new("ROOT", entries);

        tree
    }
}

impl Object for Tree {
    fn new_from_object_file<'de>(sha1: &str) -> Self
    where
        Self: Deserialize<'de>,
    {
        let file = fs::File::open(Paths::objects() + "/" + &sha1)
            .expect(&format!("Failed to open {} to create object", sha1));

        let mut tree: Tree =
            ciborium::de::from_reader(file).expect("Failed to read data from object");

        let new_entries: Vec<EntryType> = tree
            .entries
            .iter()
            .map(|entry_type| match entry_type {
                EntryType::Tree(tree) => {
                    EntryType::Tree(Tree::new_from_object_file(tree.sha1().unwrap()))
                }
                EntryType::Blob(entry) => EntryType::Blob(entry.to_owned()),
            })
            .collect();

        tree.entries = new_entries;

        tree
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Entry {
    mode: u32,
    file_name: String,
    sha1: String,
}

impl From<index::Entry> for Entry {
    fn from(entry: index::Entry) -> Self {
        if entry.path().contains("/") {
            panic!("Index entry contains path");
        }

        Entry {
            mode: entry.mode(),
            file_name: String::from(entry.path()),
            sha1: String::from(entry.sha1()),
        }
    }
}

impl Storable for Entry {
    fn store(&self) {
        let path = Paths::objects() + "/" + &self.sha1;
        let path = path::Path::new(&path);
        if !path.exists() {
            eprintln!("{} was not found while checking", path.to_string_lossy());
        }
    }
}

impl Object for Entry {}
