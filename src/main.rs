#![allow(unused)]
use cli::cli;
use data::*;
use std::io::Write;

mod cli;
mod data;

fn io(display: &str, retrieve: &mut String) {
    print!("{}", display);
    std::io::stdout().flush();
    let stdin = std::io::stdin();
    stdin.read_line(retrieve);
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            let mut source = String::new();
            let mut tags = String::new();
            let mut desc = String::new();

            io("Source: ", &mut source);
            io("Description: ", &mut desc);
            io("tags (delim=' '): ", &mut tags);

            let date = chrono::offset::Utc::now()
                .date_naive()
                .format("%Y-%m-%d")
                .to_string();
        }
        Some(("list", _)) => {}
        Some(("solve", sub_matches)) => {}
        Some(("temp", sub_matches)) => {
            println!("Marking bug has a temp fix.")
        }
        Some(("unsolved", sub_matches)) => {
            println!("Marking a bug as unsolved")
        }
        Some(("delete", sub_matches)) => {
            println!("Deleting a bug.")
        }
        Some(("tag", sub_matches)) => {
            println!("Adding a tag/s to a bug")
        }
        Some(("reset", sub_matches)) => {
            println!("Reseting the bugs file.")
        }
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 1 + 2;
        assert_eq!(result, 4);
    }
}
