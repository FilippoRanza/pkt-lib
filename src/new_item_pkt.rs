use crate::{check_len, Result};

const PKT_SIZE: usize = 8;

#[derive(Debug, PartialEq)]
pub struct NewItemInfo {
    pub id: u32,
    pub location: u32,
}

pub fn make_new_item_pkt(info: NewItemInfo) -> [u8; PKT_SIZE] {
    let mut buff = [0; PKT_SIZE];
    let id = info.id.to_be_bytes();
    let loc = info.location.to_be_bytes();
    buff[..4].clone_from_slice(&id);
    buff[4..8].clone_from_slice(&loc);
    buff
}

pub fn parse_new_item_pkt(buff: &[u8]) -> Result<NewItemInfo> {
    check_len(buff, PKT_SIZE)?;
    let mut id = [0; 4];
    let mut loc = [0; 4];

    id.clone_from_slice(&buff[..4]);
    loc.clone_from_slice(&buff[4..]);

    let id = u32::from_be_bytes(id);
    let location = u32::from_be_bytes(loc);
    Ok(NewItemInfo { id, location })
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::ParseError;

    #[test]
    fn test_wrong_size_pkt() {
        let pkt: Vec<u8> = (0..10).collect();
        let res = parse_new_item_pkt(&pkt);
        assert_eq!(
            res,
            Err(ParseError::WrongLen {
                expect: PKT_SIZE,
                got: pkt.len()
            })
        );
    }

    #[quickcheck]
    fn test_conversion(id: u32, location: u32) -> bool {
        let info = NewItemInfo { id, location };
        let result = parse_new_item_pkt(&make_new_item_pkt(info)).unwrap();
        result.id == id && result.location == location
    }
}
