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

    println!("{args:?}");

    match &args.command {
        Commands::Encode {input, output} => {
            // println!("todo; encode {input} to {output}");
            let encoder = Encoder::new(8);
            let data: Vec<u8>= match std::fs::read(input)  {
                Ok(d) => d,
                Err(err) => panic!("Error on read: {err}"),
            };
            println!("todo: encode {data:?} to {output}");
            
        },
        Commands::Decode {input, output} => {
            println!("todo; decode {input} to {output}");
        }
    }
}
