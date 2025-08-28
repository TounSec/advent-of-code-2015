/*
--- Day 14: Reindeer Olympics ---

This year is the Reindeer Olympics! Reindeer can fly at high speeds, but must rest occasionally to recover their energy. Santa would like to know which of his reindeer is fastest, and so he has them race.

Reindeer can only either be flying (always at their top speed) or resting (not moving at all), and always spend whole seconds in either state.

For example, suppose you have the following Reindeer:

    Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
    Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.

After one second, Comet has gone 14 km, while Dancer has gone 16 km. After ten seconds, Comet has gone 140 km, while Dancer has gone 160 km. On the eleventh second, Comet begins resting (staying at 140 km), and Dancer continues on for a total distance of 176 km. On the 12th second, both reindeer are resting. They continue to rest until the 138th second, when Comet flies for another ten seconds. On the 174th second, Dancer flies for another 11 seconds.

In this example, after the 1000th second, both reindeer are resting, and Comet is in the lead at 1120 km (poor Dancer has only gotten 1056 km by that point). So, in this situation, Comet would win (if the race ended at 1000 seconds).

Given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds, what distance has the winning reindeer traveled?
*/

use std::{fs::File, io::Read, path::Path};

type ResultMain = Result<(), Box<dyn std::error::Error>>;

struct Reindeer {
    name: String,
    km_s: u32,
    t_fly: u32,
    t_rest: u32
}

impl Reindeer {
    fn new(name: &str, km_s: u32, t_fly: u32, t_rest: u32) -> Self
    {
        Reindeer {
            name: name.to_string(),
            km_s,
            t_fly,
            t_rest
        }
    }

    fn reindeer_distance_traveled(&self, time: u32) -> u32
    {
        let cycle = self.t_fly + self.t_rest;
        let full_cycles = time / cycle;
        let remaining = time % cycle;

        full_cycles * self.t_fly * self.km_s + remaining.min(self.t_fly) * self.km_s
    }
}

struct Stats(Vec<Reindeer>);

impl Stats {
    fn new() -> Self
    {
        Stats(Vec::new())
    }

    fn push_reindeer(&mut self, name: &str, km_s: u32, t_fly: u32, t_rest: u32)
    {
        let new_reindeer = Reindeer::new(name, km_s, t_fly, t_rest);

        self.0.push(new_reindeer);
    }

    fn best_distance(&self, time: u32) -> u32
    {
        self.0
            .iter()
            .map(|r| r.reindeer_distance_traveled(time))
            .max()
            .unwrap_or(0)
    }

    fn points_after(&self, time: u32) -> u32
    {
        let mut scores = vec![0; self.0.len()];

        for t in 1..=time {
            let distances: Vec<u32> = self.0
                .iter()
                .map(|r| r.reindeer_distance_traveled(t))
                .collect();

            let max_dist = *distances.iter().max().unwrap();

            for (i, &d) in distances.iter().enumerate() {
                if d == max_dist {
                    scores[i] += 1;
                }
            }
        }

        *scores.iter().max().unwrap()
    }
}

fn main() -> ResultMain
{
    let path = Path::new("day14.txt");
    let mut fd = File::open(path)?;
    let mut buf = String::new();
    fd.read_to_string(&mut buf)?;

    let mut stats = Stats::new();

    for line in buf.lines().filter(|l| !l.trim().is_empty()) {
        let splited: Vec<&str> = line.trim_end_matches('.').split_whitespace().collect();

        let name = splited[0];
        let km_s = splited[3].parse().unwrap();
        let t_fly = splited[6].parse().unwrap();
        let t_rest = splited[13].parse().unwrap();

        stats.push_reindeer(name, km_s, t_fly, t_rest);
    }

    let result = stats.best_distance(2503);
    println!("Result part 1 : {}", result);

    let result2 = stats.points_after(2503);
    println!("Result part 2 : {}", result2);


    Ok(())
}
