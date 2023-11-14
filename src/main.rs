#![allow(unused)]
use cli::cli;

use std::fs::{File, OpenOptions};
use std::io::Write;

mod cli;
mod data;

const FILE_NAME: &str = "bugs.csv";

//Set to where ever you want the bugs.csv
const DIR_NAME: &str = "/home/andrew/";

fn main() {
    let file_path = std::path::PathBuf::from(DIR_NAME).join(FILE_NAME);

    let file = File::options().append(true).open(&file_path);
    if let Err(_) = file {
        println!("Creating new {} file.", FILE_NAME);
        File::create(&file_path).unwrap();
        data::write_header(&file_path);
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

            data::write_new_entry(&file_path, &source, &desc, &tags);
        }
        Some(("list", _)) => {
            //TODO: should only print unsolved, extra args for all or temp/solved
            data::list_csv(&file_path);
        }
        Some(("solve", sub_matches)) => {
            let id = sub_matches.get_one::<String>("id").unwrap();
            let id = id.parse::<usize>().unwrap();
            data::edit_line(&file_path, id, data::Status::Solved);
        }
        Some(("temp", sub_matches)) => {
            let id = sub_matches.get_one::<String>("id").unwrap();
            let id = id.parse::<usize>().unwrap();
            data::edit_line(&file_path, id, data::Status::Temp);
        }
        //Some(("unsolved", sub_matches)) => {}
        _ => (),
    }
}
