use std::env::args;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let fname = args().nth(1).ok_or("Expected filename")?;
    let content = fs::read_to_string(fname)?;
    for c in content.chars() {
        match c {
            '>' => print!("{}", c),
            '<' => print!("{}", c),
            '+' => print!("{}", c),
            '-' => print!("{}", c),
            '.' => print!("{}", c),
            ',' => print!("{}", c),
            '[' => print!("{}", c),
            ']' => print!("{}", c),
            _ => {}
        }
    }
    print!("\n");

    Ok(())
}
