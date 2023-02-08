use super::super::BytePacketBuffer;
use super::query_type::QueryType;
use super::{dns_header::DnsHeader, dns_question::DnsQuestion, dns_record::DnsRecord};

#[derive(Clone, Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn new() -> Self {
        DnsPacket {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            resources: Vec::new(),
        }
    }

    pub fn from_buffer(
        buffer: &mut BytePacketBuffer,
    ) -> Result<DnsPacket, Box<dyn std::error::Error>> {
        let mut result = DnsPacket::new();
        result.header.read(buffer)?;

        for _ in 0..result.header.questions_count {
            let mut question = DnsQuestion::new("".to_string(), QueryType::UNKNOWN(0));
            question.read(buffer)?;
            result.questions.push(question);
        }

        for _ in 0..result.header.answers_count {
            let answer = DnsRecord::read(buffer)?;
            result.answers.push(answer);
        }

        for _ in 0..result.header.authority_count {
            let auth = DnsRecord::read(buffer)?;
            result.authorities.push(auth);
        }

        for _ in 0..result.header.additional_count {
            let resource = DnsRecord::read(buffer)?;
            result.resources.push(resource);
        }

        Ok(result)
    }
}
