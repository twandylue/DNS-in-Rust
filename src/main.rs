use byte_packet_buffer::BytePacketBuffer;
use query::handle_query;
use std::net::UdpSocket;

mod byte_packet_buffer;
mod model;
mod query;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind an UDP socket on port 2054
    let socket = UdpSocket::bind(("0.0.0.0", 2054))?;

    // Queries handled sequentially
    loop {
        match handle_query(&socket) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("An error occurred: {}", e)
            }
        }
    }
}
