use super::{message::{DnsHeader, DnsQuestion, DnsRecord}, buffer::DnsPacketBuffer};




#[derive(Debug, Clone)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>,
}

impl Default for DnsPacket {
    fn default() -> Self {
        Self { 
            header: DnsHeader::default(),
            questions: vec![],
            answers: vec![],
            authorities: vec![],
            resources: vec![],
        }
    }
}

impl From<DnsPacketBuffer> for DnsPacket {
    fn from(mut value: DnsPacketBuffer) -> Self {
        let mut result = DnsPacket::default();

        result.header = DnsHeader::from(&mut value);

        for _ in 0..result.header.qdcount {
            result.questions.push(DnsQuestion::from(&mut value));
        }

        for _ in 0..result.header.ancount {
            result.answers.push(DnsRecord::from(&mut value));
        }
        for _ in 0..result.header.nscount {
            result.authorities.push(DnsRecord::from(&mut value));
        }
        for _ in 0..result.header.arcount {
            result.resources.push(DnsRecord::from(&mut value));
        }

        result
    }
}


