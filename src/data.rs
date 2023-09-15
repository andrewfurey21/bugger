use csv::{ReaderBuilder, Trim, WriterBuilder};

static HEADER: &[&'static str] = &[
    "source",
    "description",
    "solution",
    "found",
    "solved",
    "tags",
    "status",
];

const DELIMITER: u8 = b';';

pub fn write_header(file_name: &str) {
    let error_msg = format!("There was a problem with the file '{}'", file_name);
    let mut writer = WriterBuilder::new()
        .delimiter(DELIMITER)
        .from_path(file_name)
        .expect(&error_msg);

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
