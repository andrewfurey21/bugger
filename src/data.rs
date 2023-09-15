use csv::{ReaderBuilder, Trim, WriterBuilder, Writer};
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
    let mut writer = WriterBuilder::new()
        .delimiter(DELIMITER)
        .from_path(file_name)
        .expect(&error_msg);

    return writer;
}

pub fn write_header(file_name: &str) {
    let mut writer = create_writer(file_name);
    writer.write_record(HEADER).expect("Couldn't write header to file.");
    writer.flush();
}

pub fn list_csv(file_name: &str) {
    let error_msg = format!("Could not read from '{}'", file_name);
    let mut reader = ReaderBuilder::new()
        .delimiter(DELIMITER)
        .has_headers(true)
        .trim(Trim::All)
        .from_path(file_name)
        .expect(&error_msg);

    for record in reader.records() {
        println!("{:?}", record);
    }
}

pub fn write_line(file_name: &str, items: &[String; HEADER_LENGTH]) {
    let mut writer = create_writer(file_name);
}
