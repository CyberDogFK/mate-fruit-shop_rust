use std::convert::TryFrom;
use std::error::Error;
use crate::strategy::operation::Operation::{BALANCE, PURCHASE, RETURN, SUPPLY};

pub enum Operation {
    BALANCE,
    SUPPLY,
    PURCHASE,
    RETURN
}

impl TryFrom<&str> for Operation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "b" => Ok(BALANCE),
            "s" => Ok(SUPPLY),
            "p" => Ok(PURCHASE),
            "r" => Ok(RETURN),
            _ => Err("Wrong operation code".to_string())
        }
    }
}

