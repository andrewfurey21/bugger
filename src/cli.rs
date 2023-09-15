use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("bugger")
        .subcommand(Command::new("create").about("Create a log"))
        .subcommand(Command::new("list").about("List all bugs in the log."))
        .subcommand(
            Command::new("solve")
                .about("Marking bug as solved.")
                .arg(arg!(-i --id <ID> "Row number to solve, empty if most recent one.")),
        )
        .subcommand(Command::new("temp").about("Marking bug has a temporary fix."))
        .subcommand(Command::new("unsolved").about("Marking bug as unsolved"))
        .subcommand(Command::new("delete").about("Delete a bug."))
        .subcommand(Command::new("tag").about("Add a tag to the bug."))
        .subcommand(Command::new("reset").about("Reset the logs file. Must use --force tag."))
}