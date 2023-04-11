use clap::Parser;
use rgit::{
    cli::{Cli, DevSubcommands, Subcommands},
    commands,
};

// TODO: not run other commands if uninitialized

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Subcommands::Init => commands::init(),
        Subcommands::Add { file } => commands::add(file),
        Subcommands::Commit => println!("Nothing here yet :("),
        Subcommands::Status => println!("Nothing here yet :("),
        Subcommands::Checkout => println!("Nothing here yet :("),
        Subcommands::Branch => println!("Nothing here yet :("),
        Subcommands::Dev { command } => match command {
            DevSubcommands::Clean => commands::dev_commands::clean(),
            DevSubcommands::ListIndex => println!("Nothing here yet :("),
        },
    }
}
