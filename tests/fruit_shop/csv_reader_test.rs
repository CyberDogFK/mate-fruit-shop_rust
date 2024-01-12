use fruit_shop_rust::csv_reader;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;
use std::ops::{Deref, DerefMut};

const CORRECT_EXAMPLE_FILE: &str = "example.csv";
const NON_EXISTED_FILE: &str = "";
const CORRECT_EXAMPLE_LIST: [&str; 9] = [
    "type,fruit,quantity",
    "b,banana,20",
    "b,apple,100",
    "s,banana,100",
    "p,banana,13",
    "r,apple,10",
    "p,apple,20",
    "p,banana,5",
    "s,banana,50",
];

const CORRECT_NUMBER_OF_SKIP_LINES: i32 = 3;
const NON_CORRECT_NUMBER_OF_SKIP_LINES: i32 = 100;

fn init() {
    create_new_file(CORRECT_EXAMPLE_FILE).expect("Can't create file!");
}

fn after_all() {
    std::fs::remove_file(CORRECT_EXAMPLE_FILE).expect("Can't remove file!")
}

struct TestFile {
    file: File,
    name: String
}

impl TestFile {
    pub fn create_with_random_name() -> std::io::Result<TestFile> {
        let name = Uuid::new_v4().to_string();
        Ok(TestFile {
            file: File::create(name.as_str())?,
            name,
        })
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        std::fs::remove_file(self.name.as_str())
            .expect("Can't remove file!")
    }
}

impl Deref for TestFile {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl DerefMut for TestFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}

fn create_new_file(path: &str) -> std::io::Result<()> {
    let mut file = TestFile::create_with_random_name()?;
    let mut buffer: Vec<u8> = Vec::new();
    {
        CORRECT_EXAMPLE_LIST.iter().for_each(|i| {
            let i = format!("{i}\n");
            buffer.append(&mut i.as_bytes().to_vec())
        });
    }
    file.write_all(&buffer)
}

#[test]
fn read_skip_correct_number_of_lines() {
    init();
    let expected_list = CORRECT_EXAMPLE_LIST
        [(CORRECT_NUMBER_OF_SKIP_LINES as usize)..CORRECT_EXAMPLE_LIST.len()]
        .to_vec();
    let actual_list =
        csv_reader::read_file_with_skip_lines(CORRECT_EXAMPLE_FILE, CORRECT_NUMBER_OF_SKIP_LINES)
            .unwrap();
    assert_eq!(expected_list, actual_list);
}

#[test]
fn read_skip_non_correct_number_of_lines() {
    init();
    let expected: Vec<String> = Vec::new();
    let actual_list = csv_reader::read_file_with_skip_lines(
        CORRECT_EXAMPLE_FILE,
        NON_CORRECT_NUMBER_OF_SKIP_LINES,
    )
    .unwrap();
    assert_eq!(expected, actual_list);
}

#[test]
fn read_correct_file() {
    init();
    let actual = csv_reader::read_file(CORRECT_EXAMPLE_FILE).unwrap();
    assert_eq!(CORRECT_EXAMPLE_LIST.to_vec(), actual);
}

#[test]
fn read_non_existed_file_error() {
    assert!(csv_reader::read_file(NON_EXISTED_FILE).is_err());
}

#[test]
fn health_check() {
    assert!(true)
}
