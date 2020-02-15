pub use edlib::packet::*;
pub use edlib::protocol::*;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut p = Protocol::default();

    let mut data = [0 as u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            print_packet(&mut data, size);
            if let Some(packet) = p.process_data(data[0..size].to_vec()) {
                p.process_packet_host(packet);
                stream.write(&p.take_response()).unwrap();
            }
            true
        }
        Err(_) => {
            println!("Error {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn print_packet(data: &mut [u8], size: usize) {

    for i in 0..size {
        match data[i] {
            ETX => {
                print!("0x{:x?}\n", data[i]);
            }
            _ => {
                print!("0x{:x?} ", data[i]);
            }
        }
        std::io::stdout().flush().unwrap();
    }
}

fn main() {
    let ip = std::net::Ipv6Addr::UNSPECIFIED;
    let addr = std::net::SocketAddrV6::new(ip, 34254, 0, 0);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
