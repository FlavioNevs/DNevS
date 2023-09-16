use super::enums::ResponseCode;

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
    /// [bool] field that specifies whether this message is a 
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
    pub rd: Option<bool>,
    /// Recursion available
    /// 
    /// this be is set or cleared in a response, and denotes 
    /// whether recursive query support is available in the name 
    /// server.
    pub ra: Option<bool>,

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
