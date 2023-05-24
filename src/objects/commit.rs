use crate::{
    identifiers,
    objects::Tree,
    storing::{Object, Storable},
    Paths,
};
use sha1_smol::Sha1;
use std::{fs, io::Write};

// TODO: make parents lazy loaded
#[derive(Debug, Clone)]
pub struct Commit {
    tree: Tree,
    parent: Option<Box<Commit>>,
    message: String,
}

impl Commit {
    pub fn new(tree: Tree, parent: Option<Commit>, message: String) -> Commit {
        let parent = if let Some(commit) = parent {
            Some(Box::new(commit))
        } else {
            None
        };

        Commit {
            tree,
            parent,
            message,
        }
    }

    pub fn sha(&self) -> String {
        let buf = self.serialize();

        let mut hasher = Sha1::new();
        hasher.update(&buf);
        let sha = hasher.digest().to_string();

        sha
    }

    pub fn restore(&self, path: String) {
        self.tree.restore(path);
    }

    pub fn log(&self) {
        println!("commit {}\n{}\n", self.sha(), self.message);
        if let Some(commit) = &self.parent {
            commit.log();
        }
    }
}

impl Storable for Commit {
    fn store(&self) {
        self.tree.store();

        let buf = self.serialize();

        let mut file = fs::File::create(Paths::objects() + "/" + &self.sha()).unwrap();
        file.write_all(&buf).unwrap();
    }
}

impl Object for Commit {
    fn new_from_object_file(sha: &str, _: Option<String>) -> Self {
        let buf = fs::read(Paths::objects() + "/" + sha).unwrap();

        let mut parts = buf.split(|x| *x == b'\0');

        parts.next();
        let tree_sha = parts.next().unwrap();
        let tree_sha = &tree_sha[identifiers::TREE.len() + 1..];
        assert_eq!(tree_sha.len(), 40);
        let tree_sha = String::from_utf8(tree_sha.into()).unwrap();
        let tree = Tree::new_from_object_file(&tree_sha, Some(String::from("ROOT")));

        let parent_sha = parts.next().unwrap();
        let parent_sha = if parent_sha.len() != 0 {
            Some(&parent_sha[identifiers::PARENT.len() + 1..])
        } else {
            None
        };
        let parent_sha = if let Some(parent_sha) = parent_sha {
            assert_eq!(parent_sha.len(), 40);
            Some(String::from_utf8(parent_sha.into()).unwrap())
        } else {
            None
        };
        let parent = if let Some(parent_sha) = parent_sha {
            Some(Commit::new_from_object_file(&parent_sha, None))
        } else {
            None
        };

        let message = parts.next().unwrap();
        let message = String::from_utf8(message.into()).unwrap();

        Commit::new(tree, parent, message)
    }

    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.extend(identifiers::COMMIT.bytes());
        buf.push(b' ');
        buf.push(b'\0');
        buf.extend(identifiers::TREE.bytes());
        buf.push(b' ');
        buf.extend(self.tree.sha().bytes());
        buf.push(b'\0');
        if let Some(parent) = &self.parent {
            buf.extend(identifiers::PARENT.bytes());
            buf.push(b' ');
            buf.extend(parent.sha().bytes());
        };
        buf.push(b'\0');
        buf.extend(self.message.bytes());

        buf
    }
}
