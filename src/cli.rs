use clap::{Parser, Subcommand};

// CLI commands structure represented as a data structure

#[derive(Parser)]
#[command(name = "rgit")]
#[command(author = "Henri Egger")]
#[command(version = "1.0")]
#[command(about = "Tiny git in rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    Init,
    Add {
        file: String,
    },
    Commit {
        message: String,
    },
    Status,
    Checkout {
        sha: String,
        path: String,
    },
    Branch,
    Log,
    Dev {
        #[command(subcommand)]
        command: DevSubcommands,
    },
}

#[derive(Subcommand)]
pub enum DevSubcommands {
    Clean,
    BuildTree,
    DbgTree { sha: String },
    DbgCommit { sha: String },
}
