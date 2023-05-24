use crate::{
    identifiers,
    objects::{index, Blob, Index},
    storing::{Object, Storable},
    Paths,
};
use sha1_smol::Sha1;
use std::{fs, io::Write, os::unix::prelude::PermissionsExt, path};

const ENCODING_RADIX: u32 = 10;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum EntryType {
    Tree(Tree),
    Blob(Entry),
}

trait TreeEntry {
    fn serialize_as_entry(&self) -> Vec<u8>;
    fn deserialize_as_entry(buf: Vec<u8>) -> EntryType;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree {
    entries: Vec<EntryType>,
    name: String,
}

impl Tree {
    pub fn new(name: &str, entries: Vec<index::Entry>) -> Tree {
        let mut tree = Tree {
            entries: Vec::new(),
            name: String::from(name),
        };

        tree.add_index_entries(entries);

        tree
    }

    pub fn add_index_entries(&mut self, mut entries: Vec<index::Entry>) {
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

    // fn get_trees_mut(&mut self) -> impl Iterator<Item = (usize, &mut Tree)> {
    //     self.entries
    //         .iter_mut()
    //         .enumerate()
    //         .filter(|(_, entry_type)| match entry_type {
    //             EntryType::Tree(_) => true,
    //             EntryType::Blob(_) => false,
    //         })
    //         .map(|(i, entry_type)| match entry_type {
    //             EntryType::Tree(tree) => (i, tree),
    //             EntryType::Blob(_) => panic!("Blob after tree filtering"),
    //         })
    //         .rev()
    // }

    pub fn sha(&self) -> String {
        let buf = self.serialize();

        let mut hasher = Sha1::new();
        hasher.update(&buf);
        let sha = hasher.digest().to_string();

        sha
    }

    // util function
    pub fn print_shas(&self) {
        println!("{}: {}", self.name, self.sha());

        for entry in &self.entries {
            match entry {
                EntryType::Tree(tree) => tree.print_shas(),
                EntryType::Blob(blob) => println!("{}: {}", blob.file_name, blob.sha),
            };
        }
    }

    pub fn restore(&self, path: String) {
        let is_root = self.name.eq("ROOT");

        if path::PathBuf::from(&path).exists() && is_root {
            panic!("{} already exists", path);
        }

        let path = if is_root {
            path
        } else {
            format!("{}/{}", path, self.name)
        };

        fs::DirBuilder::new().recursive(true).create(&path).unwrap();

        for entry in &self.entries {
            match entry {
                EntryType::Tree(tree) => tree.restore(path.to_owned()),
                EntryType::Blob(blob) => blob.restore(path.to_owned()),
            }
        }
    }
}

impl From<Index> for Tree {
    fn from(index: Index) -> Self {
        let entries = index.entries().to_owned();
        let tree = Tree::new("ROOT", entries);

        tree
    }
}

impl Storable for Tree {
    fn store(&self) {
        self.get_trees().for_each(|(_, tree)| tree.store());

        let buf = self.serialize();

        let mut file = fs::File::create(Paths::objects() + "/" + &self.sha())
            .expect("Failed to create file to store tree");

        file.write_all(&buf).unwrap();
    }
}

impl Object for Tree {
    fn new_from_object_file(sha: &str, name: Option<String>) -> Self {
        let buf = fs::read(Paths::objects() + "/" + sha).unwrap();

        let null_i = buf.iter().position(|x| *x == b'\0').unwrap();
        let buf = Vec::from(&buf[null_i + 1..]);

        let bufs = buf.split(|x| *x == b'\0').filter(|x| x.len() != 0);

        let entries = bufs
            .map(|buf| {
                if buf.starts_with(identifiers::TREE.as_bytes()) {
                    Tree::deserialize_as_entry(buf.into())
                } else if buf.starts_with(identifiers::BLOB.as_bytes()) {
                    Entry::deserialize_as_entry(buf.into())
                } else {
                    panic!();
                }
            })
            .collect();

        Tree {
            entries,
            name: name.unwrap(),
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.extend(identifiers::TREE.bytes());
        buf.push(b' ');

        let mut entries = self.entries.to_owned();
        entries.sort();
        let entries = entries
            .iter()
            .map(|entry| match entry {
                EntryType::Tree(tree) => tree.serialize_as_entry(),
                EntryType::Blob(blob) => blob.serialize_as_entry(),
            })
            .fold(&mut Vec::new(), |buf, entry| {
                buf.extend(entry);
                buf.push(b'\0');
                buf
            })
            .to_owned();

        buf.extend(entries.len().to_string().bytes());
        buf.push(b'\0');
        buf.extend(entries);

        buf
    }
}

impl TreeEntry for Tree {
    fn serialize_as_entry(&self) -> Vec<u8> {
        format!(
            "{} {} {} {}",
            identifiers::TREE,
            0o755,
            self.name,
            self.sha()
        )
        .as_bytes()
        .to_owned()
    }

    fn deserialize_as_entry(buf: Vec<u8>) -> EntryType {
        if !buf.starts_with(identifiers::TREE.as_bytes()) {
            panic!()
        }

        let mut parts = buf.split(|x| *x == b' ');

        parts.next();
        parts.next();
        let name = parts.next().unwrap();
        let sha = parts.next().unwrap();

        let name = String::from_utf8(name.into()).unwrap();

        let sha = String::from_utf8(sha.into()).unwrap();

        let tree = Tree::new_from_object_file(&sha, Some(name));

        EntryType::Tree(tree)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Entry {
    mode: u32,
    file_name: String,
    sha: String,
}

impl Entry {
    fn restore(&self, path: String) {
        let path = format!("{}/{}", path, self.file_name);
        let mut file = fs::File::create(path).unwrap();

        file.set_permissions(fs::Permissions::from_mode(self.mode))
            .unwrap();

        let blob = Blob::new_from_object_file(&self.sha, None);
        let buf = blob.bytes();

        file.write(buf).unwrap();
    }
}

impl From<index::Entry> for Entry {
    fn from(entry: index::Entry) -> Self {
        if entry.path().contains("/") {
            panic!("Index entry contains path");
        }

        Entry {
            mode: entry.mode(),
            file_name: String::from(entry.path()),
            sha: String::from(entry.sha()),
        }
    }
}

impl Storable for Entry {
    fn store(&self) {
        let path = Paths::objects() + "/" + &self.sha;
        let path = path::Path::new(&path);
        if !path.exists() {
            eprintln!("{} was not found while checking", path.to_string_lossy());
        }
    }
}

impl TreeEntry for Entry {
    fn serialize_as_entry(&self) -> Vec<u8> {
        format!(
            "{} {} {} {}",
            identifiers::BLOB,
            self.mode,
            self.file_name,
            self.sha
        )
        .as_bytes()
        .to_owned()
    }

    fn deserialize_as_entry(buf: Vec<u8>) -> EntryType {
        if !buf.starts_with(identifiers::BLOB.as_bytes()) {
            panic!()
        }

        let mut parts = buf.split(|x| *x == b' ');

        parts.next();
        let mode = parts.next().unwrap();
        let file_name = parts.next().unwrap();
        let sha = parts.next().unwrap();

        let mode = String::from_utf8(mode.into()).unwrap();
        let mode = u32::from_str_radix(&mode, ENCODING_RADIX).unwrap();

        let file_name = String::from_utf8(file_name.into()).unwrap();

        let sha = String::from_utf8(sha.into()).unwrap();

        let entry = Entry {
            mode,
            file_name,
            sha,
        };

        EntryType::Blob(entry)
    }
}
