use std::net::TcpStream;

pub use edlib::ed::*;

fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:34254")?;
    let mut connection = protocol::wire::stream::Connection::new(
        stream,
        protocol::wire::middleware::pipeline::default(),
        protocol::Settings::default(),
    );

    let packet = PacketGenerator::get_version();

    connection
        .send_packet(&packet)
        .expect("Error sending packet");

    let packet = PacketGenerator::get_info();

    connection
        .send_packet(&packet)
        .expect("Error sending packet");

    // let packet = PacketGenerator::write_register();

    // connection
    //     .send_packet(&packet)
    //     .expect("Error sending packet");

    loop {
        if let Some(response) = connection.receive_packet().unwrap() {
            println!("{:?}", response);
            break;
        }
    }

    Ok(())
}
