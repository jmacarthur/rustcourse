use std::env::args;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
enum RawInstruction {
    IncPointer,
    DecPointer,
    IncByte,
    DecByte,
    Output,
    Input,
    JumpForward,
    JumpBackward,
}

impl RawInstruction {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Self::IncPointer),
            '<' => Some(Self::DecPointer),
            '+' => Some(Self::IncByte),
            '-' => Some(Self::DecByte),
            '.' => Some(Self::Output),
            ',' => Some(Self::Input),
            '[' => Some(Self::JumpForward),
            ']' => Some(Self::JumpBackward),
            _ => None,
        }
    }
    pub fn display(self: Self) -> String {
        match self {
            Self::IncPointer => "Increment current location",
            Self::DecPointer => "Decrement current location",
            Self::IncByte => "Increment byte at location",
            Self::DecByte => "Decrement byte at location",
            Self::Output => "Output byte at current location",
            Self::Input => "Read byte into current location",
            Self::JumpForward => "Start looping forward",
            Self::JumpBackward => "Start looping backward",
        }
        .to_string()
    }
}

#[derive(Debug)]
struct SourceInstruction {
    instruction: RawInstruction,
    col: usize,
    row: usize,
}

fn parse_source_file(filename: &str) -> Result<Vec<SourceInstruction>, Box<dyn std::error::Error>> {
    let f = File::open(filename)?;
    let f_reader = BufReader::new(f);
    let mut line_number: usize = 0;
    let mut instructions: Vec<SourceInstruction> = Vec::new();
    for line in f_reader.lines() {
        line_number += 1;
        let mut column: usize = 0;
        for c in line?.chars() {
            column += 1;
            match RawInstruction::from_char(c) {
                Some(i) => {
                    instructions.push(SourceInstruction {
                        instruction: i,
                        col: column,
                        row: line_number,
                    });
                }
                None => {}
            }
        }
    }
    println!("{} lines processed", line_number);
    Ok(instructions)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fname = args().nth(1).ok_or("Expected filename")?;
    let fname_str = fname.as_str();
    let source: Vec<SourceInstruction> = parse_source_file(&fname_str)?;

    for instruction in source {
        println!(
            "[{}:{}:{}] {}",
            fname_str,
            instruction.row,
            instruction.col,
            instruction.instruction.display()
        );
    }
    Ok(())
}
