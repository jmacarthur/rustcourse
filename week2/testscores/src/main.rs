use std::env::args;
use std::error::Error;
use std::fs;

enum TestRecord {
    TupleKind(String, i64),
    String
}

fn main() -> Result<(), Box<dyn Error>>
{
    println!("Hello, world!");
    let fname = args().nth(1).ok_or("Expected filename")?;

    Ok(())

}
