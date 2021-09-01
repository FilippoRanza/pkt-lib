use crate::{IntoData, Packet, Result};

const PKT_SIZE: usize = 4;
pub type TakeItemPkt = [u8; PKT_SIZE];
impl Packet for TakeItemPkt {}

#[derive(Debug, PartialEq)]
pub struct TakeItem {
    pub id: u32,
}

impl IntoData<TakeItemPkt> for TakeItem {
    fn into_data(p: TakeItemPkt) -> Result<Self> {
        let this = parse_take_item_pkt(p);
        Ok(this)
    }

    fn into_packet(self) -> TakeItemPkt {
        make_take_item_pkt(self)
    }
}

pub fn make_take_item_pkt(take_item: TakeItem) -> TakeItemPkt {
    take_item.id.to_be_bytes()
}

pub fn parse_take_item_pkt(pkt: TakeItemPkt) -> TakeItem {
    let id = u32::from_be_bytes(pkt);
    TakeItem { id }
}

#[cfg(test)]
mod test {

    use super::*;

    #[quickcheck]
    fn test_take_item_conversion(id: u32) -> bool {
        let take_item = TakeItem { id };
        let pkt = make_take_item_pkt(take_item);
        let res = parse_take_item_pkt(pkt);
        res.id == id
    }
}
