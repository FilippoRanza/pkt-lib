use crate::{packet_defs, IntoData, Result};

pub type PickUpItemPkt = packet_defs::FourBytePkt;

#[derive(Debug)]
pub struct PickUpItem {
    pub id: u32,
}
impl IntoData<PickUpItemPkt> for PickUpItem {
    fn into_data(p: PickUpItemPkt) -> Result<Self> {
        let data = parse_pick_up_item_pkt(p);
        Ok(data)
    }

    fn into_packet(self) -> PickUpItemPkt {
        make_pick_up_item_pkt(self)
    }
}

pub fn make_pick_up_item_pkt(item: PickUpItem) -> PickUpItemPkt {
    item.id.to_be_bytes()
}

pub fn parse_pick_up_item_pkt(pkt: PickUpItemPkt) -> PickUpItem {
    let id = u32::from_be_bytes(pkt);
    PickUpItem { id }
}

#[cfg(test)]
mod test {

    use super::*;

    #[quickcheck]
    fn check_take_item_conversion(curr_id: u32) -> bool {
        let pick_up = PickUpItem { id: curr_id };
        let pkt = make_pick_up_item_pkt(pick_up);
        let result = parse_pick_up_item_pkt(pkt);
        result.id == curr_id
    }
}
