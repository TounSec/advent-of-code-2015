/*
--- Day 16: Aunt Sue ---

Your Aunt Sue has given you a wonderful gift, and you'd like to send her a thank you card. However, there's a small problem: she signed it "From, Aunt Sue".

You have 500 Aunts named "Sue".

So, to avoid sending the card to the wrong person, you need to figure out which Aunt Sue (which you conveniently number 1 to 500, for sanity) gave you the gift. You open the present and, as luck would have it, good ol' Aunt Sue got you a My First Crime Scene Analysis Machine! Just what you wanted. Or needed, as the case may be.

The My First Crime Scene Analysis Machine (MFCSAM for short) can detect a few specific compounds in a given sample, as well as how many distinct kinds of those compounds there are. According to the instructions, these are what the MFCSAM can detect:

    children, by human DNA age analysis.
    cats. It doesn't differentiate individual breeds.
    Several seemingly random breeds of dog: samoyeds, pomeranians, akitas, and vizslas.
    goldfish. No other kinds of fish.
    trees, all in one group.
    cars, presumably by exhaust or gasoline or something.
    perfumes, which is handy, since many of your Aunts Sue wear a few kinds.

In fact, many of your Aunts Sue have many of these. You put the wrapping from the gift into the MFCSAM. It beeps inquisitively at you a few times and then prints out a message on ticker tape:

children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1

You make a list of the things you can remember about each Aunt Sue. Things missing from your list aren't zero - you simply don't remember the value.

What is the number of the Sue that got you the gift?
*/

use std::{collections::HashMap, fs::File, io::Read, path::Path};

type ResultMain = Result<(), Box<dyn std::error::Error>>;

const MFCSAM: Sue = Sue {
    children:       Some(3),
    cats:           Some(7),
    samoyeds:       Some(2),
    pomeranians:    Some(3),
    akitas:         Some(0),
    vizslas:        Some(0),
    goldfish:       Some(5),
    trees:          Some(3),
    cars:           Some(2),
    perfumes:       Some(1)
};

#[derive(Default, Debug)]
struct Sue {
    children:    Option<u8>,
    cats:        Option<u8>,
    samoyeds:    Option<u8>,
    pomeranians: Option<u8>,
    akitas:      Option<u8>,
    vizslas:     Option<u8>,
    goldfish:    Option<u8>,
    trees:       Option<u8>,
    cars:        Option<u8>,
    perfumes:    Option<u8>
}

impl Sue {
    fn from_properties(properties: &str) -> Self
    {
    let mut sue = Sue::default();

        for pair in properties.split(", ") {
            let mut it = pair.splitn(2, ": ");
            let key = it.next().unwrap().trim();
            let val: u8 = it.next().unwrap().trim().parse().unwrap();

            match key {
                "children"          =>              sue.children    = Some(val),
                "cats"              =>              sue.cats        = Some(val),
                "samoyeds"          =>              sue.samoyeds    = Some(val),
                "pomeranians"       =>              sue.pomeranians = Some(val),
                "akitas"            =>              sue.akitas      = Some(val),
                "vizslas"           =>              sue.vizslas     = Some(val),
                "goldfish"          =>              sue.goldfish    = Some(val),
                "trees"             =>              sue.trees       = Some(val),
                "cars"              =>              sue.cars        = Some(val),
                "perfumes"          =>              sue.perfumes    = Some(val),
                others              =>              panic!("unintented key : {others}")
            }
        }

        sue
    }
}

fn main() -> ResultMain
{
    let path = Path::new("day16.txt");
    let mut fd = File::open(path)?;
    let mut buf = String::new();
    fd.read_to_string(&mut buf)?;

    let mut sues: HashMap<u32, Sue> = HashMap::new();

    for line in buf.lines() {
        let parts: Vec<&str> = line.splitn(2, ": ").collect();
        let sue_part = parts[0];

        let properties = parts[1];
        let sue_num: u32 = sue_part.split_whitespace().nth(1).unwrap().parse()?;

        sues.insert(sue_num, Sue::from_properties(properties));
    }

    if let Some((k, _)) = sues.iter().find(|(_, v)| matches_part1(v)) {
        println!("Result part 1 : {}", k);

    } else {
        eprintln!("Any Sue found");
    }

    if let Some((k, _)) = sues.iter().find(|(_, v)| matches_part2(v)) {
        println!("Result part 2 : {}", k);

    } else {
        eprintln!("Any Sue found");
    }


    Ok(())
}

fn matches_part1(s: &Sue) -> bool
{
    s.children.map_or(true, |v| v == MFCSAM.children.unwrap()) &&
    s.cats.map_or(true, |v| v == MFCSAM.cats.unwrap()) &&
    s.samoyeds.map_or(true, |v| v == MFCSAM.samoyeds.unwrap()) &&
    s.pomeranians.map_or(true, |v| v == MFCSAM.pomeranians.unwrap()) &&
    s.akitas.map_or(true, |v| v == MFCSAM.akitas.unwrap()) &&
    s.vizslas.map_or(true, |v| v == MFCSAM.vizslas.unwrap()) &&
    s.goldfish.map_or(true, |v| v == MFCSAM.goldfish.unwrap()) &&
    s.trees.map_or(true, |v| v == MFCSAM.trees.unwrap()) &&
    s.cars.map_or(true, |v| v == MFCSAM.cars.unwrap()) &&
    s.perfumes.map_or(true, |v| v == MFCSAM.perfumes.unwrap())
}

fn matches_part2(s: &Sue ) -> bool
{
    let eq = |opt: Option<u8>, target:Option<u8>| opt.map_or(true, |v| v == target.unwrap());
    let gt = |opt: Option<u8>, target:Option<u8>| opt.map_or(true, |v| v > target.unwrap());
    let lt = |opt: Option<u8>, target:Option<u8>| opt.map_or(true, |v| v < target.unwrap());

    eq(s.children,          MFCSAM.children)    &&
    gt(s.cats,              MFCSAM.cats)        &&
    eq(s.samoyeds,          MFCSAM.samoyeds)    &&
    lt(s.pomeranians,       MFCSAM.pomeranians) &&
    eq(s.akitas,            MFCSAM.akitas)      &&
    eq(s.vizslas,           MFCSAM.vizslas)     &&
    lt(s.goldfish,          MFCSAM.goldfish)    &&
    gt(s.trees,             MFCSAM.trees)       &&
    eq(s.cars,              MFCSAM.cars)        &&
    eq(s.perfumes,          MFCSAM.perfumes)
}
