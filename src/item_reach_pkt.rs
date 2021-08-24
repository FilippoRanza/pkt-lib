use crate::{ParseError, Result};

const PKT_SIZE: usize = 5;
pub type ItemReachPkt = [u8; PKT_SIZE];

#[derive(Debug, PartialEq)]
pub enum ItemStatus {
    InReach(u32),
    OutReach(u32),
}

pub fn make_reach_pkt(reach: ItemStatus) -> ItemReachPkt {
    let (b, v) = reach.into_byte();
    make_buffer(b, v)
}

pub fn parse_reach_pkt(pkt: &ItemReachPkt) -> Result<ItemStatus> {
    let byte = pkt[0];
    let mut value = [0; 4];
    value.clone_from_slice(&pkt[1..]);
    let value = u32::from_be_bytes(value);
    ItemStatus::from_byte(byte, value)
}

const IN_REACH: u8 = 0;
const OUT_REACH: u8 = 1;

fn make_buffer(byte: u8, value: u32) -> ItemReachPkt {
    let mut out = ItemReachPkt::default();
    out[0] = byte;
    let value = value.to_be_bytes();
    out[1..].clone_from_slice(&value);
    out
}

impl ItemStatus {
    fn from_byte(byte: u8, value: u32) -> Result<Self> {
        match byte {
            IN_REACH => Ok(Self::InReach(value)),
            OUT_REACH => Ok(Self::OutReach(value)),
            other => Err(ParseError::unknown_byte(other)),
        }
    }

    fn into_byte(self) -> (u8, u32) {
        match self {
            Self::InReach(value) => (IN_REACH, value),
            Self::OutReach(value) => (OUT_REACH, value),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_unknown_byte() {
        let pkt = [2, 0, 0, 0, 0];
        let result = parse_reach_pkt(&pkt);
        assert_eq!(result, Err(ParseError::Unknown { value: 2 }));
    }

    #[quickcheck]
    fn test_conversion(in_reach: bool, value: u32) -> bool {
        let input = make_item_status(in_reach, value);
        let result = parse_reach_pkt(&make_reach_pkt(input)).unwrap();
        let expect = make_item_status(in_reach, value);
        result == expect
    }

    fn make_item_status(in_reach: bool, value: u32) -> ItemStatus {
        if in_reach {
            ItemStatus::InReach(value)
        } else {
            ItemStatus::OutReach(value)
        }
    }
}
