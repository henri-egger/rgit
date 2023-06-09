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
        Subcommands::Commit { message } => Commands::commit(message),
        Subcommands::Status => Commands::status(),
        Subcommands::Checkout { sha, path } => Commands::checkout(sha, path),
        Subcommands::Branch => NonStorable,
        Subcommands::Log => Commands::log(),
        Subcommands::Dev { command } => match command {
            DevSubcommands::Clean => DevCommands::clean(),
            DevSubcommands::BuildTree => DevCommands::build_tree(),
            DevSubcommands::DbgTree { sha } => DevCommands::dbg_tree(sha),
            DevSubcommands::DbgCommit { sha } => DevCommands::dbg_commit(sha),
        },
    };

    // Centralized fs operations at the end of execution
    if let Storable(storable) = command_return_val {
        storable.store();
    }
}
