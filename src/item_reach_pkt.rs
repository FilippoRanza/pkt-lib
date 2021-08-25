use crate::{insert_bytes, BuffConverter, ParseError, Result};

const PKT_SIZE: usize = 3 * 4 + 1;
pub type ItemReachPkt = [u8; PKT_SIZE];

#[derive(Debug, PartialEq)]
pub enum ItemStatus {
    InReach(ItemInfo),
    OutReach(ItemInfo),
}

#[derive(Debug, PartialEq)]
pub struct ItemInfo {
    pub index: u32,
    pub pos_x: u32,
    pub pos_y: u32,
}

pub fn make_reach_pkt(reach: ItemStatus) -> ItemReachPkt {
    let (b, i, x, y) = reach.into_byte();
    make_buffer(b, i, x, y)
}

pub fn parse_reach_pkt(pkt: &ItemReachPkt) -> Result<ItemStatus> {
    let byte = pkt[0];

    let mut convert = BuffConverter::new(&pkt[1..]);
    let index = convert.get_next_u32().unwrap();
    let pos_x = convert.get_next_u32().unwrap();
    let pos_y = convert.get_next_u32().unwrap();
    ItemStatus::from_byte(byte, index, pos_x, pos_y)
}

const IN_REACH: u8 = 0;
const OUT_REACH: u8 = 1;

fn make_buffer(byte: u8, index: u32, pos_x: u32, pos_y: u32) -> ItemReachPkt {
    let mut out = ItemReachPkt::default();
    out[0] = byte;
    insert_bytes(&mut out[1..], &[index, pos_x, pos_y]);
    out
}

impl ItemStatus {
    fn from_byte(byte: u8, index: u32, pos_x: u32, pos_y: u32) -> Result<Self> {
        let info = ItemInfo {
            index,
            pos_x,
            pos_y,
        };
        match byte {
            IN_REACH => Ok(Self::InReach(info)),
            OUT_REACH => Ok(Self::OutReach(info)),
            other => Err(ParseError::unknown_byte(other)),
        }
    }

    fn into_byte(self) -> (u8, u32, u32, u32) {
        match self {
            Self::InReach(ItemInfo {
                index,
                pos_x,
                pos_y,
            }) => (IN_REACH, index, pos_x, pos_y),
            Self::OutReach(ItemInfo {
                index,
                pos_x,
                pos_y,
            }) => (OUT_REACH, index, pos_x, pos_y),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_unknown_byte() {
        let mut pkt = ItemReachPkt::default();
        pkt[0] = 2;
        let result = parse_reach_pkt(&pkt);
        assert_eq!(result, Err(ParseError::Unknown { value: 2 }));
    }

    #[quickcheck]
    fn test_conversion(in_reach: bool, index: u32, pos_x: u32, pos_y: u32) -> bool {
        let input = make_item_status(in_reach, index, pos_x, pos_y);
        let result = parse_reach_pkt(&make_reach_pkt(input)).unwrap();
        let expect = make_item_status(in_reach, index, pos_x, pos_y);
        result == expect
    }

    fn make_item_status(in_reach: bool, index: u32, pos_x: u32, pos_y: u32) -> ItemStatus {
        let info = ItemInfo {
            index,
            pos_x,
            pos_y,
        };
        if in_reach {
            ItemStatus::InReach(info)
        } else {
            ItemStatus::OutReach(info)
        }
    }
}
