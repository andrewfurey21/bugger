use clap::{ arg, Command };
use std::path::Path;
use std::fs::{File, OpenOptions};
use csv::Writer;

const file_name: &str = "bugs.csv";
const header: &'static [&'static str] = &["source", "description", "solution", "found", "solved", "tags", "status"];

fn get_log() -> File {
    let bugs_path = Path::new(file_name);
    if !bugs_path.exists() {
        println!("Creating log...");
        File::create(bugs_path).unwrap();
        let mut writer = Writer::from_path(bugs_path).unwrap();
        writer.write_record(header);
        writer.flush();
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(bugs_path)
        .unwrap();

    return file;
}

fn cli() -> Command {
    Command::new("bugger")
        .subcommand(
            Command::new("create")
                .about("Create a bug log")
                .arg(arg!(<DESC> "Description of bug"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("list")
                .about("List all bugs in the log.")
        )
        .subcommand(
            Command::new("solve")
                .about("Marking bug as solved.")
        )
        .subcommand(
            Command::new("temp")
                .about("Marking bug has a temporary fix.")
        )
        .subcommand(
            Command::new("unsolved")
                .about("Marking bug as unsolved")
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a bug.")
        )
        .subcommand(
            Command::new("desc")
                .about("Change the description of a bug.")
        )
        .subcommand(
            Command::new("append")
                .about("Append to the description of a bug")
        )
        .subcommand(
            Command::new("tag")
                .about("Add a tag to the bug.")
        )
}

fn main() {
    let mut log = get_log();
    let mut writer = Writer::from_writer(log);

    let command = cli();
    let m = command.get_matches();


    match m.subcommand() {
        Some(("create", sub_matches)) => {
            println!("Creating bug log: {}", sub_matches.get_one::<String>("DESC").expect("required"))
        }
        Some(("list", _)) => {
            println!("Listing the logs")
        }
        Some(("solve", sub_matches)) => {
            println!("Solving a bug")
        }
        Some(("temp", sub_matches)) => {
            println!("Marking bug has a temp fix.")
        }
        Some(("unsolved", sub_matches)) => {
            println!("Marking a bug as unsolved")
        }
        Some(("delete", sub_matches)) => {
            println!("Deleting a bug.")
        }
        Some(("append", sub_matches)) => {
            println!("Adding to the description of a bug.")
        }
        Some(("tag", sub_matches)) => {
            println!("Adding a tag/s to a bug")
        }
        _ => ()
    }

}
