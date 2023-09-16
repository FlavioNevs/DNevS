use super::enums::{ResponseCode, QueryType, QueryClass};

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
    pub z: u8,
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
            z: 0,
            rcode: ResponseCode::NOERROR,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
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