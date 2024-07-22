use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::env::args;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let fname = args().nth(1).ok_or("Expected filename")?;
    let f = File::open(fname)?;
    let f_reader = BufReader::new(f);

    let mut total : i32 = 0;

    for line in f_reader.lines() {
	let num : i32 = line.unwrap().trim().parse()?;
	println!("{}", num);
	total += num;
    }

    println!("The total is {total}");

    Ok(())
}