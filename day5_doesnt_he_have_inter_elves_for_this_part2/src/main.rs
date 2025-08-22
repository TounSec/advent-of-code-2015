/*
--- Part Two ---

Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or nice. None of the old rules apply, as they are all clearly ridiculous.

Now, a nice string is one with all of the following properties:

    It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
    It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.

For example:

    qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
    xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
    uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
    ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.

How many strings are nice under these new rules?

*/

use std::{fs::File, io::Read, path::Path};

type ResultMain = Result<(), Box<dyn std::error::Error>>;

fn main() -> ResultMain
{
    let path = Path::new("day5.txt");
    let mut fd = File::open(path)?;
    let mut buf = Vec::new();
    fd.read_to_end(&mut buf)?;

    let input = String::from_utf8(buf)?;
    let mut count = 0;

    input.lines().for_each(|line| {
        let pair_twice_letter = count_pair_twice_letter(line);
        println!("Pair Twice letters : {}", pair_twice_letter);

        let repeat_letter = count_repeat_letter(line);
        println!("Repeat letter : {}\n", repeat_letter);

        if eval_properties(pair_twice_letter, repeat_letter) {
            count += 1;
        }
    });

    println!("Nice strings number : {}", count);

    Ok(())
}

fn count_pair_twice_letter(line: &str) -> bool
{
    let bytes = line.as_bytes();

    bytes
        .windows(2)
        .enumerate()
        .any(|(i, w)| {
            (i + 2..bytes.len() - 1)
                .any(|j| w == &bytes[j..j + 2])
        })
}

fn count_repeat_letter(line: &str) -> bool
{
    line
        .as_bytes()
        .windows(3)
        .any(|w| w[0] == w[2])
}

fn eval_properties(twice_letter: bool, repeat_letter: bool) -> bool
{
    twice_letter && repeat_letter
}
