use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::strategy::{FruitTransaction, Operation};

const TRANSACTION_FORMAT_OPERATION_INDEX: usize = 0;
const TRANSACTION_FORMAT_NAME_INDEX: usize = 1;
const TRANSACTION_FORMAT_VALUE_INDEX: usize = 2;
const CSV_FORMAT_SEPARATOR: &str = ",";

pub fn parse(strings: &Vec<String>) -> Result<Vec<FruitTransaction>, ParseError> {
    strings.iter()
        .map(parse_line)
        .collect()
}

fn parse_line(transaction_line: &String) -> Result<FruitTransaction, ParseError> {
    let split: Vec<&str> = transaction_line
        .split(CSV_FORMAT_SEPARATOR)
        .collect();

    let result = FruitTransaction {
        operation: Operation::try_from(split[TRANSACTION_FORMAT_OPERATION_INDEX])?,
        fruit_name: split[TRANSACTION_FORMAT_NAME_INDEX].to_string(),
        value_of_fruit: split[TRANSACTION_FORMAT_VALUE_INDEX].parse::<i32>()
            .map_err(|e| ParseError::IntParseError)?,
    };
    Ok(result)
}

#[derive(Debug, Clone)]
pub enum ParseError {
    OperationParseError,
    IntParseError
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl Error for ParseError {}