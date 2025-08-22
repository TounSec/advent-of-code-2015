/*
--- Day 5: Doesn't He Have Intern-Elves For This? ---

Santa needs help figuring out which strings in his text file are naughty or nice.

A nice string is one with all of the following properties:

    It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
    It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
    It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

For example:

    ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
    aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
    jchzalrnumimnmhp is naughty because it has no double letter.
    haegwjzuvuyypxyu is naughty because it contains the string xy.
    dvszwmarrgswjxmb is naughty because it contains only one vowel.

How many strings are nice?

Your puzzle answer was 255.
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
        let vowels = count_vowels(line);
        println!("Number vowels : {}", vowels);

        let twice_letter = count_twice_letter(line);
        println!("Twice letters : {}", twice_letter);

        let not_contain = bool_not_contain(line);
        println!("Excluded Strings : {}\n", not_contain);

        if eval_properties(vowels, twice_letter, not_contain) {
            count += 1;
        }
    });

    println!("Nice strings number : {}", count);

    Ok(())
}

fn count_vowels(line: &str) -> usize
{
    line
        .chars()
        .filter(|c| "aeiou".contains(*c))
        .count()
}

fn count_twice_letter(line: &str) -> usize
{
    line
        .as_bytes()
        .windows(2)
        .filter(|w| w[0] == w[1])
        .count()
}

fn bool_not_contain(line: &str) -> bool
{
    line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy")
}

fn eval_properties(vowels: usize, twice_letter: usize, not_contain: bool) -> bool
{
    vowels >= 3 && twice_letter >= 1 && !not_contain
}
