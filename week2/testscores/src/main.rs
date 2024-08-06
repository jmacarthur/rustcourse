use std::collections::HashMap;
use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
enum TestRecord {
    NameAndNumber(String, i64),
    NameOnly(String),
}

#[derive(Debug)]
struct ScoreStruct {
    total_score: i64,
    number_of_results: i64,
    missed_tests: i64,
}

impl ScoreStruct {
    fn add_score(&mut self, score: i64) {
        self.total_score += score;
        self.number_of_results += 1;
    }
    fn add_missed_test(&mut self) {
        self.missed_tests += 1;
    }
}

impl Default for ScoreStruct {
    fn default() -> Self {
        Self {
            total_score: 0,
            number_of_results: 0,
            missed_tests: 0,
        }
    }
}

impl TryFrom<&str> for TestRecord {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(":").collect();
        return match parts.len() {
            1 => Ok(Self::NameOnly(String::from(parts[0]))),
            2 => Ok(Self::NameAndNumber(
                String::from(parts[0]),
                parts[1].parse()?,
            )),
            _ => Err("Only records with 1 or 2 fields are supported".into()),
        };
    }
}

fn read_records(filename: &str) -> Result<Vec<TestRecord>, Box<dyn std::error::Error>> {
    let mut records: Vec<TestRecord> = Vec::new();
    let f = File::open(filename)?;
    let f_reader = BufReader::new(f);
    for line in f_reader.lines() {
        match line {
            Ok(l) => {
                let record = TestRecord::try_from(&*l);
                records.push(record?);
            }
            Err(x) => {
                return Err(format!("Failed to read line: {}", x).into());
            }
        }
    }
    Ok(records)
}

fn pluralise(root: &str, plural_suffix: &str, singular_suffix: &str, quantity: i64) -> String {
    if quantity == 1 {
        root.to_owned()+singular_suffix
    } else {
        root.to_owned()+plural_suffix
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let fname = args().nth(1).ok_or("Expected filename")?;
    let fname_str = fname.as_str();
    let records = read_records(&fname_str);
    let mut hashmap: HashMap<String, ScoreStruct> = HashMap::new();
    print!("{:?}", records);
    for record in records? {
        match record {
            TestRecord::NameAndNumber(name, score) => {
                hashmap
                    .entry(name)
                    .or_insert(ScoreStruct::default())
                    .add_score(score);
            }
            TestRecord::NameOnly(name) => {
                hashmap
                    .entry(name)
                    .or_insert(ScoreStruct::default())
                    .add_missed_test();
            }
        }
    }
    print!("{:?}\n", hashmap);

    for (name, score_struct) in &hashmap {
        print!(
            "{} took {} {}, with a total score of {}. They missed {} {}.\n",
            name,
            score_struct.number_of_results,
            pluralise("test", "s", "", score_struct.number_of_results),
            score_struct.total_score,
            score_struct.missed_tests,
            pluralise("test", "s", "", score_struct.missed_tests),
        );
    }

    Ok(())
}
