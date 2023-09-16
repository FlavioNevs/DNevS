
/// Type Field used in Resource Records (RR)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
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

    /// QType - Transfer zone
    /// 
    /// Request for a transfer of an entire zone
    AXFR = 252,
    /// QType - Mailbox records
    /// 
    /// Request for mailbox-related records (MB, MG or MR)
    MAILB = 253,
    /// QType - Mail agent
    /// 
    /// Request for mail agent RRs (Obsolete - see MX)
    MAILA = 254,
    /// QType - All
    /// 
    /// Request for all records
    ALL = 255,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryClass {
    /// Internet
    IN = 1,
    /// CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CS = 2,
    /// CHAOS class
    CH = 3,
    /// Hesiod
    HS = 4,

    /// QClass - All
    ///
    /// All classes
    ALL = 255
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