use std::cell::RefCell;
use std::rc::Rc;
use crate::db::FruitStorage;

const CSV_SEPARATOR: &str = ",";

pub fn create_report(storage: Rc<RefCell<FruitStorage>>) -> String {
    let mut buffer = String::new();
    storage.borrow().iter()
        .for_each(|(key, value)|
            buffer.push_str(get_line_from_storage(key, value).as_str())
        );
    buffer
}

fn get_line_from_storage(key: &String, value: &i32) -> String {
    format!("{key}{CSV_SEPARATOR}{value}\n")
}