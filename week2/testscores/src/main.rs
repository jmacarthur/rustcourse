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
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts : Vec<&str> = value.split(":").collect();
        let record = match parts.len() {
            1 => Self::NameOnly(String::from(parts[0])),
            2 => {
                // Why can't we just use the '?' operator here? Does it not return error?
                let score : i64 = match parts[1].parse() {
                    Ok(x) => x,
                    Err(x) => {panic!("{}", x)}
                };
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
                match record {
                    Ok(x) => { records.push(x); },
                    Err(x) => { panic!("Failed to parse line: {:?}", x);}
                };
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
    let f = File::open(fname_str)?;
    let f_reader = BufReader::new(f);
    let records = read_records(&fname_str);
    Ok(())
}
