use super::byte_packet_buffer::BytePacketBuffer;
use super::model::{
    dns_packet::DnsPacket, dns_question::DnsQuestion, query_type::QueryType,
    result_code::ResultCode,
};
use std::net::{Ipv4Addr, UdpSocket};

pub fn recursive_lookup(
    qname: &str,
    qtype: QueryType,
) -> Result<DnsPacket, Box<dyn std::error::Error>> {
    // For practice, we are always starting with `a.root-servers.net`
    let mut ns = Ipv4Addr::new(198, 41, 0, 4);

    loop {
        println!("attempting lookup of {:?} {} with ns {}", qtype, qname, ns);
        let copy_ns = ns;

        let server = (copy_ns, 53);
        let response = lookup(qname, qtype, server)?;

        // If there are no entries in the answer section, and no errors, it is Ok
        if !response.answers.is_empty() && response.header.response_code == ResultCode::NOERROR {
            return Ok(response);
        }

        // We might also get a `NXDOMAIN` reply, which is the authorities name servers way of
        // telling us that the name doesn't exist.
        if response.header.response_code == ResultCode::NXDOMAIN {
            return Ok(response);
        }

        // Otherwise, we will try to find a new name server based on NS and a corresponding A
        // record in the additional section. If this succeeds, we can switch name server and retry
        // again.
        if let Some(new_ns) = response.get_resolved_ns(qname) {
            ns = new_ns;

            continue;
        }

        // If not, we will have to resolve the ip of a NS record. If no NS records exist, we will
        // go with what the last server told us.
        let new_ns_name = match response.get_unsolved_ns(qname) {
            Some(x) => x,
            None => return Ok(response),
        };

        let recursive_response = recursive_lookup(&new_ns_name, QueryType::A)?;

        // Finally, we pick a random ip from the result, and restart the loop. If no such record is
        // available, we return the last result we got.
        if let Some(new_ns) = recursive_response.get_random_a() {
            ns = new_ns
        } else {
            return Ok(response);
        }
    }
}

/// Query the name through Google's public DNS
pub fn lookup(
    qname: &str,
    qtype: QueryType,
    server: (Ipv4Addr, u16),
) -> Result<DnsPacket, Box<dyn std::error::Error>> {
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

    // While the socket is ready, we can read a packet. This will `block` until one is received.
    let (_, src) = socket.recv_from(&mut req_buffer.buf)?;

    let mut request = DnsPacket::from_buffer(&mut req_buffer)?;

    // Create and initialize the response packet
    let mut res_packet = DnsPacket::new();
    res_packet.header.id = request.header.id;
    res_packet.header.recursion_desired = true;
    res_packet.header.recursion_available = true;
    res_packet.header.query_response = true;

    // In normal case, only one question is present
    if let Some(question) = request.questions.pop() {
        println!("Received query: {:?}", question);

        // The query can be forwarded to the target server.
        // There's always the possibility that the query will
        // fail, in which case the `SERVFAIL` response code is set to indicate
        // as much to the client.
        // If everything goes as planned, the question and response records as copied into our response packet.
        if let Ok(result) = recursive_lookup(&question.name, question.qtype) {
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
    }
    // We have to make sure that a question is actually present.
    // If not, we return `FORMERR` to indicate that the sender made something wrong.
    else {
        res_packet.header.response_code = ResultCode::FORMERR;
    }

    // Transform the response packet to buffer and send it back to our client.
    let mut res_buffer = BytePacketBuffer::new();
    res_packet.write(&mut res_buffer)?;

    // Response to the client.
    socket.send_to(&res_buffer.buf[0..res_buffer.pos()], src)?;

    Ok(())
}
