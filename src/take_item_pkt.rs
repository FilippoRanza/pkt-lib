const PKT_SIZE: usize = 4;
pub type TakeItemPkt = [u8; PKT_SIZE];

pub fn make_take_item_pkt(item_id: u32) -> TakeItemPkt {
    item_id.to_be_bytes()
}

pub fn parse_take_item_pkt(pkt: &TakeItemPkt) -> u32 {
    u32::from_be_bytes(*pkt)
}

#[cfg(test)]
mod test {

    use super::*;

    #[quickcheck]
    fn test_take_item_conversion(id: u32) -> bool {
        let pkt = make_take_item_pkt(id);
        let res = parse_take_item_pkt(&pkt);
        res == id
    }
}
