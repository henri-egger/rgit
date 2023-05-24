use crate::{
    objects::{Commit, Index, Tree},
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
        let mut index = Index::new_from_index_file();
        index.add(path);
        CommandReturnType::Storable(Box::new(index))
    }

    pub fn status() -> CommandReturnType {
        let index = Index::new_from_index_file();
        index.status();

        CommandReturnType::NonStorable
    }

    pub fn commit() -> CommandReturnType {
        let index = Index::new_from_index_file();
        let tree = Tree::from(index);
        let parent = Commit::new_from_object_file("9622c332b4ee2aa550ad7ea3be71082a0271aeca", None);
        let commit = Commit::new(
            tree,
            Some(parent),
            String::from("Some second commit message"),
        );
        dbg!(&commit);
        dbg!(commit.sha());

        CommandReturnType::Storable(Box::new(commit))
    }
}

pub struct DevCommands;

impl DevCommands {
    pub fn clean() -> CommandReturnType {
        CommandReturnType::Storable(Box::new(DirBuilder::clean()))
    }

    pub fn build_tree() -> CommandReturnType {
        let index = Index::new_from_index_file();
        let tree = Tree::from(index);
        tree.print_shas();

        CommandReturnType::Storable(Box::new(tree))
    }

    pub fn dbg_tree(sha: &str) -> CommandReturnType {
        let tree = Tree::new_from_object_file(sha, Some(String::from("ROOT")));
        dbg!(&tree);

        CommandReturnType::NonStorable
    }

    pub fn dbg_commit(sha: &str) -> CommandReturnType {
        let commit = Commit::new_from_object_file(sha, None);
        dbg!(&commit);

        CommandReturnType::NonStorable
    }
}
