
/// Type Field used in Resource Records (RR)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumType {
    /// Host address
    A = 1,
    /// Authoritative name server
    NS = 2,
    /// Mail destination (Obsolete - use MX)
    MD = 3,
    /// Mail forwarder (Obsolete - use MX)
    MF = 4,
    /// Canonical name for alias
    CNAME = 5,
    /// Start of a zone of authority
    SOA = 6,
    /// Mailbox domain name (EXPERIMENTAL)
    MB = 7,
    /// Mail group member (EXPERIMENTAL)
    MG = 8,
    /// Mail rename domain name (EXPERIMENTAL)
    MR = 9,
    /// Null RR (EXPERIMENTAL)
    NULL = 10,
    /// Well known service description
    WKS = 11,
    /// Domain name pointer
    PTR = 12,
    /// Host information
    HINFO = 13,
    /// Mailbox or mail list information
    MINFO = 14,
    /// Mailbox Exchange
    MX = 15,
    /// Text strings
    TXT = 16,
}

impl From<EnumType> for u16 {
    fn from(value: EnumType) -> Self {
        match value {
            EnumType::A => 1,
            EnumType::NS => 2,
            EnumType::MD => 3,
            EnumType::MF => 4,
            EnumType::CNAME => 5,
            EnumType::SOA => 6,
            EnumType::MB => 7,
            EnumType::MG => 8,
            EnumType::MR => 9,
            EnumType::NULL => 10,
            EnumType::WKS => 11,
            EnumType::PTR => 12,
            EnumType::HINFO => 13,
            EnumType::MINFO => 14,
            EnumType::MX => 15,
            EnumType::TXT => 16,
        }
    }
}


pub enum QueryType{
    /// All Types from [EnumType]
    EnumType(EnumType),
    /// Request for a transfer of an entire zone
    AXFR,
    /// Request for mailbox-related records (MB, MG or MR)
    MAILB,
    /// Request for mail agent RRs (Obsolete - see MX)
    MAILA,
    /// Request for all records
    ALL,
}

impl From<QueryType> for u16 {
    fn from(value: QueryType) -> Self {
        match value {
            QueryType::EnumType(x) => x.into(),
            QueryType::AXFR => 252,
            QueryType::MAILB => 253,
            QueryType::MAILA => 254,
            QueryType::ALL => 255,
        }
    }
}

pub enum ClassType {
    /// Internet
    IN,
    /// CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CS,
    /// CHAOS class
    CH,
    /// Hesiod
    HS,
}

impl From<ClassType> for u16 {
    fn from(value: ClassType) -> Self {
        match value {
            ClassType::IN => 1,
            ClassType::CS => 2,
            ClassType::CH => 3,
            ClassType::HS => 4,
        }
    }
}

pub enum QueryClassType {
    ClassType(ClassType),
    All,
}

impl From<QueryClassType> for u16 {
    fn from(value: QueryClassType) -> Self {
        match value {
            QueryClassType::ClassType(x) => x.into(),
            QueryClassType::All => 255,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseCode {
    /// No error condition
    NOERROR = 0,
    /// Format error
    /// 
    /// The name server was unable to interpret the query.
    FORMERR = 1,
    ///  Server failure
    /// 
    /// The name server was unable to process this query due to a 
    /// problem with the name server.
    SERVFAIL = 2,
    /// Name Error
    /// 
    /// Meaningful only for responses from an authoritative name 
    /// server, this code signifies that the domain name referenced 
    /// in the query does not exist.
    NXDOMAIN = 3,
    /// Not Implemented
    /// 
    /// The name server does not support the requested kind of query.
    NOTIMP = 4,
    /// Refused
    /// 
    /// The name server refuses to perform the specified operation for
    /// policy reasons.
    REFUSED = 5,
}