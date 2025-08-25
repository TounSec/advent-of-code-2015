/*
--- Part Two ---

Now, let's go the other way. In addition to finding the number of characters of code, you should now encode each code representation as a new string and find the number of characters of the new encoded representation, including the surrounding double quotes.

For example:

    "" encodes to "\"\"", an increase from 2 characters to 6.
    "abc" encodes to "\"abc\"", an increase from 5 characters to 9.
    "aaa\"aaa" encodes to "\"aaa\\\"aaa\"", an increase from 10 characters to 16.
    "\x27" encodes to "\"\\x27\"", an increase from 6 characters to 11.

Your task is to find the total number of characters to represent the newly encoded strings minus the number of characters of code in each original string literal. For example, for the strings above, the total encoded length (6 + 9 + 16 + 11 = 42) minus the characters in the original code representation (23, just like in the first part of this puzzle) is 42 - 23 = 19.

*/

use std::{fs::File, io::Read, path::Path};

type ResultMain = Result<(), Box<dyn std::error::Error>>;

struct Literals {
    current: Vec<usize>,
    encoded: Vec<usize>
}

impl Literals {
    fn new() -> Self
    {
        Literals { current: Vec::new(), encoded: Vec::new() }
    }

    fn current(&mut self, line: &str)
    {
        self.current.push(line.len());
        println!("code: {}", line.len())
    }

    fn encoded(&mut self, line: &str)
    {
        let mut count = 2;

        for c in line.chars() {
            match c {
                '"' | '\\'          =>              count += 2,
                _                   =>              count += 1
            }
        }

        println!("Encoded : {}\n", count);
        self.encoded.push(count);
    }

    fn total_code_character(&self) -> u32
    {
        self.current.iter().map(|&v| v as u32).sum()
    }

    fn total_encoded_character(&self) -> u32
    {
        self.encoded.iter().map(|&v| v as u32).sum()
    }

    fn total_character(&self) -> u32
    {
        self.total_encoded_character() - self.total_code_character()
    }
}

fn main() -> ResultMain
{
    let path = Path::new("day8.txt");
    let mut fd = File::open(path)?;
    let mut buf = Vec::new();
    fd.read_to_end(&mut buf)?;

    let input = str::from_utf8(&buf)?;
    let mut literals = Literals::new();

    input.lines().for_each(|line| {
        literals.current(line);
        literals.encoded(line);
    });

    let result = literals.total_character();
    println!("Result : {}", result);

    Ok(())
}

