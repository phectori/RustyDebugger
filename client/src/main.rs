use lognplot::TcpClient;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::SystemTime;

pub use edlib::packet::PacketGenerator;
pub use edlib::protocol::Protocol;

fn main() -> std::io::Result<()> {
    let ip = std::net::Ipv6Addr::LOCALHOST;
    let addr = std::net::SocketAddrV6::new(ip, 34254, 0, 0);
    let mut stream = TcpStream::connect(addr)?;

    let mut pg = PacketGenerator::default();
    let p = pg.get_version();
    stream.write(&p).unwrap();

    let mut protocol = Protocol::default();
    let mut buffer = [0; 10];

    let mut lnp_client = TcpClient::new("[::1]:12345").unwrap();

    loop {
        stream.read(&mut buffer)?;
        if let Some(packet) = protocol.process_data(buffer.to_vec()) {
            protocol.process_packet(packet);
            stream.write(&protocol.take_response()).unwrap();
        }
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let timestamp = (now.as_millis() as f64) / 1000.0;
        lnp_client.send_sample("Test", timestamp, 10.0)?;
    }

    // stream
    //     .shutdown(Shutdown::Both)
    //     .expect("shutdown call failed");

    // Ok(())
}
