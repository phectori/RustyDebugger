use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

pub use edlib::ed::*;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    let packet = PacketGenerator::serialize(PacketGenerator::get_version());

    stream.write(&packet)?;
    stream
        .shutdown(Shutdown::Both)
        .expect("shutdown call failed");

    Ok(())
}
