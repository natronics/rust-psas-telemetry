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
                               .value_name("SEQN_I")
                               .help("First sequence number to include in output (if not set, start from the beginning)")
                               .takes_value(true))
                          .arg(Arg::with_name("end-seqn")
                               .short("e")
                               .long("end-seqn")
                               .value_name("SEQN_F")
                               .help("Last sequence number to include in output (if not set, read until end of file)")
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
                          .after_help("EXAMPLES:

    Copy all of the contents of 'input_file.log' to 'output_file.log' up until the first read error:
        $ psas_logfile_trunc input_file.log output_file.log

    Truncate 'input_file.log' from SEQN 23 to the end of the file:
        $ psas_logfile_trunc --begin-seqn=23 input_file.log output_file.log

    Truncate 'input_file.log' from SEQN 23 to SEQN 101:
        $ psas_logfile_trunc --begin-seqn=23 --end-seqn=101 input_file.log output_file.log

    Copy only the message type 'ADIS' from 'input_file.log' to 'output_file.log'
        $ psas_logfile_trunc --keep-mesg=ADIS input_file.log output_file.log

    Copy only the message types 'ADIS', 'RNHH', and 'FCFH' from 'input_file.log' to 'output_file.log'
        $ psas_logfile_trunc --keep-mesg=ADIS,RNHH,FCFH input_file.log output_file.log

    Copy only the message types 'ADIS', 'RNHH', and 'FCFH' between SEQN 23 and 100:
        $ psas_logfile_trunc --begin-seqn=23 --end-seqn=101 --keep-mesg=ADIS,RNHH,FCFH input_file.log output_file.log
")
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
    let keep_mesgs_list = keep_mesg_raw.split(",");
    let mut keep_mesgs = Vec::new();
    eprintln!("Keep messages:");
    for mesg in keep_mesgs_list {
        if mesg.len() > 0 {
            keep_mesgs.push(mesg);
            eprintln!("  - {}", mesg);
        }
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
    psas_telemetry::trunc_sequence(input_path, seqn_i, seqn_f, keep_mesgs, &mut write_buffer);
}
