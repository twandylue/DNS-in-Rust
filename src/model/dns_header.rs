use super::super::BytePacketBuffer;
use super::result_code::ResultCode;

#[derive(Clone, Debug)]
pub struct DnsHeader {
    pub id: u16, // 16 bits

    pub query_response: bool,       // 1 bit
    pub opcode: u8,                 // 4 bits
    pub authoritative_answer: bool, // 1 bit
    pub truncated_message: bool,    // 1 bit
    pub recursion_desired: bool,    // 1 bit

    pub recursion_available: bool, // 1 bit
    pub z: bool,                   // 1 bit
    pub authed_data: bool,         // 1 bit (from z)
    pub checking_disabled: bool,   // 1 bit (from z)
    pub response_code: ResultCode, // 4 bits

    pub questions_count: u16,  // 16 bits
    pub answers_count: u16,    // 16 bits
    pub authority_count: u16,  // pub authoritative_entries: u16, // 16 bits
    pub additional_count: u16, // pub resource_entries: u16,      // 16 bits
}

impl DnsHeader {
    pub fn new() -> Self {
        DnsHeader {
            id: 0,

            query_response: false,
            opcode: 0,
            authoritative_answer: false,
            truncated_message: false,
            recursion_desired: false,

            recursion_available: false,
            z: false,
            authed_data: false,
            checking_disabled: false,
            response_code: ResultCode::NOERROR,

            questions_count: 0,
            answers_count: 0,
            authority_count: 0,
            additional_count: 0,
        }
    }

    pub fn read(
        &mut self,
        buffer: &mut BytePacketBuffer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.id = buffer.read_u16()?;

        let flags = buffer.read_u16()?;
        let a = (flags >> 8) as u8;
        let b = (flags & 0xFF) as u8;
        self.query_response = (a & (1 << 7)) > 0;
        self.opcode = (a >> 3) & 0x0F;
        self.authoritative_answer = (a & (1 << 2)) > 0;
        self.truncated_message = (a & (1 << 1)) > 0;
        self.recursion_desired = (a & (1 << 0)) > 0;

        self.recursion_available = (b & (1 << 7)) > 0;
        self.z = (b & (1 << 6)) > 0;
        self.authed_data = (b & (1 << 5)) > 0;
        self.checking_disabled = (b & (1 << 4)) > 0;
        self.response_code = ResultCode::from_number(b & 0x0F);

        self.questions_count = buffer.read_u16()?;
        self.answers_count = buffer.read_u16()?;
        self.authority_count = buffer.read_u16()?;
        self.additional_count = buffer.read_u16()?;

        Ok(())
    }

    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<(), Box<dyn std::error::Error>> {
        buffer.write_u16(self.id)?;

        // 0 0 0 0 0 0 0 1  0 0 1 0 0 0 0 0
        // - -+-+-+- - - -  - -+-+- -+-+-+-
        // Q    O    A T R  R   Z      R
        // R    P    A C D  A          C
        //      C                      O
        //      O                      D
        //      D                      E
        //      E
        buffer.write_u8(
            ((self.query_response as u8) << 7)
                | ((self.opcode as u8) << 3)
                | ((self.authoritative_answer as u8) << 2)
                | ((self.truncated_message as u8) << 1)
                | ((self.recursion_desired as u8) << 0),
        )?;

        buffer.write_u8(
            ((self.recursion_available as u8) << 7)
                | ((self.z as u8) << 6)
                | ((self.authed_data as u8) << 5)
                | ((self.checking_disabled as u8) << 4)
                | (self.response_code as u8),
        )?;

        buffer.write_u16(self.questions_count)?;
        buffer.write_u16(self.answers_count)?;
        buffer.write_u16(self.authority_count)?;
        buffer.write_u16(self.additional_count)?;

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::BytePacketBuffer;
    use super::DnsHeader;
    use super::ResultCode;

    #[test]
    fn dns_header_read_ok() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let mut sut = DnsHeader::new();
        let mut buffer = BytePacketBuffer::new();

        buffer.buf = [
            0x86, 0x2a, 0x01, 0x20, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ];

        // act
        sut.read(&mut buffer)?;

        // assert
        // 0 0 0 0 0 0 0 1  0 0 1 0 0 0 0 0
        // - -+-+-+- - - -  - -+-+- -+-+-+-
        // Q    O    A T R  R   Z      R
        // R    P    A C D  A          C
        //      C                      O
        //      O                      D
        //      D                      E
        //      E
        assert_eq!(sut.id, 34346);

        assert_eq!(sut.query_response, false);
        assert_eq!(sut.opcode, 0);
        assert_eq!(sut.authoritative_answer, false);
        assert_eq!(sut.truncated_message, false);
        assert_eq!(sut.recursion_desired, true);

        assert_eq!(sut.recursion_available, false);
        assert_eq!(sut.z, false);
        assert_eq!(sut.authed_data, true);
        assert_eq!(sut.checking_disabled, false);
        assert_eq!(sut.response_code, ResultCode::from_number(0));

        assert_eq!(sut.questions_count, 1);
        assert_eq!(sut.answers_count, 0);
        assert_eq!(sut.authority_count, 0);
        assert_eq!(sut.additional_count, 0);

        assert_eq!(buffer.pos(), 12);

        Ok(())
    }

    #[test]
    fn write_ok() -> Result<(), Box<dyn std::error::Error>> {
        // arrange
        let expected = [
            0x86, 0x2a, 0x01, 0x20, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ];

        let mut buffer = BytePacketBuffer::new();
        let mut sut = DnsHeader::new();
        sut.id = 34346;

        sut.query_response = false;
        sut.opcode = 0;
        sut.authoritative_answer = false;
        sut.truncated_message = false;
        sut.recursion_desired = true;

        sut.recursion_available = false;
        sut.z = false;
        sut.authed_data = true;
        sut.checking_disabled = false;
        sut.response_code = ResultCode::from_number(0);

        sut.questions_count = 1;
        sut.answers_count = 0;
        sut.authority_count = 0;
        sut.additional_count = 0;

        // act
        sut.write(&mut buffer)?;

        // assert
        assert_eq!(expected, buffer.buf);

        Ok(())
    }
}
