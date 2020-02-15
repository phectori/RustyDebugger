use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

pub use edlib::packet::PacketGenerator;
pub use edlib::protocol::Protocol;

fn main() -> std::io::Result<()> {
    let ip = std::net::Ipv6Addr::UNSPECIFIED;
    let addr = std::net::SocketAddrV6::new(ip, 34254, 0, 0);
    let mut stream = TcpStream::connect(addr)?;

    let packet = PacketGenerator::serialize(PacketGenerator::get_version());

    stream.write(&packet)?;

    let mut protocol = Protocol::default();
    let mut buffer = [0; 10];
    loop {
        stream.read(&mut buffer)?;
        if let Some(packet) = protocol.process_data(buffer.to_vec()) {
            protocol.process_packet(packet);
            stream.write(&protocol.take_response()).unwrap();
        }
    }

    // stream
    //     .shutdown(Shutdown::Both)
    //     .expect("shutdown call failed");

    // Ok(())
}
