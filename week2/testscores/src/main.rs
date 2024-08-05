use std::env::args;
use std::error::Error;
use std::io::BufReader;
use std::io::BufRead;

use std::fs::File;

#[derive (Debug)]
enum TestRecord {
    NameAndNumber(String, i64),
    NameOnly(String)
}

impl TestRecord {
    fn new(name: String) -> TestRecord {
        TestRecord::NameOnly ( name )
    }
}

impl TryFrom<&str> for TestRecord {
    type Error = ();
    fn try_from(value: &str) -> Result<TestRecord, ()> {
        let parts : Vec<&str> = value.split(":").collect();
        let record = match parts.len() {
            1 => TestRecord::NameOnly(String::from(parts[0])),
            2 => {
                // Why can't we just use the '?' operator here?
                let score : i64 = match parts[1].parse() {
                    Ok(x) => x,
                    Err(x) => {panic!("{}", x)}
                };
                TestRecord::NameAndNumber(String::from(parts[0]), score)
            },
            _ => panic!("Unimplemented")
        };
        Ok(record)
    }
}

fn readRecords(filename: &str) -> Result<Vec<TestRecord>, Box<dyn std::error::Error>> {

    let mut records : Vec<TestRecord> = Vec::new();
    let f = File::open(filename)?;
    let f_reader = BufReader::new(f);
    for line in f_reader.lines() {
        match line {
            Ok(l) => {
                let record = TestRecord::try_from(&*l);
                println!("{:?}", record );
                records.push(record?);
            },
            Err(x) => { panic!("Failed to read line: {}", x) }
        }
    }
    Ok(records)
}

fn main() -> Result<(), Box<dyn Error>>
{
    println!("Hello, world!");
    let fname = args().nth(1).ok_or("Expected filename")?;
    let f = File::open(fname)?;
    let f_reader = BufReader::new(f);
    for line in f_reader.lines() {
        match line {
            Ok(l) => {
                let record = TestRecord::try_from(&*l);
                println!("{:?}", record );
            },
            Err(x) => { panic!("Failed to read line: {}", x) }
        }
    }
    Ok(())
}
