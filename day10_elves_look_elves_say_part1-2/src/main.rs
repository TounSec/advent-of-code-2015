/*
--- Day 10: Elves Look, Elves Say ---

Today, the Elves are playing a game called look-and-say. They take turns making sequences by reading aloud the previous sequence and using that reading as the next sequence. For example, 211 is read as "one two, two ones", which becomes 1221 (1 2, 2 1s).

Look-and-say sequences are generated iteratively, using the previous value as input for the next step. For each step, take the previous value, and replace each run of digits (like 111) with the number of digits (3) followed by the digit itself (1).

For example:

    1 becomes 11 (1 copy of digit 1).
    11 becomes 21 (2 copies of digit 1).
    21 becomes 1211 (one 2 followed by one 1).
    1211 becomes 111221 (one 1, one 2, and two 1s).
    111221 becomes 312211 (three 1s, two 2s, and one 1).

Starting with the digits in your puzzle input, apply this process 40 times. What is the length of the result?

Your puzzle input is 1321131112.
*/

type ResultMain = Result<(), Box<dyn std::error::Error>>;

fn main() -> ResultMain
{
    let input = "1321131112";

    let result_part1 = look_and_say(input, 40);
    println!("Result : {}", result_part1);

    let result_part2 = look_and_say(input, 50);
    println!("Result : {}", result_part2);

    Ok(())
}

fn look_and_say(input: &str, iterations: usize) -> usize
{
    let mut current = input.to_string();

    for _ in 0..iterations {
        let mut next = String::new();
        let mut chars = current.chars().peekable();

        while let Some(c) = chars.next() {
            let mut count = 1;

            while let Some(&next_c) = chars.peek() {
                if next_c == c {
                    count += 1;
                    chars.next();

                } else {
                    break;
                }
            }

            next.push_str(&count.to_string());
            next.push(c);
        }

        current = next;
    }

    current.len()
}
