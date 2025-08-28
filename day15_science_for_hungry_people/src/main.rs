/*
--- Day 15: Science for Hungry People ---

Today, you set out on the task of perfecting your milk-dunking cookie recipe. All you have to do is find the right balance of ingredients.

Your recipe leaves room for exactly 100 teaspoons of ingredients. You make a list of the remaining ingredients you could use to finish the recipe (your puzzle input) and their properties per teaspoon:

    capacity (how well it helps the cookie absorb milk)
    durability (how well it keeps the cookie intact when full of milk)
    flavor (how tasty it makes the cookie)
    texture (how it improves the feel of the cookie)
    calories (how many calories it adds to the cookie)

You can only measure ingredients in whole-teaspoon amounts accurately, and you have to be accurate so you can reproduce your results in the future. The total score of a cookie can be found by adding up each of the properties (negative totals become 0) and then multiplying together everything except calories.

For instance, suppose you have these two ingredients:

Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3

Then, choosing to use 44 teaspoons of butterscotch and 56 teaspoons of cinnamon (because the amounts of each ingredient must add up to 100) would result in a cookie with the following properties:

    A capacity of 44*-1 + 56*2 = 68
    A durability of 44*-2 + 56*3 = 80
    A flavor of 44*6 + 56*-2 = 152
    A texture of 44*3 + 56*-1 = 76

Multiplying these together (68 * 80 * 152 * 76, ignoring calories for now) results in a total score of 62842880, which happens to be the best score possible given these ingredients. If any properties had produced a negative total, it would have instead become zero, causing the whole score to multiply to zero.

Given the ingredients in your kitchen and their properties, what is the total score of the highest-scoring cookie you can make?

*/

use std::{fs::File, io::Read, path::Path};

type ResultMain = Result<(), Box<dyn std::error::Error>>;

#[derive(Default)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}

impl Ingredient {
    fn new(name: &str, capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Self
    {
        Ingredient { name: name.to_string(), capacity, durability, flavor, texture, calories }
    }

    fn from_line(line: &str) -> Self
    {
        let clean = line.replace(",", "");
        let p: Vec<&str> = clean.split_whitespace().collect();
        let name = p[0].trim_end_matches(':');
        let capacity: i32 = p[2].parse().unwrap();
        let durability: i32 = p[4].parse().unwrap();
        let flavor: i32 = p[6].parse().unwrap();
        let texture: i32 = p[8].parse().unwrap();
        let calories: i32 = p[10].parse().unwrap();

        Ingredient::new(name, capacity, durability, flavor, texture, calories)
    }

    fn apply(&self, teaspoons: i32) -> (i32, i32, i32, i32, i32)
    {
        (
            self.capacity * teaspoons,
            self.durability * teaspoons,
            self.flavor * teaspoons,
            self.texture * teaspoons,
            self.calories * teaspoons
        )
    }
}

fn main() -> ResultMain
{
    let path = Path::new("day15.txt");
    let mut fd = File::open(path)?;
    let mut buf = String::new();
    fd.read_to_string(&mut buf)?;

    let mut ingredients: Vec<Ingredient> = Vec::new();

    buf.lines().for_each(|line| {
        let ing = Ingredient::from_line(line);
        ingredients.push(ing);
    });

    let mut best1 = 0;
    let mut best2 = 0;

    fn search(
        idx: usize,
        remaining: i32,
        ingredients: &[Ingredient],
        teaspoons: &mut Vec<i32>,
        best1: &mut i32,
        best2: &mut i32
    )
    {
        if idx == ingredients.len() - 1 {
            teaspoons[idx] = remaining;

            let (cap, dur, fla, tex, cal) = teaspoons
                .iter()
                .enumerate()
                .map(|(i, &t)| ingredients[i].apply(t))
                .fold((0, 0, 0, 0, 0), |acc, v| {
                    (
                        acc.0 + v.0,
                        acc.1 + v.1,
                        acc.2 + v.2,
                        acc.3 + v.3,
                        acc.4 + v.4
                    )
                });

            let score = cap.max(0) * dur.max(0) * fla.max(0) * tex.max(0);

            if score > *best1 {
                *best1 = score;
            }

            if cal == 500 && score > *best2 {
                *best2 = score;
            }

        } else {
            for t in 0..=remaining {
                teaspoons[idx] = t;
                search(idx + 1, remaining - t, ingredients, teaspoons, best1, best2);
            }
        }
    }

    let mut teaspoons = vec![0; ingredients.len()];
    search(0, 100, &ingredients, &mut teaspoons, &mut best1, &mut best2);

    println!("Best score part 1 : {}", best1);
    println!("Best score part 2 : {}", best2);

    Ok(())
}
