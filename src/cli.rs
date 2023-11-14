use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("bugger")
        .subcommand(Command::new("create").about("Create a log"))
        .subcommand(Command::new("list").about("List all bugs in the log."))
        .subcommand(
            Command::new("solve")
                .about("Marking bug as solved.")
                .arg(arg!(-i --id <ID> "Row number to solve."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("temp")
                .about("Marking bug as temporarily solved.")
                .arg(arg!(-i --id <ID>).required(true)),
        )
        .subcommand(Command::new("unsolved").about("Marking bug as unsolved"))
}

//TODO:
//list out unsolved bugs
