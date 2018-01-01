# psas-telemetry

[![Build Status](https://travis-ci.org/natronics/rust-psas-telemetry.svg?branch=master)](https://travis-ci.org/natronics/rust-psas-telemetry)

Rust crate to read, write, and manuplate binary telemetry data messages in the PSAS rocket framework.


## PSAS Framework

[Portland State Aerospace Society](http://psas.pdx.edu) uses a custom binary format for sending telemetry messages between nodes of the flight computer and from the rocket to the ground. In theory, once layout and contents are defined messages should not change.

Since the messages are used in many places it's useful to have a single library that contains all the up-to-date message formats. This is a proof of concept to show how a central library would work in the Rust programming language.


## Code Generation

Since the format of the messages is tightly constrained, all of the code for working with individual messages can be generated from templates.

There is a template directory `templates` that have templates for writing both the main module code and a Rust source code file for each type of message that can define a `struct` for that message type.

### TOML Files

Each message is defined in a single [TOML](https://github.com/toml-lang/toml) file in the `definitions` directory. The order, type, and size of fixed-width binary data can be described here along with helpful documentation messages and information about how to convert a raw value into MKS units (if applicable).

### Build System 

The `build.rs` file reads in the definitions, parses the TOML, then sends that information to the template files in order to build to source code.

This will build all of the files under `src/messages`.


## Utilities

There are also some helpful logfile utilities provided.

### Logfile Stats

Read a logfile and report its statistics (size, message counts, etc.)

    $ cargo run --bin psas_logfile_stats FILE_TO_READ

for example:

    $ cargo run --bin psas_logfile_stats testdata/launch-12_short.log
    > Number of sequence numbers in file: 191
    > First sequence number: 0
    > Last sequence number: 190
    > Number of bytes read: 249997
    > Number of messages (excluding SEQN): 8755
    > Message counts:
    >  - FCFH: 16
    >  - VERS: 1
    >  - SEQE: 3
    >  - RNHH: 9
    >  - RNHP: 8726
