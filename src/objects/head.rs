use std::fs;

use crate::{objects::Commit, storing::Object, Paths};

pub struct Head {
    name: String,
    commit: Option<Commit>,
}

impl Head {
    fn read(name: String) -> Head {
        let buf = fs::read(Paths::heads() + "/" + &name).unwrap();

        if buf.len() == 0 {
            Head { name, commit: None }
        } else {
            let sha = String::from_utf8(buf).unwrap();
            let commit = Commit::new_from_object_file(&sha, None);

            Head {
                name,
                commit: Some(commit),
            }
        }
    }

    #[allow(non_snake_case)]
    fn HEAD() -> String {
        let buf = fs::read(Paths::HEAD()).unwrap();
        let HEAD = String::from_utf8(buf).unwrap();
        HEAD
    }

    #[allow(non_snake_case)]
    pub fn read_HEAD() -> Head {
        Head::read(Head::HEAD())
    }

    #[allow(non_snake_case)]
    pub fn update(&self, commit: &Commit) {
        fs::write(Paths::heads() + "/" + &self.name, commit.sha()).unwrap();
    }

    pub fn commit(&self) -> &Option<Commit> {
        &self.commit
    }
}
