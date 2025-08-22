/*
--- Part Two ---

The elves are also running low on ribbon. Ribbon is all the same width, so they only have to worry about the length they need to order, which they would again like to be exact.

The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face. Each present also requires a bow made out of ribbon as well; the feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present. Don't ask how they tie the bow, though; they'll never tell.

For example:

    A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
    A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.

How many total feet of ribbon should they order?

Your puzzle answer was 3812909.
*/

use std::{fs::File, io::Read, path::Path};
use num::{traits::Unsigned, FromPrimitive, Num};

type ResultMain = Result<(), Box<dyn std::error::Error>>;

#[derive(Clone, Copy)]
struct FeetRibbon<T>
where
    T: Num + Unsigned + FromPrimitive + Copy + Ord
{
    l: T,
    w: T,
    h: T
}

impl<T> FeetRibbon<T>
where 
    T: Num + Unsigned + FromPrimitive + Copy + Ord
{
    fn new(l: T, w: T, h: T) -> Self
    {
        FeetRibbon { l, w, h }
    }

    fn perimeter(&self) -> T
    {
        // (l² + w²)
        let n = T::from_u8(2).unwrap();
        let p1 = (self.l + self.w) * n;
        let p2 = (self.w + self.h) * n;
        let p3 = (self.h + self.l) * n;
        p1.min(p2).min(p3)
    }

    fn bow(&self) -> T
    {
        self.l * self.w * self.h
    }

    fn feet_ribon_sum(&self) -> T
    {
        self.perimeter() + self.bow()
    }
}

fn main() -> ResultMain
{
    let path = Path::new("day2.txt");
    let mut fd = File::open(path)?;
    let mut buf = Vec::new();

    fd.read_to_end(&mut buf)?;
    let str = str::from_utf8(&buf)?;
    let result: u32 = str.lines().map(|line| {
        let op = line.split('x').collect::<Vec<&str>>();

        println!("{:?}", op);

        let (l, w, h): (u16, u16, u16) = (
            op.get(0).unwrap().parse().unwrap(),
            op.get(1).unwrap().parse().unwrap(),
            op.get(2).unwrap().parse().unwrap()
        );

        let fr = FeetRibbon::new(l, w, h);
        
        let fr_perimeter = fr.perimeter();
        println!("Perimeter : {}", fr_perimeter);
        
        let fr_bow = fr.bow();
        println!("Bow : {}", fr_bow);

        let fr_sum = fr.feet_ribon_sum();
        println!("Sum : {}\n", fr_sum);

        fr_sum as u32

    }).sum();

    println!("Result : {}", result);
    
    Ok(())
}
