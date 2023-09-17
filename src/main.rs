extern crate flate2;

use flate2::Compression;
use std::io::{BufReader, copy};
use flate2::write::GzEncoder;
use std::time::Instant;
use std::env::args;
use std::fs::File;

/* 
use std::io::copy;
use std::io::BufReader; */

fn main() {
    // checking if user put three arguments
    // return error if true
    if args().len() != 3 {
        eprintln!("Usage: `run` `source file` `target file`");
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    
    // tracking time
    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!(
        "Source length: {:?}",
        input.get_ref().metadata().unwrap().len()    
    );
    println!(
        "Target length: {:?}",
        output.metadata().unwrap().len()
    );
    println!(
        "Elapsed time: {:?}",
        start.elapsed()
    );
}
