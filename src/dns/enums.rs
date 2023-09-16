
/// Type Field used in Resource Records (RR)
pub enum EnumType {
    /// Host address
    A,
    /// Authoritative name server
    NS,
    /// Mail destination (Obsolete - use MX)
    MD,
    /// Mail forwarder (Obsolete - use MX)
    MF,
    /// Canonical name for alias
    CNAME,
    /// Start of a zone of authority
    SOA,
    /// Mailbox domain name (EXPERIMENTAL)
    MB,
    /// Mail group member (EXPERIMENTAL)
    MG,
    /// Mail rename domain name (EXPERIMENTAL)
    MR,
    /// Null RR (EXPERIMENTAL)
    NULL,
    /// Well known service description
    WKS,
    /// Domain name pointer
    PTR,
    /// Host information
    HINFO,
    /// Mailbox or mail list information
    MINFO,
    /// Mailbox Exchange
    MX,
    /// Text strings
    TXT
}

impl From<EnumType> for i32 {
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


pub enum EnumQType{
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

impl From<EnumQType> for i32 {
    fn from(value: EnumQType) -> Self {
        match value {
            EnumQType::EnumType(x) => x.into(),
            EnumQType::AXFR => 252,
            EnumQType::MAILB => 253,
            EnumQType::MAILA => 254,
            EnumQType::ALL => 255,
        }
    }
}

pub enum EnumClass {
    /// Internet
    IN,
    /// CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CS,
    /// CHAOS class
    CH,
    /// Hesiod
    HS,
}

impl From<EnumClass> for i32 {
    fn from(value: EnumClass) -> Self {
        match value {
            EnumClass::IN => 1,
            EnumClass::CS => 2,
            EnumClass::CH => 3,
            EnumClass::HS => 4,
        }
    }
}

pub enum EnumQClass {
    EnumClass(EnumClass),
    All,
}

impl From<EnumQClass> for i32 {
    fn from(value: EnumQClass) -> Self {
        match value {
            EnumQClass::EnumClass(x) => x.into(),
            EnumQClass::All => 255,
        }
    }
}