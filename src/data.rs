use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use prettytable::{self, color, Attr, Cell, Row};

static HEADER: &[&'static str] = &[
    "id",
    "source",
    "description",
    "solution",
    "found",
    "solved",
    "tags",
    "status",
];
const HEADER_LENGTH: usize = 8;
const DELIMITER: char = ';';

pub fn io(display: &str, retrieve: &mut String) {
    print!("{}", display);
    std::io::stdout().flush();
    let stdin = std::io::stdin();
    stdin.read_line(retrieve);
}

pub enum Status {
    Solved,
    Temp,
    Unsolved,
}

pub fn write_header(file_name: &PathBuf) {
    let mut file = std::fs::File::options()
        .append(true)
        .open(file_name)
        .expect("Couldn't open file.");

    for col in HEADER {
        write!(file, "{};", col);
    }
    writeln!(file, "");
}

pub fn list_csv(file_name: &PathBuf) {
    let mut table = prettytable::Table::new();

    let mut header: Vec<Cell> = vec![];
    for item in HEADER {
        header.push(
            Cell::new(*item)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN)),
        );
    }
    table.add_row(Row::new(header));

    let data = std::fs::read_to_string(file_name).unwrap();
    let data = data.trim();
    let mut data = data.split("\n").collect::<Vec<_>>();
    data.remove(0);
    for line in data {
        let mut items = line.split(DELIMITER).collect::<Vec<_>>();
        items.pop();
        let mut row = Row::empty();
        for index in 0..items.len() {
            if index == 7 {
                match items[index] {
                    "solved" => row.add_cell(
                        Cell::new(items[index]).with_style(Attr::ForegroundColor(color::GREEN)),
                    ),
                    "temp" => row.add_cell(
                        Cell::new(items[index]).with_style(Attr::ForegroundColor(color::YELLOW)),
                    ),
                    "unsolved" => row.add_cell(
                        Cell::new(items[index]).with_style(Attr::ForegroundColor(color::RED)),
                    ),
                    item => panic!("Unknown status item: {}", item),
                }
            } else {
                row.add_cell(Cell::new(items[index]));
            }
        }
        table.add_row(row);
    }
    table.printstd();
}

pub fn write_new_entry(file_name: &PathBuf, source: &String, desc: &String, tags: &String) {
    let mut file = std::fs::File::options()
        .append(true)
        .open(file_name)
        .expect("Couldn't open file.");
    let source = source.trim();
    let desc = desc.trim();
    let tags = tags.trim();
    let id = std::fs::read_to_string(file_name)
        .unwrap()
        .split("\n")
        .count()
        - 2;
    let id = id.to_string();
    let date = &chrono::offset::Utc::now()
        .date_naive()
        .format("%Y-%m-%d")
        .to_string();
    let items = [
        &id, source, desc, "unknown", date, "unknown", tags, "unsolved",
    ];
    for col in items {
        write!(file, "{};", col);
    }
    writeln!(file, "");
}

pub fn edit_line(file_name: &PathBuf, index: usize, status: Status) {
    let data = std::fs::read_to_string(file_name).unwrap();
    let mut data = data.split('\n').collect::<Vec<_>>();
    data.remove(0);
    data.remove(data.len() - 1);

    let update = data
        .iter()
        .filter(|line| {
            let split_line = line.split(DELIMITER).collect::<Vec<_>>();
            let i = split_line[0].parse::<usize>().unwrap();
            index == i
        })
        .collect::<Vec<_>>();

    let data = data
        .iter()
        .filter(|line| {
            let split_line = line.split(DELIMITER).collect::<Vec<_>>();
            let i = split_line[0].parse::<usize>().unwrap();
            !(index == i)
        })
        .collect::<Vec<_>>();

    let split_line = update[0].split(DELIMITER).collect::<Vec<_>>();
    let id = String::from(split_line[0]);
    let source = String::from(split_line[1]);
    let desc = String::from(split_line[2]);
    let found = String::from(split_line[4]);
    let tags = String::from(split_line[6]);

    let date_solved = chrono::offset::Utc::now()
        .date_naive()
        .format("%Y-%m-%d")
        .to_string();
    let new_line = match status {
        Status::Solved => {
            let mut solution = String::new();
            io("Solution: ", &mut solution);
            let solution = String::from(solution.trim());
            vec![
                id,
                source,
                desc,
                solution,
                found,
                date_solved,
                tags,
                String::from("solved"),
            ]
        }
        Status::Temp => {
            let mut solution = String::new();
            io("Temp Solution: ", &mut solution);
            let solution = String::from(solution.trim());
            vec![
                id,
                source,
                desc,
                solution,
                found,
                date_solved,
                tags,
                String::from("temp"),
            ]
        }
        //Status::Unsolved => {
        //}
        _ => {
            vec![String::new()]
        }
    };

    let mut file = std::fs::File::create(file_name).expect("Couldn't open file.");
    file.set_len(0);
    for col in HEADER {
        write!(file, "{};", col);
    }
    writeln!(file, "");
    for line in data {
        writeln!(file, "{}", line);
    }
    for item in new_line {
        write!(file, "{};", item);
    }
    writeln!(file, "");
}

#[cfg(test)]
mod tests {
    // header, delimiter, commands, args
}
