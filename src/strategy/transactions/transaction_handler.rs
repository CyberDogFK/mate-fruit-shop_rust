use crate::strategy::FruitTransaction;
use crate::FruitStorage;
use std::cell::RefCell;
use std::rc::Rc;

pub trait TransactionHandler {
    fn apply(&mut self, fruit_transaction: &FruitTransaction);
}

#[derive(PartialEq)]
pub struct AdderHandler(Rc<RefCell<FruitStorage>>);

impl TransactionHandler for AdderHandler {
    fn apply(&mut self, fruit_transaction: &FruitTransaction) {
        let mut mut_ref = self.0.borrow_mut();
        let old_value = mut_ref[&fruit_transaction.fruit_name];
        let new_value = old_value + fruit_transaction.value_of_fruit;
        mut_ref.insert(fruit_transaction.fruit_name.clone(), new_value);
    }
}

#[derive(PartialEq)]
pub struct ReduceHandler(Rc<RefCell<FruitStorage>>);

impl TransactionHandler for ReduceHandler {
    fn apply(&mut self, fruit_transaction: &FruitTransaction) {
        let mut mut_ref = self.0.borrow_mut();
        let old_value = mut_ref[&fruit_transaction.fruit_name];
        let new_value = old_value - fruit_transaction.value_of_fruit;
        if new_value < 0 {
            panic!("You put wrong data. Value is storage can't be less 0")
        }
        mut_ref.insert(fruit_transaction.fruit_name.clone(), new_value);
    }
}

#[derive(PartialEq)]
pub struct SavedHandler(Rc<RefCell<FruitStorage>>);

impl TransactionHandler for SavedHandler {
    fn apply(&mut self, fruit_transaction: &FruitTransaction) {
        self.0.borrow_mut().insert(
            fruit_transaction.fruit_name.clone(),
            fruit_transaction.value_of_fruit,
        );
    }
}

macro_rules! new_boxed_handler {
    ($($i:ty),*) => {
        $(impl $i {
            pub fn new_boxed(fruit_storage: Rc<RefCell<FruitStorage>>) -> Box<Self> {
                Box::new(Self(fruit_storage))
            }
        })*
    };
}

new_boxed_handler!(SavedHandler, AdderHandler, ReduceHandler);
