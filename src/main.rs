#![allow(unused)]
use cli::cli;

use std::io::Write;
use std::fs::{File, OpenOptions};

mod cli;
mod data;

const FILE_NAME: &str = "bugs.csv";

fn io(display: &str, retrieve: &mut String) {
    print!("{}", display);
    std::io::stdout().flush();
    let stdin = std::io::stdin();
    stdin.read_line(retrieve);
}

fn main() {
    let file = File::options().append(true).open(FILE_NAME);
    if let Err(error) = file {
        println!("Creating new file.");
        File::create(FILE_NAME);
        data::write_header(FILE_NAME);
    }
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            let mut source = String::new();
            let mut desc = String::new();
            let mut tags = String::new();

            io("Source: ", &mut source);
            io("Description: ", &mut desc);
            io("tags (delim=' '): ", &mut tags);

            data::write_new_entry(FILE_NAME, &source, &desc, &tags);
        }
        Some(("list", _)) => {
            data::list_csv(FILE_NAME);
        }
        Some(("solve", sub_matches)) => {}
        Some(("temp", sub_matches)) => {
            println!("Marking bug has a temp fix.")
        }
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    // header, delimiter, commands, args
}
