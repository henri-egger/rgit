use crate::{objects, Paths};
use std::{fmt, fs, path};

pub fn init() {
    if path::Path::new(&Paths::root()).exists() == true {
        println!("Working directory is already a repository");
        return;
    }

    println!("Initlizing repository...");

    for dir in Paths::dirs() {
        fs::create_dir_all(&dir).expect(&format!("Failed to create directory {}", dir));
    }

    fs::File::create(Paths::index())
        .expect(&format!("Failed to create directory {}", Paths::index()));
}

pub fn add(path: impl AsRef<path::Path> + fmt::Display) {
    let mut index = objects::Index::from_index_file();
    index.add_entry(path);
}

pub mod dev_commands {
    use crate::Paths;
    use std::fs;

    pub fn clean() {
        println!("Removing .rgit...");
        fs::remove_dir_all(Paths::root()).expect("Failed to remove .rgit");
        super::init();
    }
}
