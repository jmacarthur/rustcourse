use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let fname = args().nth(1).ok_or("Expected filename")?;
    let f = File::open(fname)?;
    let f_reader = BufReader::new(f);

    let mut total: i32 = 0;

    for line in f_reader.lines() {
        let num: i32 = line.unwrap().trim().parse()?;
        println!("{}", num);
        total += num;
    }

    println!("The total is {total}");

    Ok(())
}
