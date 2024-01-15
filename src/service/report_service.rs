use crate::FruitStorage;
use std::cell::RefCell;
use std::rc::Rc;

const CSV_SEPARATOR: &str = ",";

pub fn create_report(storage: Rc<RefCell<FruitStorage>>) -> String {
    let mut buffer = String::new();
    get_sorted_vector_of_strings_from_storage(storage)
        .iter()
        .for_each(|s| buffer.push_str(s.as_str()));
    buffer
}

fn get_sorted_vector_of_strings_from_storage(storage: Rc<RefCell<FruitStorage>>) -> Vec<String> {
    let mut vector: Vec<String> = Vec::new();
    storage.borrow().iter().for_each(|(key, value)|
            // buffer.push_str(get_line_from_storage(key, value).as_str())
            vector.push(get_line_from_storage(key, value)));
    vector.sort();
    vector
}

fn get_line_from_storage(key: &String, value: &i32) -> String {
    format!("{key}{CSV_SEPARATOR}{value}\n")
}
