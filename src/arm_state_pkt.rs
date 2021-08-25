use crate::{ParseError, Result};

const PKT_SIZE: usize = 5;
pub type ArmStatePkt = [u8; PKT_SIZE];

#[derive(Debug, PartialEq)]
pub enum ArmState {
    Ready,
    Working(u32),
}

pub fn make_arm_state_pkt(state: &ArmState) -> ArmStatePkt {
    let mut pkt = ArmStatePkt::default();
    match state {
        ArmState::Ready => pkt[0] = READY,
        ArmState::Working(value) => working_into_pkt(*value, &mut pkt),
    }
    pkt
}

pub fn parse_arm_state_pkt(pkt: &ArmStatePkt) -> Result<ArmState> {
    match pkt[0] {
        READY => Ok(ArmState::Ready),
        WORKING => Ok(pkt_into_working(pkt)),
        other => Err(ParseError::unknown_byte(other)),
    }
}

const READY: u8 = 0;
const WORKING: u8 = 1;

fn working_into_pkt(value: u32, pkt: &mut ArmStatePkt) {
    let value = value.to_be_bytes();
    pkt[1..].clone_from_slice(&value);
    pkt[0] = WORKING;
}

fn pkt_into_working(pkt: &ArmStatePkt) -> ArmState {
    let mut buff = [0; 4];
    buff.clone_from_slice(&pkt[1..]);
    let val = u32::from_be_bytes(buff);
    ArmState::Working(val)
}

#[cfg(test)]
mod test {

    use super::*;

    #[quickcheck]
    fn test_arm_state_conversion(value: Option<u32>) -> bool {
        let correct = into_arm_state(value);
        let pkt = make_arm_state_pkt(&correct);
        let result = parse_arm_state_pkt(&pkt).unwrap();
        result == correct
    }

    fn into_arm_state(opt: Option<u32>) -> ArmState {
        opt.map(|v| ArmState::Working(v)).unwrap_or(ArmState::Ready)
    }
}
