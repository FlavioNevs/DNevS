use thiserror::Error;

#[derive(Debug, Error)]
pub enum DnsPacketError {
    #[error("DnsPacketBuffer out of bounds at `{0}`")]
    OutOfBounds(usize),
    #[error("Jump limit exceed. max: `{0}`")]
    MaxJumpsLimit(i32),
}