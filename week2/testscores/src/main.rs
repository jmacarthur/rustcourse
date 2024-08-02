use std::env::args;
use std::error::Error;
use std::io::BufReader;
use std::io::BufRead;

use std::fs::File;

enum TestRecord {
    TupleKind(String, i64),
    String
}

impl TestRecord {
    fn new(name: String) {
        Self ( name )
    }
}

impl TryFrom<&str> for TestRecord {
    fn try_from(value: &str) -> Result<TestRecord, Box<dyn Error>> {
        let parts = value.split(":");
        let record = TestRecord::new(String::new("Hello world"));
        Ok(record)
    }
}

fn main() -> Result<(), Box<dyn Error>>
{
    println!("Hello, world!");
    let fname = args().nth(1).ok_or("Expected filename")?;
    let f = File::open(fname)?;
    let f_reader = BufReader::new(f);
    for line in f_reader.lines() {
        println!("{}", line?);
    }
    Ok(())

}
