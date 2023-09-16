use std::net::Ipv4Addr;

use super::{enums::{ResponseCode, QueryType, QueryClass}, buffer::DnsPacketBuffer};

#[derive(Debug, Clone)]
pub struct DnsHeader{
    /// Identifier
    pub id: u16,
    /// Operation code
    /// 
    /// specifies kind of query in this message.
    pub opcode: u8,

    /// Query/Response
    /// 
    /// [`bool`] field that specifies whether this message is a 
    /// query (false), or a response (true).
    pub qr: bool,
    /// Authoritative answer
    /// 
    /// Specifies that the responding name server is an authority 
    /// for the domain name in question section.
    pub aa: bool,
    /// Truncation
    /// 
    /// specifies that this message was truncated due to length 
    /// greater than that permitted on the transmission channel.
    pub tc: bool,
    /// Recursion Desired
    pub rd: bool,
    /// Recursion available
    /// 
    /// this be is set or cleared in a response, and denotes 
    /// whether recursive query support is available in the name 
    /// server.
    pub ra: bool,

    /// Reserved
    pub z: bool,
    /// Response code
    pub rcode: ResponseCode,

    /// Question count
    /// 
    /// Number of questions entries in the question section.
    pub qdcount: u16,
    /// Answer count
    /// 
    /// Number of resource records in the answer section.
    pub ancount: u16,
    /// Authority count
    /// 
    /// Number of name server resource records in the authority 
    /// records section.
    pub nscount: u16,
    /// Aditional count
    /// 
    /// Number of resource records in the additional records section.
    pub arcount: u16,
}

impl Default for DnsHeader {
    fn default() -> Self {
        Self {
            id: 0,
            opcode: 0,
            qr: false,
            aa: false,
            tc: false,
            rd: false,
            ra: false,
            z: false,
            rcode: ResponseCode::NOERROR,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }
}

impl From<&mut DnsPacketBuffer> for DnsHeader {
    fn from(value: &mut DnsPacketBuffer) -> Self {
        let id = value.read_u16().unwrap();

        let flags = value.read_u16().unwrap();
        let a = (flags >> 8) as u8;
        let b = (flags & 0xFF) as u8;

        Self {
            id,
            rd: (a & (1 << 0)) > 0,
            tc: (a & (1 << 1)) > 0,
            aa: (a & (1 << 2)) > 0,
            opcode: (a >> 3) & 0x0F,
            qr: (a & (1 << 7)) > 0,

            rcode: ResponseCode::from(b & 0x0F),

            z: (b & (1 << 6)) > 0,
            ra:  (b & (1 << 7)) > 0,

            qdcount: value.read_u16().unwrap(),
            ancount: value.read_u16().unwrap(),
            nscount: value.read_u16().unwrap(),
            arcount: value.read_u16().unwrap(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct DnsQuestion {
    /// Label sequence
    /// 
    /// A domain name represented as a sequence of labels.
    pub name: String,
    /// Query type
    /// 
    /// Specifies the type of the query.
    pub qtype: QueryType,
    /// Query class
    /// 
    /// specifies the class of the query.
    pub qclass: QueryClass
}

impl DnsQuestion {
    pub fn new(name: String, qtype: QueryType, qclass: QueryClass) -> Self {
        Self {
            name,
            qtype,
            qclass
        }
    }
}

impl Default for DnsQuestion {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            qtype: QueryType::ALL,
            qclass: QueryClass::ALL,
        }
    }
}

impl From<&mut DnsPacketBuffer> for DnsQuestion {
    fn from(value: &mut DnsPacketBuffer) -> Self {
        let mut question = DnsQuestion::default();

        value.read_qname(&mut question.name).unwrap();
        question.qtype = QueryType::from(value.read_u16().unwrap());
        question.qclass = QueryClass::from(value.read_u16().unwrap());

        question
    }
}

#[derive(Debug, Clone)]
pub enum DnsRecord {
    A {
        domain: String,
        addr: Ipv4Addr,
        ttl: u32
    },
    None
}

impl From<&mut DnsPacketBuffer> for DnsRecord {
    fn from(value: &mut DnsPacketBuffer) -> Self {
        let mut domain = String::new();
        value.read_qname(&mut domain).unwrap();

        let qtype_num = value.read_u16().unwrap();
        let qtype = QueryType::from(qtype_num);
        let _ = value.read_u16().unwrap();
        let ttl = value.read_u32().unwrap();
        let data_len = value.read_u16().unwrap();

        match qtype {
            QueryType::A => {
                let raw_addr = value.read_u32().unwrap();
                let addr = Ipv4Addr::new(
                    ((raw_addr >> 24) & 0xFF) as u8,
                    ((raw_addr >> 16) & 0xFF) as u8,
                    ((raw_addr >> 8) & 0xFF) as u8,
                    ((raw_addr >> 0) & 0xFF) as u8,
                );

                DnsRecord::A {
                    domain: domain,
                    addr: addr,
                    ttl: ttl,
                }
            }
            _ => todo!()
        }
    }
}
