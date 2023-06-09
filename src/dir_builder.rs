use crate::{storing::Storable, Paths};
use std::{fs, path};

/// Struct implementing the store trait so it can be returned to main for centralized fs operations
pub struct DirBuilder;

impl DirBuilder {
    pub fn clean() -> DirBuilder {
        println!("Removing .rgit...");
        fs::remove_dir_all(Paths::root()).expect("Failed to remove .rgit");
        DirBuilder
    }
}

impl Storable for DirBuilder {
    fn store(&self) {
        if path::Path::new(&Paths::root()).exists() == true {
            println!("Working directory is already a repository");
            return;
        }

        println!("Initlizing repository...");

        for dir in Paths::dirs() {
            fs::create_dir_all(&dir).expect(&format!("Failed to create directory {}", dir));
        }

        fs::write(Paths::HEAD(), "main").unwrap();
        fs::File::create(Paths::heads() + "/main").unwrap();

        fs::File::create(Paths::index())
            .expect(&format!("Failed to create directory {}", Paths::index()));
    }
}
