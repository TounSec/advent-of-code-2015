/*
--- Day 11: Corporate Policy ---

Santa's previous password expired, and he needs help choosing a new one.

To help him remember his new password after the old one expires, Santa has devised a method of coming up with a password based on the previous one. Corporate policy dictates that passwords must be exactly eight lowercase letters (for security reasons), so he finds his new password by incrementing his old password string repeatedly until it is valid.

Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on. Increase the rightmost letter one step; if it was z, it wraps around to a, and repeat with the next letter to the left until one doesn't wrap around.

Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some additional password requirements:

    Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
    Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing.
    Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.

For example:

    hijklmmn meets the first requirement (because it contains the straight hij) but fails the second requirement requirement (because it contains i and l).
    abbceffg meets the third requirement (because it repeats bb and ff) but fails the first requirement.
    abbcegjk fails the third requirement, because it only has one double letter (bb).
    The next password after abcdefgh is abcdffaa.
    The next password after ghijklmn is ghjaabcc, because you eventually skip all the passwords that start with ghi..., since i is not allowed.

Given Santa's current password (your puzzle input), what should his next password be?
*/

use std::collections::HashSet;

type ResultMain = Result<(), Box<dyn std::error::Error>>;

struct Password<'a>(&'a str);

impl<'a> Password<'a> {
    fn increase_letters(&self) -> String
    {
        let mut carry = true;
        let mut new_password = self.0
            .as_bytes()
            .iter()
            .rev()
            .map(|&b| {
                if carry {
                    if b == b'z' {
                        b'a'

                    } else {
                        carry = false;
                        b + 1
                    }

                } else {
                    b
                }

            })
            .collect::<Vec<u8>>();

        new_password.reverse();
        String::from_utf8(new_password).unwrap()
    }

    fn verify_rule1(&self) -> bool
    {
        self.0.as_bytes().windows(3).any(|w| (w[0] + 1) == w[1] && (w[1] + 1) == w[2])
    }

    fn verify_rule2(&self) -> bool
    {
        self.0.chars().all(|c| c != 'i' && c != 'o' && c != 'l')
    }

    fn verify_rule3(&self) -> bool
    {
        let bytes = self.0.as_bytes();
        let mut i = 0;
        let mut count_pairs = 0;

        while i + 1 < bytes.len() {
            if bytes[i] == bytes[i+1] {
                count_pairs += 1;
                i += 2;

            } else {
                i += 1;
            }
        }

        count_pairs >= 2
    }

    fn verify_rules(&self) -> bool
    {
        self.verify_rule1() && self.verify_rule2() && self.verify_rule3()
    }
}

fn main() -> ResultMain
{
    let mut password = String::from("hxbxwxba");

    loop {
        password = Password(&password).increase_letters();
        // println!("Current password : {}", password);
        
        if Password(&password).verify_rules() {
            println!("Part 1 password : {}", password);
            break;
        }
    }

    loop {
        password = Password(&password).increase_letters();

        if Password(&password).verify_rules() {
            println!("Part 2 password : {}", password);
            break;
        }
    }

    Ok(())
}
