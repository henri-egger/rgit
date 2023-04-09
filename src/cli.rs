use clap::{Parser, Subcommand};

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
    Add { file: String },
    Commit,
    Status,
    Checkout,
    Branch,
}
