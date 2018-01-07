/* This file is auto generated! Do not edit. */
{% for definition in defs %}
pub mod {{ definition.fourcc | lower }};
{% endfor %}

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Error;
use std::io::BufReader;
use std::io::Cursor;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};


// The `SEQN` header is part of the spec, and not defined with the TOML files.
pub const SIZE_OF_HEADER: usize = 4 + 6 + 2;

/// A message header
pub struct Header {
    /// The four character code, determines the type of message
    pub fourcc: [u8; 4],

    /// Time received in nanoseconds from message epoch
    pub timestamp: u64,

    /// The size of the upcoming message body, in bytes
    pub size: usize,
}


impl Header {

    /// Write a message header to a buffer.
    pub fn write(&self, buffer: &mut Vec<u8>) {

        // Four character code
        buffer.write(&self.fourcc).unwrap();

        // Time, truncated to 6 bytes
        let mut time_buffer = vec![];
        time_buffer.write_u64::<BigEndian>(self.timestamp).unwrap();
        buffer.write(&time_buffer[2..8]).unwrap();

        // Size
        buffer.write_u16::<BigEndian>(self.size as u16).unwrap();
    }
}


/// Read from a buffer assuming were at the boundary of a message.
/// Extracts the header
pub fn read_header<I>(reader: &mut BufReader<I>) -> Result<Header, Error> where I: Read  {
    let mut fourcc_buf = [0; 4];
    let mut header_buf = [0; 8];

    let _char_read = reader.read_exact(&mut fourcc_buf)?;
    let _char_read = reader.read_exact(&mut header_buf)?;
    let mut header_cursor = Cursor::new(&header_buf);

    let timestamp_hi = header_cursor.read_u16::<BigEndian>().unwrap() as u64;
    let timestamp_lo = header_cursor.read_u32::<BigEndian>().unwrap() as u64;
    let message_length = header_cursor.read_u16::<BigEndian>().unwrap() as usize;

    let timestamp: u64 = timestamp_hi << 32 | timestamp_lo;

    Ok(Header {
        fourcc: fourcc_buf,
        timestamp: timestamp,
        size: message_length,
    })
}


pub fn read_seqn_body(body: &Vec<u8>) -> u32 {
    let mut body = Cursor::new(body);
    body.read_u32::<BigEndian>().unwrap()
}


/// Attempt to read Message body based on it's fourcc and convert to a CSV line of scaled, unit-converted values
pub fn to_csv(fourcc: [u8; 4], body: Vec<u8>) -> Option<String> {
    match &fourcc {
        {% for definition in defs %}
        b"{{ definition.fourcc }}" => {
            let data_raw = {{ definition.fourcc | lower }}::read_raw(body);
            let data_scaled = {{ definition.fourcc | lower }}::{{ definition.fourcc }}::from_raw(data_raw);
            Some(data_scaled.to_csv_line())
        },
        {% endfor %}
        _ => {
            println!("Skipped type {}", String::from_utf8_lossy(&fourcc));
            None
        },
    }
}


/// Attempt to read Message body based on it's fourcc and convert to a map of scaled, unit-converted values
pub fn to_values(fourcc: [u8; 4], body: Vec<u8>) -> Option<HashMap<String, f64>> {
    match &fourcc {
        {% for definition in defs %}
        b"{{ definition.fourcc }}" => {
            let data_raw = {{ definition.fourcc | lower }}::read_raw(body);
            let data_scaled = {{ definition.fourcc | lower }}::{{ definition.fourcc }}::from_raw(data_raw);
            Some(data_scaled.to_dict())
        },
        {% endfor %}
        _ => {
            println!("Skipped type {}", String::from_utf8_lossy(&fourcc));
            None
        },
    }
}
