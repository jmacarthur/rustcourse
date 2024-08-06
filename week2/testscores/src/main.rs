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
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts : Vec<&str> = value.split(":").collect();
        let record = match parts.len() {
            1 => Self::NameOnly(String::from(parts[0])),
            2 => {
                let score : i64 = parts[1].parse()?;
                Self::NameAndNumber(String::from(parts[0]), score)
            },
            _ => panic!("Unimplemented")
        };
        Ok(record)
    }
}

fn read_records(filename: &str) -> Result<Vec<TestRecord>, Box<dyn std::error::Error>> {

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
    let fname_str = fname.as_str();
    let records = read_records(&fname_str);
    Ok(())
}
