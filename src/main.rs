#![allow(unused)]
use cli::cli;

use std::fs::{File, OpenOptions};
use std::io::Write;

mod cli;
mod data;

const FILE_NAME: &str = "bugs.csv";

fn main() {
    let file = File::options().append(true).open(FILE_NAME);
    if let Err(_) = file {
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

            data::io("Source: ", &mut source);
            data::io("Description: ", &mut desc);
            data::io("tags (delim=' '): ", &mut tags);

            data::write_new_entry(FILE_NAME, &source, &desc, &tags);
        }
        Some(("list", _)) => {
            data::list_csv(FILE_NAME);
        }
        Some(("solve", sub_matches)) => {
            data::edit_line(FILE_NAME, 0, data::Status::Solved);
        }
        Some(("temp", sub_matches)) => {}
        Some(("unsolved", sub_matches)) => {}
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    // header, delimiter, commands, args
}
