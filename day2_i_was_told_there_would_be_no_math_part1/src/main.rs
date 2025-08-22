/*
--- Day 2: I Was Told There Would Be No Math ---

The elves are running low on wrapping paper, and so they need to submit an order for more. They have a list of the dimensions (length l, width w, and height h) of each present, and only want to order exactly as much as they need.

Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating the required wrapping paper for each gift a little easier: find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The elves also need a little extra paper for each present: the area of the smallest side.

For example:

    A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.

All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?

Your puzzle answer was 1598415
*/

use std::{fs::File, io::Read, path::Path};
use num::{traits::Unsigned, Num, FromPrimitive};

type ResultMain = Result<(), Box<dyn std::error::Error>>;

#[derive(Clone, Copy)]
struct SquareFeet<T>
where
    T: Num + Unsigned + FromPrimitive + Ord + Copy
{
    l: T,
    w: T,
    h: T
}

impl<T> SquareFeet<T>
where 
    T: Num + Unsigned + FromPrimitive + Ord + Copy
{
    fn new(l: T, w: T, h: T) -> Self
    {
        SquareFeet { l, w, h }
    }

    fn calcul_square_feet(&self) -> T
    {
        // (2*l*w + 2*w*h + 2*h*l) + l*w
        let n = T::from_u8(2).unwrap();
        let lw = self.l * self.w;
        let wh = self.w * self.h;
        let hl = self.h * self.l;

        let surface = (n * lw) + (n * wh) + (n * hl);
        let slack = lw.min(wh).min(hl);

        surface + slack
    }
}

fn main() -> ResultMain
{
    let path = Path::new("day2.txt");
    let mut fd = File::open(path)?;
    let mut buf = Vec::new();

    fd.read_to_end(&mut buf)?;
    let str = str::from_utf8(&buf)?;
    let result: u64 = str.lines().map(|line| {
        let op = line.split('x').collect::<Vec<&str>>();

        println!("{:?}", op);

        let (l, w, h): (u16, u16, u16) = (
            op.get(0).unwrap().parse().unwrap(),
            op.get(1).unwrap().parse().unwrap(),
            op.get(2).unwrap().parse().unwrap()
        );

        let sf = SquareFeet::new(l, w, h);
        println!("{:?}", sf.calcul_square_feet());
        sf.calcul_square_feet() as u64

    }).sum();

    println!("Result : {}", result);
    
    Ok(())
}
