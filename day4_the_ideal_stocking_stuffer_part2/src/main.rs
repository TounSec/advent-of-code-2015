/*
Now find one that starts with six zeroe

Your puzzle answer was 9958218.
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

        if zeros >= 6 {
            println!("Solve : {}:{:?}", s, digest);
            break;
        }
    }

    Ok(())
}
