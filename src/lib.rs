#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    WrongLen { expected: usize, actual: usize },
    Unknown { value: u8 },
}

impl ParseError {
    fn unknown_byte(byte: u8) -> Self {
        Self::Unknown { value: byte }
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;

mod item_reach_pkt;
pub use item_reach_pkt::{make_reach_pkt, parse_reach_pkt, ItemReachPkt, ItemStatus};

mod new_item_pkt;
pub use new_item_pkt::{make_new_item_pkt, parse_new_item_pkt, NewItemInfo, NewItemPkt};