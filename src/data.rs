use csv::{ReaderBuilder, Reader, Trim, WriterBuilder, Writer};
use std::fs::File;

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
const DELIMITER: u8 = b';';

fn create_writer(file_name: &str) -> Writer<File> {
    let error_msg = format!("There was a problem with the file '{}'", file_name);
    let file = File::options().append(true).open(file_name).expect(&error_msg);
    WriterBuilder::new()
        .delimiter(DELIMITER)
        .from_writer(file)
}

fn create_reader(file_name: &str) -> Reader<File> {
    let error_msg = format!("Could not read from '{}'", file_name);
    let mut reader = ReaderBuilder::new()
        .delimiter(DELIMITER)
        .trim(Trim::All)
        .from_path(file_name)
        .expect(&error_msg);
    return reader;
}

pub fn write_header(file_name: &str) {
    let mut writer = create_writer(file_name);
    writer.write_record(HEADER).expect("Couldn't write header to file.");
    writer.flush().unwrap();
}

pub fn list_csv(file_name: &str) {
    //TODO: add header
    let mut reader = create_reader(file_name);
    for record in reader.records() {
        println!("{:?}", record);
    }
}


pub fn write_new_entry(file_name: &str, source: &String, desc: &String, tags: &String) {
    let source = source.trim();
    let desc = desc.trim();
    let tags = tags.trim();
    let id = create_reader(file_name).records().count().to_string();
    let date = &chrono::offset::Utc::now()
        .date_naive()
        .format("%Y-%m-%d")
        .to_string();
    let items = [&id, source, desc, "unknown", date, "unknown", tags, "unknown"];
    write_line(file_name, &items);
}

fn write_line(file_name: &str, items: &[&str; HEADER_LENGTH]) {
    let mut writer = create_writer(file_name);
    writer.write_record(items);
    writer.flush();
}
