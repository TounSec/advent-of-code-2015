/*
--- Day 4: The Ideal Stocking Stuffer ---

Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the economically forward-thinking little girls and boys.

To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes. The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal. To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.

For example:

    If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
    If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....

Your puzzle input is iwrupvqb.

Your puzzle answer was 346386.
*/

type ResultMain = Result<(), Box<dyn std::error::Error>>;

fn main() -> ResultMain
{
    let secret_key = String::from("iwrupvqb");

    for i in 1..u32::MAX {
        let s = format!("{}{}", secret_key, i);
        println!("Updated secret key : {}", &s);

        let digest = md5::compute(&s);
        println!("MD5 : {:?}\n", &digest);

        let hex = format!("{:x}", digest);
        let zeros = hex.chars().take_while(|&c| c == '0').count();

        if zeros >= 5 {
            println!("Solve : {}:{:?}", s, digest);
            break;
        }
    }

    Ok(())
}
