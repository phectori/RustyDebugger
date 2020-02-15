use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

pub use edlib::packet::PacketGenerator;

fn main() -> std::io::Result<()> {
    let ip = std::net::Ipv6Addr::UNSPECIFIED;
    let addr = std::net::SocketAddrV6::new(ip, 34254, 0, 0);
    let mut stream = TcpStream::connect(addr)?;

    let packet = PacketGenerator::serialize(PacketGenerator::get_version());

    stream.write(&packet)?;

    // loop {
    //     stream.read(buf: &mut [u8])
    // }

    stream
        .shutdown(Shutdown::Both)
        .expect("shutdown call failed");

    Ok(())
}
