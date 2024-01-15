use crate::service::ParseError;
use crate::strategy::operation::Operation::{BALANCE, PURCHASE, RETURN, SUPPLY};

#[derive(PartialEq, Eq, Hash)]
pub enum Operation {
    BALANCE,
    SUPPLY,
    PURCHASE,
    RETURN,
}

impl TryFrom<&str> for Operation {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "b" => Ok(BALANCE),
            "s" => Ok(SUPPLY),
            "p" => Ok(PURCHASE),
            "r" => Ok(RETURN),
            _ => Err(ParseError::OperationParseError),
        }
    }
}
