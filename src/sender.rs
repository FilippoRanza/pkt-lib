use crate::{IntoData, Packet};

use std::io::Write;
use std::net::{SocketAddr, TcpStream};

pub fn send_to<D: IntoData<T>, T: Packet>(addr: SocketAddr, data: &D) -> std::io::Result<()> {
    let mut pkt = IntoData::into_packet(data);

    let mut client = TcpStream::connect(addr)?;
    client.write_all(pkt.as_mut())?;

    Ok(())
}
