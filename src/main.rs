use clap::Parser;
use rgit::{
    cli::{Cli, DevSubcommands, Subcommands},
    commands,
    storing::Storable,
    DirBuilder,
};

// TODO: not run other commands if uninitialized, replace all empty storables
fn main() {
    let cli = Cli::parse();

    let storable: Box<dyn Storable> = match cli.command {
        Subcommands::Init => Box::new(DirBuilder),
        Subcommands::Add { file } => Box::new(commands::add(file)),
        Subcommands::Commit => Box::new(EmptyStorable),
        Subcommands::Status => Box::new(EmptyStorable),
        Subcommands::Checkout => Box::new(EmptyStorable),
        Subcommands::Branch => Box::new(EmptyStorable),
        Subcommands::Dev { command } => match command {
            DevSubcommands::Clean => Box::new(DirBuilder::clean()),
            DevSubcommands::ListIndex => Box::new(EmptyStorable),
        },
    };

    storable.store();
}

struct EmptyStorable;

impl Storable for EmptyStorable {
    fn store(&self) {
        println!("Nothing here yet :(")
    }
}
