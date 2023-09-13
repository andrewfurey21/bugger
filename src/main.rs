use clap::{ Command };
use std::path::Path;
use std::fs::{File, OpenOptions};
use csv::Writer;
use std::io::Write;

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
                .about("Create a log")
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
        .subcommand(
            Command::new("reset")
                .about("Reset the logs file. Must use --force tag.")
        )
}

fn main() {
    let mut log = get_log();
    let mut writer = Writer::from_writer(log);

    let command = cli();
    let m = command.get_matches();


    match m.subcommand() {
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


            let date = chrono::offset::Utc::now().date_naive().format("%Y-%m-%d").to_string();
            let record = &[source, desc, "unknown", date.as_str(), "unknown", tags, "unsolved"];
            writer.write_record(record);
            writer.flush();
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
        Some(("force", sub_matches)) => {
            println!("Reseting the bugs file.")
        }
        _ => ()
    }


}
