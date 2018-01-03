extern crate psas_telemetry;
extern crate clap;

use std::error::Error;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

use clap::{Arg, App};


fn main() {
    // Setup argument parsing
    let matches = App::new("PSAS Telemetry Logfile Message Data Extract Utility")
                          .version("0.1.0")
                          .author("Nathan Bergey <nathan@psas.pdx.edu>")
                          .about("
  This utility can pull converted message data from a logfile.")
                          .arg(Arg::with_name("mesg")
                               .short("m")
                               .long("mesg")
                               .value_name("MESG")
                               .help("Message type to extract")
                               .required(true)
                               .takes_value(true))
                          .arg(Arg::with_name("INPUT LOGFILE")
                               .help("Logfile file to read")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("OUTPUT")
                               .help("What write output to")
                               .required(true)
                               .index(2))
                          .after_help("EXAMPLES:
").get_matches();


    // We expect a filename to parse
    let input_filename = matches.value_of("INPUT LOGFILE").unwrap();
    let input_path = Path::new(input_filename);
    eprintln!("Using input file: {}", input_filename);

    // What we're splitting on
    let mesg_raw = matches.value_of("mesg").unwrap();
    if mesg_raw.len() != 4 {
        panic!("Message type should be four character code, got \"{}\"", mesg_raw)
    }
    let mut mesg_fourcc: [u8;4] = [0,0,0,0];
    mesg_fourcc[0] = mesg_raw.as_bytes()[0];
    mesg_fourcc[1] = mesg_raw.as_bytes()[1];
    mesg_fourcc[2] = mesg_raw.as_bytes()[2];
    mesg_fourcc[3] = mesg_raw.as_bytes()[3];
    eprintln!("Message type to extract: {}", String::from_utf8_lossy(&mesg_fourcc));


    // Write out to this file
    let out_filename = matches.value_of("OUTPUT").unwrap();
    let output_path = Path::new(out_filename);
    let file = match File::create(&output_path) {
        Err(why) => panic!("couldn't create output file {}: {}",
                    output_path.display(),
                    why.description()),
        Ok(file) => file,
    };
    let mut write_buffer = BufWriter::new(file);


    psas_telemetry::serialize_message_to_csv(input_path, &mesg_fourcc, &mut write_buffer);
}
