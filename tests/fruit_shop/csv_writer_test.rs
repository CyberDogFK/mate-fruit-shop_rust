use std::fs::File;
use std::io::{BufWriter, Write};
use std::string::ToString;
use uuid::Uuid;
use fruit_shop_rust::service::CsvWriter;
use crate::helper::TestFile;

const ONE_LINE_TO_FILE_EXAMPLE_1: &str = "Example line 1";
const ONE_LINE_TO_FILE_EXAMPLE_2: &str = "Example line 2";
const TWO_LINE_TO_FILE: &str = "First line\nSecond line";
const TITLE_TO_FILE: &str = "Title";

#[test]
fn write_with_title() {
    let csv_writer = CsvWriter::new(
        Some(
            TITLE_TO_FILE.to_string()
        )
    );
    let file_name = Uuid::new_v4().to_string();
    csv_writer.save_to_file(
        &file_name,
        TWO_LINE_TO_FILE.to_string(),
    ).unwrap();
    let result_file = TestFile::from_file(file_name);
    let result_string = result_file.read_from_file();
    let expected_string = format!("{TITLE_TO_FILE}\n{TWO_LINE_TO_FILE}");
    assert_eq!(expected_string, result_string)
}

#[test]
fn write_without_title() {
    let csv_writer = CsvWriter::new(None);
    let file_name = Uuid::new_v4().to_string();
    csv_writer.save_to_file(
        &file_name,
        TWO_LINE_TO_FILE.to_string(),
    ).unwrap();
    let result_file = TestFile::from_file(file_name);
    let result_string = result_file.read_from_file();
    assert_eq!(TWO_LINE_TO_FILE, result_string)
}

#[test]
fn rewrite_lines_from_existed_file() {
    let file_name = Uuid::new_v4().to_string();
    {
        let file = File::create(&file_name).unwrap();
        let mut wri = BufWriter::new(file);
        wri.write_all(ONE_LINE_TO_FILE_EXAMPLE_1.as_bytes()).unwrap();
    }

    let test_file = TestFile::from_file(file_name);

    let _ = CsvWriter::new(None)
        .save_to_file(&test_file.name,
                      ONE_LINE_TO_FILE_EXAMPLE_2.to_string());
    let actual = test_file.read_from_file();
    assert_eq!(ONE_LINE_TO_FILE_EXAMPLE_2, actual)
}
