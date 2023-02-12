use byte_packet_buffer::BytePacketBuffer;
use model::{dns_packet::DnsPacket, dns_question::DnsQuestion, query_type::QueryType};
use std::net::UdpSocket;
mod byte_packet_buffer;
mod model;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let qname = "yahoo.com";
    // let qtype = QueryType::MX;

    // A query for google.com
    let qname = "google.com";
    let qtype = QueryType::A;

    // Using google's public DNS server
    // ref: https://zh.wikipedia.org/zh-tw/Google_Public_DNS
    let server = ("8.8.8.8", 53);

    // Bind a UDP socket to an arbitrary port
    let socket = UdpSocket::bind(("0.0.0.0", 43210))?;

    // Build our query packet. It's important that we remember to set the
    // `recursion_desired` flag. As noted earlier, the packet id is arbitrary.
    let mut packet = DnsPacket::new();

    packet.header.id = 1234;
    packet.header.questions_count = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer)?;

    // Send it off to the server by using our socket
    socket.send_to(&req_buffer.buf[0..req_buffer.pos], server)?;

    let mut res_buffer = BytePacketBuffer::new();

    // Ask the socket to write the response directly to our res_buffer
    socket.recv(&mut res_buffer.buf)?;

    let res_packet = DnsPacket::from_buffer(&mut res_buffer)?;
    println!("{:#?}", res_packet.header);

    for q in res_packet.questions {
        println!("{:#?}", q);
    }

    for rec in res_packet.answers {
        println!("{:#?}", rec);
    }

    for rec in res_packet.authorities {
        println!("{:#?}", rec);
    }

    for rec in res_packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
