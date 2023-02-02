pub struct BytePacketBuffer {
    buf: [u8; 512],
    pos: usize,
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
            return Err("End of the buffer".into());
        }
        let res = self.buf[self.pos];
        self.pos += 1;

        Ok(res)
    }

    /// Get a single byte, without changing the buffer position
    fn get(&self, pos: usize) -> Result<u8, Box<dyn std::error::Error>> {
        if self.pos >= 512 {
            return Err("End of the buffer".into());
        }

        Ok(self.buf[pos])
    }

    /// Get a range of bytes
    pub fn get_range(
        &mut self,
        start: usize,
        len: usize,
    ) -> Result<&[u8], Box<dyn std::error::Error>> {
        if start + len > 512 {
            return Err("End of the buffer".into());
        }

        Ok(&self.buf[start..start + len])
    }

    /// Read two bytes, stepping two steps forward
    pub fn read_u16(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        let res = ((self.read()? as u16) << 8) | ((self.read()? as u16) << 16);

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

    /// Read a qname
    ///
    pub fn read_qname(&mut self, outstr: &mut str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
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
    // TODO: how to test error message?
    fn read_ok_throw_error() {
        // arrange
        let mut buffer = BytePacketBuffer::new();
        buffer.seek(512).unwrap();

        // act
        let actual = buffer.read();

        // assert
        assert!(actual.is_err());
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

    // TODO: need more unit tests...
}
