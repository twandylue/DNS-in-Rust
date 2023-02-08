use std::error::Error;

use crate::{byte_packet_buffer::BytePacketBuffer, result_code::ResultCode};

#[derive(Clone, Debug)]
pub struct DnsHeader {
    pub id: u16,

    pub truncated_message: bool,
    pub recursion_desired: bool,
    pub authoritative_answer: bool,
    pub opcode: u8,
    pub query_response: bool,

    pub response_code: ResultCode,
    // pub checking_disabled: bool,   // 1 bit
    // pub authed_data: bool,         // 1 bit
    pub z: bool,
    pub recursion_available: bool,

    pub questions_count: u16,
    pub answers_count: u16,
    pub authority_count: u16,  // pub authoritative_entries: u16, // 16 bits
    pub additional_count: u16, // pub resource_entries: u16,      // 16 bits
}

impl DnsHeader {
    pub fn new() -> Self {
        DnsHeader {
            id: 0,

            truncated_message: false,
            recursion_desired: false,
            authoritative_answer: false,
            opcode: 0,
            query_response: false,

            response_code: ResultCode::NOERROR,
            // checking_disabled: false,
            // authed_data: false,
            z: false,
            recursion_available: false,

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
        todo!()
    }
}
