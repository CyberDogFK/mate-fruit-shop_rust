use fruit_shop_rust::service::parse;
use fruit_shop_rust::strategy::{FruitTransaction, Operation};

const TEST_NAME_1: &str = "banana";
const TEST_NAME_2: &str = "apple";
const CORRECT_LINE: &str = "b,banana,20";

fn get_correct_fruit_transaction() -> FruitTransaction {
    get_fruit_transaction(Operation::BALANCE, TEST_NAME_1.to_string(), 20)
}

fn get_fruit_transactions_one_line_list() -> Vec<FruitTransaction> {
    vec![get_correct_fruit_transaction()]
}

fn get_correct_example_list() -> Vec<String> {
    vec![
        format!("b,{TEST_NAME_2},100"),
        format!("s,{TEST_NAME_1},100"),
        format!("p,{TEST_NAME_1},13"),
        format!("r,{TEST_NAME_2},10"),
        format!("p,{TEST_NAME_2},20"),
        format!("p,{TEST_NAME_1},5"),
        format!("s,{TEST_NAME_1},50"),
    ]
}

fn get_correct_fruit_transaction_list() -> Vec<FruitTransaction> {
    vec![
        get_fruit_transaction(Operation::BALANCE, TEST_NAME_2.to_string(), 100),
        get_fruit_transaction(Operation::SUPPLY, TEST_NAME_1.to_string(), 100),
        get_fruit_transaction(Operation::PURCHASE, TEST_NAME_1.to_string(), 13),
        get_fruit_transaction(Operation::RETURN, TEST_NAME_2.to_string(), 10),
        get_fruit_transaction(Operation::PURCHASE, TEST_NAME_2.to_string(), 20),
        get_fruit_transaction(Operation::PURCHASE, TEST_NAME_1.to_string(), 5),
        get_fruit_transaction(Operation::SUPPLY, TEST_NAME_1.to_string(), 50),
    ]
}

#[test]
fn parse_correct_list() {
    let actual = parse(&get_correct_example_list()).unwrap();
    let expected = get_correct_fruit_transaction_list();
    assert!(actual.iter().all(|t| expected.contains(t)))
}

#[test]
fn parse_correct_string() {
    let actual = parse(&[CORRECT_LINE.to_string()]).unwrap();
    let expect = get_fruit_transactions_one_line_list();
    assert!(actual.iter().all(|t| expect.contains(t)))
}

fn get_fruit_transaction(
    operation: Operation,
    fruit_name: String,
    value_of_fruit: i32,
) -> FruitTransaction {
    FruitTransaction {
        operation,
        fruit_name,
        value_of_fruit,
    }
}
