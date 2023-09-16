use super::error::DnsPacketError;

pub struct DnsPacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl DnsPacketBuffer {

    /// Step the buffer forward
    pub fn step(&mut self, steps: usize) -> Result<(), DnsPacketError> {
        let new_pos = self.pos + steps;

        if new_pos >= 512 {
            Err(DnsPacketError::OutOfBounds(new_pos))
        } else {
            self.pos = new_pos;
            Ok(())
        }
    }

    /// Change the buffer position
    pub fn seek(&mut self, pos: usize) -> Result<(), DnsPacketError> {
        if pos >= 512 {
            Err(DnsPacketError::OutOfBounds(pos))
        } else {
            self.pos = pos;
            Ok(())
        }
    }

    /// Read a single byte and move the position one step forward
    pub fn read(&mut self) -> Result<u8, DnsPacketError> {
        if self.pos >= 512 {
            Err(DnsPacketError::OutOfBounds(self.pos))
        } else {
            let res = self.buf[self.pos];
            self.pos += 1;
    
            Ok(res)
        }
    }

    /// Get a single byte, without changing the buffer position
    pub fn get(&mut self, pos: usize) -> Result<u8, DnsPacketError> {
        if self.pos >= 512 {
            Err(DnsPacketError::OutOfBounds(self.pos))
        } else {
            Ok(self.buf[pos])
        }
    }

    /// Get a range of bytes
    pub fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], DnsPacketError> {
        if self.pos >= 512 {
            Err(DnsPacketError::OutOfBounds(self.pos))
        } else {
            Ok(&self.buf[start..start + len as usize])
        }
    }

    /// Read two bytes, stepping two steps forward
    pub fn read_u16(&mut self) -> Result<u16, DnsPacketError> {
        let res = ((self.read()? as u16) << 8) | (self.read()? as u16);

        Ok(res)
    }

    /// Read four bytes, stepping four steps forward
    pub fn read_u32(&mut self) -> Result<u32, DnsPacketError> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);

        Ok(res)
    }


    /// Read a qname
    ///
    /// The tricky part: Reading domain names, taking labels into consideration.
    /// Will take something like [3]www[6]google[3]com[0] and append
    /// www.google.com to outstr.
    pub fn read_qname(&mut self, outstr: &mut String) -> Result<(), DnsPacketError> {
        
        let mut pos = self.pos;

        let mut jumped = false;
        let max_jumps = 5;
        let mut jumps_performed = 0;

        let mut delim = "";
        loop {

            if jumps_performed > max_jumps {
                return Err(DnsPacketError::MaxJumpsLimit(max_jumps));
            }

            let len = self.get(pos)?;

            
            if (len & 0xC0) == 0xC0 {
                
                if !jumped {
                    self.seek(pos + 2)?;
                }
                
                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;
                
                jumped = true;
                jumps_performed += 1;

                continue;
            } else {
                
                pos += 1;
                
                if len == 0 {
                    break;
                }

                outstr.push_str(delim);

                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

                delim = ".";

                pos += len as usize;
            }
        }

        if !jumped {
            self.seek(pos)?;
        }

        Ok(())
    }
}

impl Default for DnsPacketBuffer {
    fn default() -> Self {
        Self {
            buf: [0; 512],
            pos: 0,
        }
    }
}
