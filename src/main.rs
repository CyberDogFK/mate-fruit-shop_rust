use fruit_shop_rust::db::FruitStorage;
use fruit_shop_rust::service::csv_reader::read_file;
use fruit_shop_rust::service::CsvWriter;

const NUMBER_OF_TITLE_IN_CSV: i32 = 1;
const EXAMPLE_FILE: &str = "example.csv";
const TITLE_FOR_CSV: &str = "fruit,quantity";
const RESULT_TARGET_FILE: &str = "sms.csv";

fn main() {
    let fruit_storage = FruitStorage::new();
    let res = read_file(EXAMPLE_FILE).expect("Can't read file");
    println!("{:?}", res);

    let writer = CsvWriter::new(Some("Cool title".to_string()));
    let r = writer
        .save_to_file(
            "some_new_file.csv",
            "BigShit\nSomeShit\nNextLine".to_string(),
        )
        .expect("Can't write file!");
    // println!("written bytes {}", r);
}
