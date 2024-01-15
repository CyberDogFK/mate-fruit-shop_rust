use std::cell::RefCell;
use std::rc::Rc;
use fruit_shop_rust::db::FruitStorage;
use fruit_shop_rust::service::create_report;

const EMPTY_LINE: &str = "";
const TEST_NAME_1: &str = "banana";
const TEST_VALUE_1: i32 = 20;
const TEST_RESULT_1: &str = "banana,20";
const TEST_NAME_2: &str = "apple";
const TEST_VALUE_2: i32 = 30;
const TEST_RESULT_2: &str = "apple,30";
const TEST_NAME_3: &str = "milk";
const TEST_VALUE_3: i32 = 100;
const TEST_RESULT_3: &str = "milk,100";

fn get_fruit_storage() -> FruitStorage {
    FruitStorage::new()
}

#[test]
fn report_empty_storage() {
    let storage = Rc::new(RefCell::new(FruitStorage::new()));
    let actual = create_report(storage);
    assert_eq!(EMPTY_LINE, actual)
}

#[test]
fn report_get_correct_report_from_lines() {
    let mut storage_ref = Rc::new(RefCell::new(FruitStorage::new()));
    {
        let mut storage_mut = storage_ref.borrow_mut();
        storage_mut.insert(TEST_NAME_1.to_string(), TEST_VALUE_1);
        storage_mut.insert(TEST_NAME_2.to_string(), TEST_VALUE_2);
        storage_mut.insert(TEST_NAME_3.to_string(), TEST_VALUE_3);
    }
    let expected = format!(
        "{TEST_RESULT_2}\n{TEST_RESULT_1}\n{TEST_RESULT_3}\n"
    );
    let actual = create_report(storage_ref.clone());
    assert_eq!(expected, actual);
}

#[test]
fn report_get_correct_report_from_one_line() {
    let storage_ref = Rc::new(RefCell::new(FruitStorage::new()));
    {
        let mut storage_mut = storage_ref.borrow_mut();
        storage_mut.insert(TEST_NAME_1.to_string(), TEST_VALUE_1);
    }
    let expected = format!("{TEST_RESULT_1}\n");
    let actual = create_report(storage_ref.clone());
    assert_eq!(expected, actual)
}
