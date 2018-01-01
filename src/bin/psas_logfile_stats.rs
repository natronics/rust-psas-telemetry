extern crate psas_telemetry;

use std::env;
use std::io;
use std::io::prelude::*;


fn main() {
    let mut args = env::args();
    let mut input = String::new();
    if args.len() > 1 {
        let name = args.nth(1).unwrap();

        let stats = psas_telemetry::get_stats(name);
        println!("Number of sequence numbers in file: {}", stats.n_seqn);
        println!("First sequence number: {}", stats.first_seqn);
        println!("Last sequence number: {}", stats.last_seqn);
        println!("Number of bytes read: {}", stats.n_bytes);
        println!("Number of messages (excluding SEQN): {}", stats.n_messages);
        println!("Message counts:");
        for (fourcc, n) in &stats.message_counts {
            println!("  - {}: {}", String::from_utf8_lossy(fourcc), n);
        }
    } else {
        io::stdin().read_to_string(&mut input).unwrap();
    }
}
