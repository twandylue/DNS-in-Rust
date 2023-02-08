pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl BytePacketBuffer {
    pub fn new() -> Self {
        BytePacketBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }

    /// Current position within buffer
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Step the buffer position forward a specific number of steps
    pub fn step(&mut self, steps: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.pos += steps;

        Ok(())
    }

    /// Change the buffer position
    pub fn seek(&mut self, pos: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.pos = pos;

        Ok(())
    }

    /// Read a single byte and move the position one step forward
    fn read(&mut self) -> Result<u8, Box<dyn std::error::Error>> {
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }
        let res = self.buf[self.pos];
        self.pos += 1;

        Ok(res)
    }

    /// Get a single byte, without changing the buffer position
    fn get(&self, pos: usize) -> Result<u8, Box<dyn std::error::Error>> {
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }

        Ok(self.buf[pos])
    }

    /// Get a range of bytes
    pub fn get_range(&self, start: usize, len: usize) -> Result<&[u8], Box<dyn std::error::Error>> {
        if start + len > 512 {
            return Err("End of buffer".into());
        }

        Ok(&self.buf[start..start + len])
    }

    /// Read two bytes, stepping two steps forward
    pub fn read_u16(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        let res = ((self.read()? as u16) << 8) | ((self.read()? as u16) << 0);

        return Ok(res);
    }

    /// Read four bytes, stepping four steps forward
    pub fn read_u32(&mut self) -> Result<u32, Box<dyn std::error::Error>> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);

        return Ok(res);
    }

    /// Read a qname(domain name)
    pub fn read_qname(&mut self, outstr: &mut String) -> Result<(), Box<dyn std::error::Error>> {
        let mut pos = self.pos();

        let mut jumped = false;
        let max_jumps = 5;
        let mut jump_performed = 0;

        let mut delim = "";
        loop {
            if jump_performed > max_jumps {
                return Err(format!("Limit of {} jumps exceeded", max_jumps).into());
            }

            let b = self.get(pos)?;

            if (b & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(pos + 2)?;
                }

                let b2 = self.get(pos + 1)? as u16;
                let offset = (((b as u16) ^ 0xC0) << 8) | b2;
                // from the beginning of the buffer
                pos = offset as usize;

                jumped = true;
                jump_performed += 1;

                continue;
            } else {
                pos += 1;
                if b == 0 {
                    break;
                }

                outstr.push_str(delim);

                let str_buffer = self.get_range(pos, b as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

                delim = ".";
                pos += b as usize;
            }
        }

        if !jumped {
            self.seek(pos)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::BytePacketBuffer;

    #[test]
    fn new_ok_1() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected_buffer_len: usize = 512;
        let expected_pos: usize = 0;

        // act
        let buffer = BytePacketBuffer::new();

        // assert
        assert_eq!(expected_buffer_len, buffer.buf.len());
        assert_eq!(expected_pos, buffer.pos);
        Ok(())
    }

    #[test]
    fn pos_ok_1() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected: usize = 0;
        let buffer = BytePacketBuffer::new();

        // act
        let actual = buffer.pos();

        // assert
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn step_ok_1() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected: usize = 1;
        let mut buffer = BytePacketBuffer::new();
        buffer.step(1).unwrap();

        // act
        let actual = buffer.pos();

        // assert
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn seek_ok_1() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected: usize = 10;
        let mut buffer = BytePacketBuffer::new();
        buffer.seek(10).unwrap();

        // act
        let actual = buffer.pos();

        // assert
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn read_ok_1() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected_byte: u8 = 0;
        let expected_pos: usize = 1;
        let mut buffer = BytePacketBuffer::new();

        // act
        let actual = buffer.read()?;

        // assert
        assert_eq!(expected_byte, actual);
        assert_eq!(expected_pos, buffer.pos);
        Ok(())
    }

    #[test]
    fn read_error() {
        // arrange
        let mut buffer = BytePacketBuffer::new();
        buffer.seek(512).unwrap();

        // act
        let actual = buffer.read();

        // assert
        match actual {
            Ok(_) => (),
            Err(r) => {
                assert_eq!(r.to_string(), "End of buffer".to_string());
            }
        }
    }

    #[test]
    fn get_ok_1() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected: u8 = 0;
        let buffer = BytePacketBuffer::new();

        // act
        let actual = buffer.get(10)?;

        // assert
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn get_error() {
        // arrange
        let mut buffer = BytePacketBuffer::new();
        buffer.seek(512).unwrap();

        // act
        let actual = buffer.get(10);

        // assert
        match actual {
            Ok(_) => (),
            Err(r) => {
                assert_eq!(r.to_string(), "End of buffer".to_string());
            }
        }
    }

    #[test]
    fn get_range_ok_1() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected: [u8; 512] = [0; 512];
        let buffer = BytePacketBuffer::new();

        // act
        let actual = buffer.get_range(0, 512)?;

        // assert
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn get_range_ok_2() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected: [u8; 10] = [0; 10];
        let buffer = BytePacketBuffer::new();

        // act
        let actual = buffer.get_range(1, 10)?;

        // assert
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn get_range_error_1() {
        // arrange
        let buffer = BytePacketBuffer::new();

        // act
        let actual = buffer.get_range(0, 513);

        // assert
        match actual {
            Ok(_) => (),
            Err(r) => {
                assert_eq!(r.to_string(), "End of buffer".to_string());
            }
        }
    }

    #[test]
    fn get_range_error_2() {
        // arrange
        let buffer = BytePacketBuffer::new();

        // act
        let actual = buffer.get_range(1, 512);

        // assert
        match actual {
            Ok(_) => (),
            Err(r) => {
                assert_eq!(r.to_string(), "End of buffer".to_string());
            }
        }
    }

    #[test]
    fn get_read_u16_ok_1() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected: u16 = 0;
        let mut buffer = BytePacketBuffer::new();

        // act
        let actual = buffer.read_u16()?;

        // assert
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn get_read_u16_ok_2() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected: u16 = 257;
        let mut buffer = BytePacketBuffer::new();
        buffer.buf = [1; 512];

        // act
        let actual = buffer.read_u16()?;

        // assert
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn get_read_u16_ok_3() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected: u16 = 514;
        let mut buffer = BytePacketBuffer::new();
        buffer.buf = [2; 512];

        // act
        let actual = buffer.read_u16()?;

        // assert
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn get_read_u32_ok_1() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected: u32 = 0;
        let mut buffer = BytePacketBuffer::new();
        // buffer.buf = [0; 512];

        // act
        let actual = buffer.read_u32()?;

        // assert
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn get_read_u32_ok_2() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected: u32 = 16843009;
        let mut buffer = BytePacketBuffer::new();
        buffer.buf = [1; 512];

        // act
        let actual = buffer.read_u32()?;

        // assert
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn get_read_u32_ok_3() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected: u32 = 33686018;
        let mut buffer = BytePacketBuffer::new();
        buffer.buf = [2; 512];

        // act
        let actual = buffer.read_u32()?;

        // assert
        assert_eq!(expected, actual);
        Ok(())
    }
}
