use byte_packet_buffer::BytePacketBuffer;
use std::{fs::File, io::Read};
mod byte_packet_buffer;
mod model;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO:
    let mut f = File::open("./docs/response_packet.txt")?;
    let mut buffer = BytePacketBuffer::new();
    f.read(&mut buffer.buf)?;

    let packet = model::dns_packet::DnsPacket::from_buffer(&mut buffer);
    println!("{:#?}", packet);

    Ok(())
}
