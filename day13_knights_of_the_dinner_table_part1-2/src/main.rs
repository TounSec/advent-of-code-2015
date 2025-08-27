/*
--- Day 13: Knights of the Dinner Table ---

In years past, the holiday feast with your family hasn't gone so well. Not everyone gets along! This year, you resolve, will be different. You're going to find the optimal seating arrangement and avoid all those awkward conversations.

You start by writing up a list of everyone invited and the amount their happiness would increase or decrease if they were to find themselves sitting next to each other person. You have a circular table that will be just big enough to fit everyone comfortably, and so each person will have exactly two neighbors.

For example, suppose you have only four attendees planned, and you calculate their potential happiness as follows:

Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.

Then, if you seat Alice next to David, Alice would lose 2 happiness units (because David talks so much), but David would gain 46 happiness units (because Alice is such a good listener), for a total change of 44.

If you continue around the table, you could then seat Bob next to Alice (Bob gains 83, Alice gains 54). Finally, seat Carol, who sits next to Bob (Carol gains 60, Bob loses 7) and David (Carol gains 55, David gains 41). The arrangement looks like this:

     +41 +46
+55   David    -2
Carol       Alice
+60    Bob    +54
     -7  +83

After trying every other seating arrangement in this hypothetical scenario, you find that this one is the most optimal, with a total change in happiness of 330.

What is the total change in happiness for the optimal seating arrangement of the actual guest list?
*/

use std::{collections::HashMap, fs::File, io::Read, path::Path};

type ResultMain = Result<(), Box<dyn std::error::Error>>;

struct Happiness {
    happiness: HashMap<String, HashMap<String, i32>>,
    people: Vec<String>
}

impl Happiness {
    fn new() -> Self
    {
        Happiness { happiness: HashMap::new(), people: Vec::new() }
    }

    fn add_self(&mut self, me: &str)
    {
        let me = me.to_string();

        if self.people.contains(&me) {
            return;
        }

        self.happiness.entry(me.clone()).or_default();
        
        for p in &self.people {
            self.happiness.get_mut(&me).unwrap().insert(p.clone(), 0);
        }

        for p in &self.people {
            self.happiness.entry(p.clone()).or_default().insert(me.clone(), 0);
        }

        self.people.push(me);
    }

    fn add_from_line(&mut self, line: &str)
    {
        let parts: Vec<&str> = line.trim_end_matches('.').split_whitespace().collect();

        let a = parts[0].to_string();
        let sign = if parts[2] == "gain" { 1 } else { -1 };
        let v: i32 = parts[3].parse().unwrap();
        let b = parts.last().unwrap().to_string();

        self.happiness.entry(a.clone()).or_default().insert(b.clone(), sign * v);

        if !self.people.contains(&a) {
            self.people.push(a);
        }

        if !self.people.contains(&b) {
            self.people.push(b);
        }
    }

    fn score_arrangement(&self, order: &[usize]) -> i32
    {
        let n = order.len();
        let mut total = 0;

        for i in 0..n {
            let a = &self.people[order[i]];
            let b = &self.people[order[(i + 1) % n]];

            let ab = self.happiness.get(a).and_then(|m| m.get(b)).copied().unwrap_or(0);
            let ba = self.happiness.get(b).and_then(|m| m.get(a)).copied().unwrap_or(0);

            total += ab + ba;
        }

        total
    }

    fn best_total_round(&self) -> i32
    {
        let n = self.people.len();
        if n == 0 { return 0; }

        let mut idx: Vec<usize> = (0..n).collect();
        let mut best = i32::MIN;

        fn rec<F: FnMut(&[usize])>(k: usize, a: &mut [usize], f: &mut F)
        {
            if k == a.len() {
                f(a);

                return;
            }

            for i in k..a.len() {
                a.swap(k, i);
                rec(k + 1, a, f);
                a.swap(k, i);
            }
        }

        rec(1, &mut idx, &mut |perm| {
            let s = self.score_arrangement(perm);
            if s > best { best = s; }
        });

        best
    }
}

fn main() -> ResultMain
{
    let path = Path::new("day13.txt");
    let mut fd = File::open(path)?;
    let mut buf = String::new();
    fd.read_to_string(&mut buf)?;

    let mut happiness = Happiness::new();

    for line in buf.lines().filter(|l| !l.trim().is_empty()) {
        happiness.add_from_line(line);
    }

    let result = happiness.best_total_round();
    println!("Result part 1 : {}", result);

    happiness.add_self("TounSec");
    let result2 = happiness.best_total_round();
    println!("Result part 2 : {}", result2);

    Ok(())
}
