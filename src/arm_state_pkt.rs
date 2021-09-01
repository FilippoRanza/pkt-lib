use crate::{insert_bytes, BuffConverter, IntoData, Packet, ParseError, Result};

const PKT_SIZE: usize = 1 + 4 + 4;
pub type ArmStatePkt = [u8; PKT_SIZE];
impl Packet for ArmStatePkt {}

#[derive(Debug, PartialEq)]
pub struct ArmInfo {
    pub arm_id: u32,
    pub status: ArmState,
}

impl IntoData<ArmStatePkt> for ArmInfo {
    fn into_data(p: ArmStatePkt) -> Result<Self> {
        parse_arm_state_pkt(p)
    }

    fn into_packet(self) -> ArmStatePkt {
        make_arm_state_pkt(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum ArmState {
    Ready,
    Waiting,
    Working(u32),
}

pub fn make_arm_state_pkt(state: ArmInfo) -> ArmStatePkt {
    let mut pkt = ArmStatePkt::default();
    match state.status {
        ArmState::Ready => ready_into_pkt(state.arm_id, &mut pkt),
        ArmState::Waiting => waiting_into_pkt(state.arm_id, &mut pkt),
        ArmState::Working(value) => working_into_pkt(value, state.arm_id, &mut pkt),
    }
    pkt
}

pub fn parse_arm_state_pkt(pkt: ArmStatePkt) -> Result<ArmInfo> {
    let mut converter = BuffConverter::new(&pkt[1..]);
    let working_val = converter.get_next_u32().unwrap();
    let arm_id = converter.get_next_u32().unwrap();
    let status = parse_arm_status(pkt[0], working_val)?;
    Ok(ArmInfo { arm_id, status })
}

fn parse_arm_status(first: u8, working_val: u32) -> Result<ArmState> {
    match first {
        READY => Ok(ArmState::Ready),
        WORKING => Ok(ArmState::Working(working_val)),
        WAITING => Ok(ArmState::Waiting),
        other => Err(ParseError::unknown_byte(other)),
    }
}

const READY: u8 = 0;
const WORKING: u8 = 1;
const WAITING: u8 = 2;

fn working_into_pkt(value: u32, index: u32, pkt: &mut ArmStatePkt) {
    build_packet(WORKING, value, index, pkt);
}

fn waiting_into_pkt(index: u32, pkt: &mut ArmStatePkt) {
    build_packet(WAITING, 0, index, pkt);
}

fn ready_into_pkt(index: u32, pkt: &mut ArmStatePkt) {
    build_packet(READY, 0, index, pkt);
}

fn build_packet(kind: u8, value: u32, index: u32, pkt: &mut ArmStatePkt) {
    pkt[0] = kind;
    insert_bytes(&mut pkt[1..], &[value, index]);
}

#[cfg(test)]
mod test {

    use super::*;

    #[quickcheck]
    fn test_arm_state_conversion(waiting: bool, value: Option<u32>, arm_id: u32) -> bool {
        let correct = into_arm_info(waiting, value, arm_id);
        let pkt = make_arm_state_pkt(correct);
        let result = parse_arm_state_pkt(pkt).unwrap();
        let expect = into_arm_info(waiting, value, arm_id);
        result == expect
    }

    fn into_arm_info(waiting: bool, opt: Option<u32>, arm_id: u32) -> ArmInfo {
        let default = if waiting {
            ArmState::Waiting
        } else {
            ArmState::Ready
        };
        let status = opt.map(|v| ArmState::Working(v)).unwrap_or(default);
        ArmInfo { arm_id, status }
    }
}
