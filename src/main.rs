use clap::{arg, Command};
use sqlite;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

const file_name: &str = "bugs.db";
static header: &[&'static str] = &[
    "source",
    "description",
    "solution",
    "found",
    "solved",
    "tags",
    "status",
];
const table: &str = "table";

fn cli() -> Command {
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
        .subcommand(Command::new("desc").about("Change the description of a bug."))
        .subcommand(Command::new("append").about("Append to the description of a bug"))
        .subcommand(Command::new("tag").about("Add a tag to the bug."))
        .subcommand(Command::new("reset").about("Reset the logs file. Must use --force tag."))
}

fn main() {
    let connection = sqlite::open(file_name).unwrap();
    //let query = "CREATE TABLE bugs (source varchar(255), description varchar(255), solution varchar(255), found varchar(255), solved varchar(255), tags varchar(255), status varchar(255))";
    //connection.execute(query).unwrap();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            let mut source = String::new();
            let mut tags = String::new();
            let mut desc = String::new();

            print!("Source: ");
            std::io::stdout().flush();
            let mut stdin = std::io::stdin();
            stdin.read_line(&mut source);

            print!("Description: ");
            std::io::stdout().flush();
            stdin = std::io::stdin();
            stdin.read_line(&mut desc);

            print!("tags (delim=' '): ");
            std::io::stdout().flush();
            stdin = std::io::stdin();
            stdin.read_line(&mut tags);

            let source = source.trim();
            let desc = desc.trim();
            let tags = tags.trim();

            println!("Source of bug: {}", source);
            println!("Bug description: {}", desc);

            let date = chrono::offset::Utc::now()
                .date_naive()
                .format("%Y-%m-%d")
                .to_string();

            let insertion = std::format!(
                "
INSERT INTO bugs
VALUES ('{}', '{}', 'unknown', '{}', 'unknown', '{}', 'unknown')",
                source,
                desc,
                date,
                tags
            );
            connection.execute(insertion).unwrap();
        }
        Some(("list", _)) => {
            let query = "SELECT * from bugs";
            for item in header {
                print!("{} :: ", item);
            }
            println!("");
            connection.iterate(query, |pairs| {
                for &(name, value) in pairs {
                    let item = value.unwrap();
                    print!("{} :: ", item);
                }
                println!("");
                true
            });
        }
        _ => (),
        //        Some(("solve", sub_matches)) => {
        //            let id = sub_matches.get_one::<String>("id");
        //            let date_solved = chrono::offset::Utc::now().date_naive().format("%Y-%m-%d").to_string();
        //
        //            if let Some(s) = id {
        //                let index = s.to_string().parse::<usize>().unwrap();
        //
        //                let mut position = Position::new();
        //                position.set_line(index as u64);
        //
        //                let mut iter = reader.into_records();
        //                iter.reader_mut().seek(position).unwrap();
        //                let mut iter = iter.into_reader().into_records();
        //                iter.next();
        //                if let Some(result) = iter.next() {
        //                    let record = result.unwrap();
        //                    println!("{:?}", record);
        //                }
        //
        //            } else {
        //                // change most recent one
        //            }
        //        }
        //        Some(("temp", sub_matches)) => {
        //            println!("Marking bug has a temp fix.")
        //        }
        //        Some(("unsolved", sub_matches)) => {
        //            println!("Marking a bug as unsolved")
        //        }
        //        Some(("delete", sub_matches)) => {
        //            println!("Deleting a bug.")
        //        }
        //        Some(("append", sub_matches)) => {
        //            println!("Adding to the description of a bug.")
        //        }
        //        Some(("tag", sub_matches)) => {
        //            println!("Adding a tag/s to a bug")
        //        }
        //        Some(("force", sub_matches)) => {
        //            println!("Reseting the bugs file.")
        //        }
        //        _ => ( }
    }
}
