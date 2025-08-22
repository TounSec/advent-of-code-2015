/*
--- Part Two ---

Now, given the same instructions, find the position of the first character that causes him to enter the basement (floor -1). The first character in the instructions has position 1, the second character has position 2, and so on.

For example:

    ) causes him to enter the basement at character position 1.
    ()()) causes him to enter the basement at character position 5.

What is the position of the character that causes Santa to first enter the basement?

Your puzzle answer was 1795.
*/

use std::{fs::File, io::Read, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let path = Path::new("day1.txt");
    let mut fd = File::open(path)?;
    let mut buf = Vec::new();
    fd.read_to_end(&mut buf)?;

    let str = str::from_utf8(&buf)?;
    let mut floor = 0;
    
    for (index, char) in str.chars().enumerate() {
        match char {
            '('             =>              floor += 1,
            ')'             =>              floor -= 1,
            _               =>              {}
        }

        if floor == -1 {
            println!("{}", index+1);
            break;
        }
    }


    Ok(())
}
