use super::super::BytePacketBuffer;
use super::query_type::QueryType;
use super::{dns_header::DnsHeader, dns_question::DnsQuestion, dns_record::DnsRecord};
use std::net::Ipv4Addr;

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

    pub fn write(
        &mut self,
        buffer: &mut BytePacketBuffer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.header.questions_count = self.questions.len() as u16;
        self.header.answers_count = self.answers.len() as u16;
        self.header.authority_count = self.authorities.len() as u16;
        self.header.additional_count = self.resources.len() as u16;

        self.header.write(buffer)?;

        for question in &self.questions {
            question.write(buffer)?;
        }

        for rec in &self.answers {
            rec.write(buffer)?;
        }

        for rec in &self.authorities {
            rec.write(buffer)?;
        }

        for rec in &self.resources {
            rec.write(buffer)?;
        }

        Ok(())
    }

    // If we get multiple IP's for a single name, it doesn't matter which one we choose, so in
    // those cases we can now pick one at random.
    pub fn get_random_a(&self) -> Option<Ipv4Addr> {
        self.answers.iter().find_map(|record| match record {
            DnsRecord::A { addr, .. } => Some(*addr),
            _ => None,
        })
    }

    /// A helper function which return an iterator over all name servers in
    /// the authorities section, represented as (domain, host) tuples.
    fn get_ns<'a>(&'a self, qname: &'a str) -> impl Iterator<Item = (&'a str, &'a str)> {
        self.authorities
            .iter()
            // Convert the NS records to a tuple which has only the data we need to make it easy to
            // work
            .filter_map(|record| match record {
                DnsRecord::NS { domain, host, .. } => Some((domain.as_str(), host.as_str())),
                _ => None,
            })
            // Discard servers which are not authoritaintive to our query
            .filter(|(domain, _)| qname.ends_with(domain))
    }

    /// Name servers often bundle the corresponding A records when replying to an NS query. Thus,
    /// we implement a function that returns the actual IP for an NS record if possible.
    pub fn get_resolved_ns(&self, qname: &str) -> Option<Ipv4Addr> {
        self.get_ns(qname).find_map(|(_, host)| {
            let result = self.resources.iter().find_map(move |record| match record {
                DnsRecord::A { domain, addr, .. } if domain == host => Some(*addr),
                _ => None,
            });

            result
        })
    }

    /// In certain cases there won't be any A records in the additional section. For this case, we
    /// introduce a method for returning the hostname of an appropriate name server.
    pub fn get_unsolved_ns<'a>(&'a self, qname: &'a str) -> Option<&'a str> {
        // Get an iterator over the name servers in the authorities section
        self.get_ns(qname)
            // Finally, pick the first one
            .find_map(|(_, host)| Some(host))
    }
}

#[cfg(test)]
mod tests {
    use super::DnsPacket;
    use super::DnsRecord;
    use std::net::Ipv4Addr;

    #[test]
    fn get_random_a_ok() {
        // arrange
        let mut dns_packet = DnsPacket::new();
        let ips = vec![
            Ipv4Addr::new(216, 239, 34, 10),
            Ipv4Addr::new(216, 239, 38, 10),
        ];
        dns_packet.answers.push(DnsRecord::A {
            domain: String::from("google.com"),
            addr: Ipv4Addr::new(216, 239, 34, 10),
            ttl: 87,
        });

        dns_packet.answers.push(DnsRecord::A {
            domain: String::from("yahoo.com"),
            addr: Ipv4Addr::new(216, 239, 38, 10),
            ttl: 88,
        });

        // act
        let actual = dns_packet.get_random_a().unwrap();

        // assert
        assert!(ips.contains(&actual));
    }

    #[test]
    fn get_ns_ok() {
        // arrange
        let mut dns_packet = DnsPacket::new();
        let expected = vec![
            // (domain, hostname)
            ("google.com", "ns2.google.com."),
            ("google.com", "ns1.google.com."),
        ];

        dns_packet.authorities.push(DnsRecord::NS {
            domain: String::from("google.com"),
            host: String::from("ns2.google.com."),
            ttl: 88,
        });

        dns_packet.authorities.push(DnsRecord::NS {
            domain: String::from("google.com"),
            host: String::from("ns1.google.com."),
            ttl: 91,
        });

        dns_packet.authorities.push(DnsRecord::NS {
            domain: String::from("yahoo.com"),
            host: String::from("ns1.yahoo.com."),
            ttl: 90,
        });

        // act
        let actual = dns_packet.get_ns("https://google.com");

        // assert
        let result: Vec<(&str, &str)> = actual.collect();
        assert_eq!(expected, result);
    }

    #[test]
    fn get_resolved_ns_ok() {
        // arrange
        let mut dns_packet = DnsPacket::new();
        let expected = vec![
            Ipv4Addr::new(216, 239, 34, 10),
            Ipv4Addr::new(216, 239, 32, 10),
        ];

        dns_packet.authorities.push(DnsRecord::NS {
            domain: String::from("google.com"),
            host: String::from("ns2.google.com."),
            ttl: 88,
        });

        dns_packet.authorities.push(DnsRecord::NS {
            domain: String::from("google.com"),
            host: String::from("ns1.google.com."),
            ttl: 91,
        });

        dns_packet.authorities.push(DnsRecord::NS {
            domain: String::from("yahoo.com"),
            host: String::from("ns1.yahoo.com."),
            ttl: 90,
        });

        dns_packet.resources.push(DnsRecord::A {
            domain: String::from("ns2.google.com."),
            addr: Ipv4Addr::new(216, 239, 32, 10),
            ttl: 40,
        });

        dns_packet.resources.push(DnsRecord::A {
            domain: String::from("ns1.google.com."),
            addr: Ipv4Addr::new(216, 239, 34, 10),
            ttl: 38,
        });

        // act
        let actual = dns_packet.get_resolved_ns("https://google.com").unwrap();

        // assert
        assert!(expected.contains(&actual));
    }

    #[test]
    fn get_unsolved_ns_ok() {
        // arrange
        let mut dns_packet = DnsPacket::new();
        let expected = vec![
            String::from("ns1.google.com."),
            String::from("ns2.google.com."),
        ];

        dns_packet.authorities.push(DnsRecord::NS {
            domain: String::from("google.com"),
            host: String::from("ns2.google.com."),
            ttl: 88,
        });

        dns_packet.authorities.push(DnsRecord::NS {
            domain: String::from("google.com"),
            host: String::from("ns1.google.com."),
            ttl: 91,
        });

        dns_packet.authorities.push(DnsRecord::NS {
            domain: String::from("yahoo.com"),
            host: String::from("ns1.yahoo.com."),
            ttl: 90,
        });

        // act
        let actual = dns_packet.get_unsolved_ns("https://google.com").unwrap();

        // assert
        assert!(expected.contains(&actual.to_string()));
    }
}
