extern crate psas_telemetry;
extern crate clap;

use std::error::Error;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

use clap::{Arg, App};


fn main() {
    // Setup argument parsing
    let matches = App::new("PSAS Telemetry Logfile Truncate Utility")
                          .version("0.1.0")
                          .author("Nathan Bergey <nathan@psas.pdx.edu>")
                          .about("
  This utility can help cut down full logfiles into specific sections.
  Use it to cut up a file by sequence number")
                          .arg(Arg::with_name("begin-seqn")
                               .short("s")
                               .long("begin-seqn")
                               .value_name("SEQN_F")
                               .help("First sequence number to include in output (don't include to start from the beginning)")
                               .takes_value(true))
                          .arg(Arg::with_name("end-seqn")
                               .short("f")
                               .long("end-seqn")
                               .value_name("SEQN_F")
                               .help("Last sequence number to include in output (don't include to read until end)")
                               .takes_value(true))
                          .arg(Arg::with_name("keep-mesg")
                               .short("k")
                               .long("keep-mesg")
                               .value_name("list of message to keep")
                               .help("Include these message types in the output (Reject everything else)")
                               .takes_value(true))
                          .arg(Arg::with_name("strip-mesg")
                               .short("x")
                               .long("strip-mesg")
                               .value_name("list of message to exclude")
                               .help("Exclude these message types in the output (Keep everything else)")
                               .takes_value(true))
                          .arg(Arg::with_name("INPUT LOGFILE")
                               .help("Sets the logfile file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("OUTPUT LOGFILE")
                               .help("What file to write output to")
                               .required(true)
                               .index(2))
                          .get_matches();

    // Set up from args:

    // We expect a filename to parse
    let input_filename = matches.value_of("INPUT LOGFILE").unwrap();
    let input_path = Path::new(input_filename);
    eprintln!("Using input file: {}", input_filename);

    // What we're splitting on
    let seqn_i_raw = matches.value_of("begin-seqn").unwrap_or("-1");
    let seqn_i = match seqn_i_raw.parse::<i64>() {
        Err(_) => panic!("begin-seqn \"{}\" must be a number", seqn_i_raw),
        Ok(num) => num,
    };
    eprintln!("First seqn to keep: {}", seqn_i);

    let seqn_f_raw = matches.value_of("end-seqn").unwrap_or("-1");
    let seqn_f = match seqn_f_raw.parse::<i64>() {
        Err(_) => panic!("end-seqn \"{}\" must be a number", seqn_f_raw),
        Ok(num) => num,
    };
    if seqn_f > 0 && seqn_f < seqn_i{
        panic!("end-seqn ({}) must be greater than begin-seqn ({})", seqn_f, seqn_i);
    }
    eprintln!("Last seqn to keep: {}", seqn_f);

    // What messages we're keeping
    let keep_mesg_raw = matches.value_of("keep-mesg").unwrap_or("");
    let keep_mesgs = keep_mesg_raw.split(",");
    eprintln!("Keep messages:");
    for mesg in keep_mesgs {
        eprintln!("  - {}", mesg);
    }


    // Write out to this file
    let out_filename = matches.value_of("OUTPUT LOGFILE").unwrap();
    let output_path = Path::new(out_filename);
    let file = match File::create(&output_path) {
        Err(why) => panic!("couldn't create output file {}: {}",
                    output_path.display(),
                    why.description()),
        Ok(file) => file,
    };
    let mut write_buffer = BufWriter::new(file);

    // Use `trunc_sequence`
    psas_telemetry::trunc_sequence(input_path, seqn_i, seqn_f, &mut write_buffer);
}
