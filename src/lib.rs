/*!
Utilities to read and write binary messages in the PSAS rocket framework.
*/
extern crate byteorder;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

// This module is entirely generated by the code generator based on defined
// message types.
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
        stats.n_bytes += 4 + 6 + 2;

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
                let seqn = messages::read_seqn_body(body_buf);
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
}
