use clap::Parser;
use rgit::cli::{Cli, Subcommands};
use rgit::commands;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Subcommands::Init => commands::init(),
        Subcommands::Add => println!("Nothing here yet :("),
        Subcommands::Commit => println!("Nothing here yet :("),
        Subcommands::Status => println!("Nothing here yet :("),
        Subcommands::Checkout => println!("Nothing here yet :("),
        Subcommands::Branch => println!("Nothing here yet :("),
    }
}
