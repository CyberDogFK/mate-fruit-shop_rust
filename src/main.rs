use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use fruit_shop_rust::db::FruitStorage;
use fruit_shop_rust::service::{create_report, csv_reader, CsvWriter, parse};
use fruit_shop_rust::service;
use fruit_shop_rust::strategy::{AdderHandler, FruitTransaction, Operation, ReduceHandler, SavedHandler, TransactionHandler};
use fruit_shop_rust::strategy::Operation::{BALANCE, PURCHASE, RETURN, SUPPLY};

const NUMBER_OF_TITLE_IN_CSV: usize = 1;
const EXAMPLE_FILE: &str = "example.csv";
const TITLE_FOR_CSV: &str = "fruit,quantity";
const RESULT_TARGET_FILE: &str = "sms.csv";

fn main() {
    let fruit_storage = Rc::new(RefCell::new(FruitStorage::new()));
    let strings = csv_reader::read_file_with_skip_lines(EXAMPLE_FILE, NUMBER_OF_TITLE_IN_CSV)
        .unwrap();
    let parsed_transactions = parse(&strings).unwrap();

    apply_transactions(parsed_transactions, fruit_storage.clone());

    let report = create_report(fruit_storage.clone());
    let csv_writer = CsvWriter::new(Some(TITLE_FOR_CSV.to_string()));
    csv_writer.save_to_file(RESULT_TARGET_FILE, report).unwrap();
}

fn apply_transactions(transactions: Vec<FruitTransaction>, fruit_storage: Rc<RefCell<FruitStorage>>) {
    let mut map_transaction = map_transactions(fruit_storage);
    transactions.iter()
        .for_each(|t| {
            if let Some(s) = map_transaction.get_mut(&t.operation) {
                s.apply(t)
            }
        })
}

fn map_transactions(fruit_storage: Rc<RefCell<FruitStorage>>) -> HashMap<Operation, Box<dyn TransactionHandler>> {
    let mut transaction_map: HashMap<Operation, Box<dyn TransactionHandler>> = HashMap::new();
    transaction_map.insert(BALANCE, SavedHandler::new_boxed(fruit_storage.clone()));
    transaction_map.insert(SUPPLY, AdderHandler::new_boxed(fruit_storage.clone()));
    transaction_map.insert(PURCHASE, ReduceHandler::new_boxed(fruit_storage.clone()));
    transaction_map.insert(RETURN, AdderHandler::new_boxed(fruit_storage.clone()));
    transaction_map
}
