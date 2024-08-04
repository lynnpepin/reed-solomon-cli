extern crate reed_solomon;
use std::fs;
use reed_solomon::{Encoder, Decoder};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// If verbose is set, print extra messages while running.
    #[arg(short, long)]
    verbose: Option<bool>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Reed-Solomon encode an input file, to a given output file
    Encode {
        /// Input filename
        #[arg(short, long)]
        input: String,

        /// Output filename (will be overwritten)
        #[arg(short, long)]
        output: String,
    },

    /// Decode a Reed-Solomon encoded file to restore the original input.
    Decode {
        /// Input filename
        #[arg(short, long)]
        input: String,

        /// Output filename (will be overwritten)
        #[arg(short, long)]
        output: String,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Encode {input, output} => {
            // println!("todo; encode {input} to {output}");
            let encoder = Encoder::new(32);
            let data: Vec<u8> = match std::fs::read(input)  {
                Ok(d) => d,
                Err(err) => panic!("Error on read before encode: {err}"),
            };
            let encoded = encoder.encode(&data[..]);
            match std::fs::write(output, &encoded[..])  {
              Ok(_) => (),
              Err(err) => panic!("Error on writing after encode: {err}")
            };
            
        },
        Commands::Decode {input, output} => {
            let decoder = Decoder::new(32);
            let data: Vec<u8> = match std::fs::read(input)  {
                Ok(d) => d,
                Err(err) => panic!("Error on read before decode: {err}"),
            };
            match decoder.correct(data.as_slice(), None) {
                Ok(decoded) => match std::fs::write(output, decoded.data()) {
                  Ok(_) => (),
                  Err(err) => panic!("Error on writing after decode: {err}")
                },
                Err(err) => println!("Error on decoding: {err:?}")
            };
        }
    }
}
