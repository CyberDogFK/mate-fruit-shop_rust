use crate::strategy::operation::Operation;

#[derive(PartialEq)]
pub struct FruitTransaction {
    pub operation: Operation,
    pub fruit_name: String,
    pub value_of_fruit: i32,
}

