use std::fmt;

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

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown { value } => {
                write!(f, "Unknown byte value. Cannot parse byte: {}.", value)
            }
            Self::WrongLen { expected, actual } => write!(
                f,
                "Input buffer has wrong length. Expected: {} - Actual: {}",
                expected, actual
            ),
        }
    }
}

impl ParseError {
    fn unknown_byte(byte: u8) -> Self {
        Self::Unknown { value: byte }
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;

mod item_reach_pkt;
pub use item_reach_pkt::{make_reach_pkt, parse_reach_pkt, ItemInfo, ItemReachPkt, ItemStatus};

mod new_item_pkt;
pub use new_item_pkt::{make_new_item_pkt, parse_new_item_pkt, NewItemInfo, NewItemPkt};

mod take_item_pkt;
pub use take_item_pkt::{make_take_item_pkt, parse_take_item_pkt, TakeItemPkt};

mod arm_state_pkt;
pub use arm_state_pkt::{make_arm_state_pkt, parse_arm_state_pkt, ArmInfo, ArmState, ArmStatePkt};

#[cfg(feature = "listener")]
pub mod listener;

struct BuffConverter<'a> {
    buff: &'a [u8],
    index: usize,
}

impl<'a> BuffConverter<'a> {
    fn new(buff: &'a [u8]) -> Self {
        Self { buff, index: 0 }
    }

    fn get_next_u32(&mut self) -> Option<u32> {
        if self.buff.len() >= self.index + 4 {
            let mut buff = [0; 4];
            buff.clone_from_slice(&self.buff[self.index..self.index + 4]);
            self.index += 4;
            let value = u32::from_be_bytes(buff);
            Some(value)
        } else {
            None
        }
    }
}

fn insert_bytes(buff: &mut [u8], ints: &[u32]) {
    let mut i = 0;
    for int in ints {
        let val = int.to_be_bytes();
        buff[i..i + 4].clone_from_slice(&val);
        i += 4;
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[quickcheck]
    fn check_u32_to_buff_convertion(a: u32, b: u32, c: u32, d: u32) -> bool {
        let mut buff = [0; 4 * 4];
        insert_bytes(&mut buff, &[a, b, c, d]);
        let mut convert = BuffConverter::new(&buff);
        let res_a = convert.get_next_u32().unwrap();
        let res_b = convert.get_next_u32().unwrap();
        let res_c = convert.get_next_u32().unwrap();
        let res_d = convert.get_next_u32().unwrap();

        res_a == a && res_b == b && res_c == c && res_d == d
    }
}
