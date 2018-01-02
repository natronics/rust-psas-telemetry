/*!
Utilities to read and write binary messages in the PSAS rocket framework.
*/
extern crate byteorder;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Cursor;
use std::fs::File;
use std::path::Path;
use std::str;


// This module is entirely generated by the code generator based on defined message types.
pub mod messages;


/// Statistics about a set of telemetry messages
pub struct Stats {
    /// Number of `SEQN` (sequence number) encountered
    pub n_seqn: u64,

    /// The value of the first `SEQN` (sequence number) encountered
    pub first_seqn: u32,

    /// The value of the last `SEQN` (sequence number) encountered
    pub last_seqn: u32,

    /// Total number or bytes successfully read including unknown messages
    pub n_bytes: u64,

    /// Total number of messages encountered
    pub n_messages: u64,

    /// A map of the number of messages encountered, keyed by their four character code
    pub message_counts: HashMap<[u8;4], u64>,
}


/// Return a collection of statistics about a log file
pub fn get_stats(filename: String) -> Stats {

    // Read our file
    let fh = File::open(&filename).unwrap();
    let mut reader = BufReader::new(fh);

    // Initialize our stats to zero 
    let mut stats = Stats {
        n_seqn: 0,
        first_seqn: 0,
        last_seqn: 0,
        n_bytes: 0,
        n_messages: 0,
        message_counts: HashMap::new(),
    };

    // Flag to set after we find our first `SEQN` header
    let mut first_seqn = true;

    // Scan the file
    loop {

        let header = match messages::read_header(&mut reader) {
            Ok(header) => header,
            Err(err) => {
                eprintln!("Warning! \n{:?}", err);
                break;
            },
        };

        // We read a header message, which is always this many bytes
        stats.n_bytes += messages::SIZE_OF_HEADER as u64;

        // Try and copy the message size number of bytes into a buffer
        let mut body_buf = vec![0u8; header.size];
        match reader.read_exact(&mut body_buf) {
            Ok(()) => (),
            Err(err) => {
                eprintln!("Warning! \n{:?}", err);
                break;
            },
        };

        // Note the amount of bytes read
        stats.n_bytes += header.size as u64;

        // Match on the kind of message
        match &header.fourcc {
            b"SEQN" => {
                let seqn = messages::read_seqn_body(&body_buf);
                stats.n_seqn += 1;
                if first_seqn {
                    stats.first_seqn = seqn;
                    first_seqn = false;
                }
                stats.last_seqn = seqn;
            },
            _ => {
                stats.n_messages += 1;
                let n = stats.message_counts.entry(header.fourcc).or_insert(0);
                *n += 1;
            },
        };
    }

    stats
}


/// Given a logfile and a message type (four character code), write a CSV file
pub fn serialize_message_to_csv<I>(input_filename: String, fourcc_to_match: &[u8; 4], writer: &mut BufWriter<I>) where I: Write {
    // Read our file
    let fh = File::open(&input_filename).unwrap();
    let mut reader = BufReader::new(fh);

    loop {
        let header = match messages::read_header(&mut reader) {
            Ok(header) => header,
            Err(_) => break,
        };
        let mut body_buf = vec![0u8; header.size];
        match reader.read_exact(&mut body_buf) {
            Ok(()) => (),
            Err(_) => break,
        };

        if header.fourcc == *fourcc_to_match {
            // We match all in read_all, but because we're already filtering
            // we'll only get back the data we want
            match messages::to_csv(header.fourcc, body_buf) {
                Some(line) => write!(writer, "{},{}\n", header.timestamp, line).unwrap(),
                None => {},
            };
        }
    }
}


/// Parse file and only return certain sequence numbers. Writes into a buffer
pub fn trunc_sequence<I>(input_filename: &Path, seqn_i: i64, seqn_f: i64, keeplist: Vec<&str>, output_buffer: &mut BufWriter<I>) where I: Write {

    // Read our file
    let fh = File::open(input_filename).unwrap();
    let mut reader = BufReader::new(fh);

    let mut write = false;

    loop {
        let mut raw_header = [0; messages::SIZE_OF_HEADER];
        match reader.read_exact(&mut raw_header) {
            Ok(()) => (),
            Err(err) => {
                eprintln!("Warning! \n {:?}", err);
                break;
            },
        }
        let header = messages::read_header(&mut BufReader::new(Cursor::new(&raw_header))).unwrap();
        let mut body_buf = vec![0u8; header.size];
        match reader.read_exact(&mut body_buf) {
            Ok(()) => (),
            Err(err) => {
                eprintln!("Warning! \n {:?}", err);
                break;
            },
        };

        // Match on the kind of message
        match &header.fourcc {
            b"SEQN" => {
                let seqn = messages::read_seqn_body(&body_buf) as i64;

                // Valid ending SEQN
                if seqn_f > 0 {
                    // In the write range
                    if seqn >= seqn_i && seqn <= seqn_f {
                        write = true;
                        output_buffer.write(&raw_header).unwrap();
                        output_buffer.write(body_buf.as_slice()).unwrap();
                    }
                    else { write = false; }
                }
                else {
                    // In the write range as long as we're higher than the begin value
                    if seqn >= seqn_i {
                        write = true;
                        output_buffer.write(&raw_header).unwrap();
                        output_buffer.write(body_buf.as_slice()).unwrap();
                    }
                    else { write = false; }
                }
            }
            _ => {
                if write {
                    if keeplist.len() > 0 {
                        let fourcc = str::from_utf8(&header.fourcc).unwrap();
                        for mesg in &keeplist {
                            if &fourcc == mesg {
                                output_buffer.write(&raw_header).unwrap();
                                output_buffer.write(body_buf.as_slice()).unwrap();
                            }
                        }
                    }
                    else {
                        output_buffer.write(&raw_header).unwrap();
                        output_buffer.write(body_buf.as_slice()).unwrap();
                    }
                }
            },
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_file_stats_from_l12_short() {
        let stats = get_stats("testdata/launch-12_short.log".to_owned());
        assert_eq!(stats.n_seqn, 191);
        assert_eq!(stats.first_seqn, 0);
        assert_eq!(stats.last_seqn, 190);
        assert_eq!(stats.n_bytes, 249997);
        assert_eq!(stats.n_messages, 8755);

        // What's in our dict of message counts
        for (fourcc, n) in &stats.message_counts {
            match fourcc {
                b"FCFH" => assert_eq!(*n,   16),
                b"RNHH" => assert_eq!(*n,    9),
                b"VERS" => assert_eq!(*n,    1),
                b"SEQE" => assert_eq!(*n,    3),
                b"RNHP" => assert_eq!(*n, 8726),
                _ => {
                    // We should have found all the possible messages, so we don't want to get here!
                    eprintln!("{}", String::from_utf8_lossy(fourcc));
                    assert!(false);  // Fail!
                },
            }
        }
    }

    #[test]
    fn make_csv_of_one_message_type_inmem_from_l12_short() {

        let mem: Vec<u8> = Vec::new();
        let mut write_buffer = BufWriter::new(mem);

        serialize_message_to_csv("testdata/launch-12_short.log".to_owned(), b"RNHP", &mut write_buffer);

        // This is our "file" in memory
        let mem = write_buffer.into_inner().unwrap();
        let file_lines = String::from_utf8_lossy(&mem);

        let mut linecount = 0;
        for _ in file_lines.split("\n") {
            linecount += 1;
        }
        assert_eq!(linecount, 8727);
    }
}
