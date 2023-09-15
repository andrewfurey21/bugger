use csv::Reader;

const FILE_NAME: &str = "bugs.db";
static HEADER: &[&'static str] = &[
    "source",
    "description",
    "solution",
    "found",
    "solved",
    "tags",
    "status",
];
