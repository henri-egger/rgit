use clap::Parser;
use rgit::{
    cli::{Cli, DevSubcommands, Subcommands},
    commands::{
        CommandReturnType::{self, NonStorable, Storable},
        Commands, DevCommands,
    },
};

// TODO: not run other commands if uninitialized, replace all empty storables
fn main() {
    let cli = Cli::parse();

    let command_return_val: CommandReturnType = match cli.command {
        Subcommands::Init => Commands::init(),
        Subcommands::Add { file } => Commands::add(file),
        Subcommands::Commit => NonStorable,
        Subcommands::Status => Commands::status(),
        Subcommands::Checkout => NonStorable,
        Subcommands::Branch => NonStorable,
        Subcommands::Dev { command } => match command {
            DevSubcommands::Clean => DevCommands::clean(),
            DevSubcommands::ListIndex => NonStorable,
        },
    };

    if let Storable(storable) = command_return_val {
        storable.store();
    }
}
