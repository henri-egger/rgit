use crate::{
    objects::{self, Tree},
    storing::{Object, Storable},
    DirBuilder,
};

pub enum CommandReturnType {
    Storable(Box<dyn Storable>),
    NonStorable,
}

pub struct Commands;

impl Commands {
    pub fn init() -> CommandReturnType {
        CommandReturnType::Storable(Box::new(DirBuilder))
    }

    pub fn add(path: String) -> CommandReturnType {
        let mut index = objects::Index::new_from_index_file();
        index.add(path);
        CommandReturnType::Storable(Box::new(index))
    }

    pub fn status() -> CommandReturnType {
        let index = objects::Index::new_from_index_file();
        index.status();

        CommandReturnType::NonStorable
    }
}

pub struct DevCommands;

impl DevCommands {
    pub fn clean() -> CommandReturnType {
        CommandReturnType::Storable(Box::new(DirBuilder::clean()))
    }

    pub fn build_tree() -> CommandReturnType {
        let index = objects::Index::new_from_index_file();
        let tree = Tree::from(index);
        tree.print_shas();

        CommandReturnType::Storable(Box::new(tree))
    }

    pub fn dbg_tree(sha: &str) -> CommandReturnType {
        let tree = Tree::new_from_object_file(sha, Some(String::from("ROOT")));
        dbg!(&tree);

        CommandReturnType::NonStorable
    }
}
