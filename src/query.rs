use super::byte_packet_buffer::BytePacketBuffer;
use super::model::{
    dns_packet::DnsPacket, dns_question::DnsQuestion, query_type::QueryType,
    result_code::ResultCode,
};
use std::net::UdpSocket;

pub fn lookup(qname: &str, qtype: QueryType) -> Result<DnsPacket, Box<dyn std::error::Error>> {
    // Forward queries to Google's public DNS
    let server = ("8.8.8.8", 53);

    let socket = UdpSocket::bind(("0.0.0.0", 43210))?;

    let mut packet = DnsPacket::new();

    packet.header.id = 1234;
    packet.header.questions_count = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer)?;
    socket.send_to(&req_buffer.buf[0..req_buffer.pos()], server)?;

    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf)?;

    Ok(DnsPacket::from_buffer(&mut res_buffer)?)
}

/// Handle a single incoming packet
pub fn handle_query(socket: &UdpSocket) -> Result<(), Box<dyn std::error::Error>> {
    let mut req_buffer = BytePacketBuffer::new();
    let (_, src) = socket.recv_from(&mut req_buffer.buf)?;
    let mut request = DnsPacket::from_buffer(&mut req_buffer)?;

    let mut res_packet = DnsPacket::new();
    res_packet.header.id = request.header.id;
    res_packet.header.recursion_desired = true;
    res_packet.header.recursion_available = true;
    res_packet.header.query_response = true;

    if let Some(question) = request.questions.pop() {
        println!("Received query: {:?}", question);

        if let Ok(result) = lookup(&question.name, question.qtype) {
            res_packet.questions.push(question);
            res_packet.header.response_code = result.header.response_code;

            for rec in result.answers {
                println!("Answer: {:?}", rec);
                res_packet.answers.push(rec);
            }
            for rec in result.authorities {
                println!("Authority: {:?}", rec);
                res_packet.authorities.push(rec);
            }
            for rec in result.resources {
                println!("Resource(Additional Resource): {:?}", rec);
                res_packet.resources.push(rec);
            }
        } else {
            res_packet.header.response_code = ResultCode::SERVFAIL;
        }
    } else {
        res_packet.header.response_code = ResultCode::FORMERR;
    }

    let mut res_buffer = BytePacketBuffer::new();
    res_packet.write(&mut res_buffer)?;

    let len = res_buffer.pos();
    let data = res_buffer.get_range(0, len)?;

    socket.send_to(data, src)?;

    Ok(())
}
