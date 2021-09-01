use crate::Packet;

macro_rules! define_pkt {
    ($name: ident, $size: expr) => {
        pub type $name = [u8; $size];
        impl Packet for $name {}
    };
}

define_pkt! {FourBytePkt, 4}
