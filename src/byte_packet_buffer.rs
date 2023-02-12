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

    fn write(&mut self, val: u8) -> Result<(), Box<dyn std::error::Error>> {
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }

        self.buf[self.pos] = val;
        self.pos += 1;

        Ok(())
    }

    pub fn write_u8(&mut self, val: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.write(val)?;

        Ok(())
    }

    pub fn write_u16(&mut self, val: u16) -> Result<(), Box<dyn std::error::Error>> {
        self.write((val >> 8) as u8)?;
        self.write((val & 0xFF) as u8)?;

        Ok(())
    }

    pub fn write_u32(&mut self, val: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.write(((val >> 24) & 0xFF) as u8)?;
        self.write(((val >> 16) & 0xFF) as u8)?;
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write(((val >> 0) & 0xFF) as u8)?;

        Ok(())
    }

    pub fn write_qname(&mut self, qname: &str) -> Result<(), Box<dyn std::error::Error>> {
        for section in qname.split('.') {
            let len = section.len();
            if len > 0x3f {
                return Err("Single section exceeds 63 characters of length".into());
            }

            self.write_u8(len as u8)?;
            for b in section.as_bytes() {
                self.write_u8(*b)?;
            }
        }

        self.write_u8(0)?;

        Ok(())
    }

    pub fn set(&mut self, pos: usize, val: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.buf[pos] = val;

        Ok(())
    }

    pub fn set_u16(&mut self, pos: usize, val: u16) -> Result<(), Box<dyn std::error::Error>> {
        self.set(pos, (val >> 8) as u8)?;
        self.set(pos + 1, (val & 0xFF) as u8)?;

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

    #[test]
    fn write_ok() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let mut sut = BytePacketBuffer::new();
        let input: u8 = 1;

        // act
        sut.write(input)?;

        // assert
        assert_eq!(sut.buf[0], 1);

        Ok(())
    }

    #[test]
    fn write_u8_ok() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let mut sut = BytePacketBuffer::new();
        let input: u8 = 1;

        // act
        sut.write_u8(input)?;

        // assert
        assert_eq!(sut.buf[0], 1);

        Ok(())
    }

    #[test]
    fn write_u16_ok() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let mut sut = BytePacketBuffer::new();
        let input = 0xff;

        // act
        sut.write_u16(input)?;

        // assert
        assert_eq!(sut.buf[0], 0);
        assert_eq!(sut.buf[1], 255);

        Ok(())
    }

    #[test]
    fn write_u32_ok() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let mut sut = BytePacketBuffer::new();
        // 111111111111111111111111
        let input: u32 = (0xff << 24) | (0xff << 16) | (0xff << 8) | 0xff;

        // act
        sut.write_u32(input)?;

        // assert
        assert_eq!(sut.buf[0], 255);
        assert_eq!(sut.buf[1], 255);
        assert_eq!(sut.buf[2], 255);
        assert_eq!(sut.buf[3], 255);

        Ok(())
    }

    #[test]
    fn write_write_qname_ok() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let mut sut = BytePacketBuffer::new();
        let mut input = "google.com.tw".to_string();

        // act
        sut.write_qname(&mut input)?;

        // assert
        println!("{:?}", sut.buf);
        // 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 2, 116, 119, 0
        assert_eq!(sut.buf[0], 6);
        assert_eq!(sut.buf[1], 103);
        assert_eq!(sut.buf[2], 111);
        assert_eq!(sut.buf[3], 111);
        assert_eq!(sut.buf[4], 103);
        assert_eq!(sut.buf[5], 108);
        assert_eq!(sut.buf[6], 101);
        assert_eq!(sut.buf[7], 3);
        assert_eq!(sut.buf[8], 99);
        assert_eq!(sut.buf[9], 111);
        assert_eq!(sut.buf[10], 109);
        assert_eq!(sut.buf[11], 2);
        assert_eq!(sut.buf[12], 116);
        assert_eq!(sut.buf[13], 119);
        assert_eq!(sut.buf[14], 0);

        Ok(())
    }

    #[test]
    fn set_ok() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let mut sut = BytePacketBuffer::new();

        // act
        sut.set(0, 1)?;

        // assert
        sut.buf[0] = 1;

        Ok(())
    }

    #[test]
    fn set_u16_ok() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let mut sut = BytePacketBuffer::new();

        // act
        sut.set_u16(0, 0xFFFF)?;

        // assert
        sut.buf[0] = 255;
        sut.buf[1] = 255;

        Ok(())
    }
}
