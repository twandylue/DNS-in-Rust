use byte_packet_buffer::BytePacketBuffer;
use dns_packet::DnsPacket;
use std::{fs::File, io::Read};
mod byte_packet_buffer;
mod dns_header;
mod dns_packet;
mod dns_question;
mod dns_record;
mod query_type;
mod result_code;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO:
    let mut f = File::open("./docs/response_packet.txt")?;
    let mut buffer = BytePacketBuffer::new();
    f.read(&mut buffer.buf)?;

    let packet = DnsPacket::from_buffer(&mut buffer);
    println!("{:#?}", packet);

    Ok(())
}
