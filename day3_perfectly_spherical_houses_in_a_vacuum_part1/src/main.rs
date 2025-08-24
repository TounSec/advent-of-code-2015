/*
--- Day 3: Perfectly Spherical Houses in a Vacuum ---

Santa is delivering presents to an infinite two-dimensional grid of houses.

He begins by delivering a present to the house at his starting location, and then an elf at the North Pole calls him via radio and tells him where to move next. Moves are always exactly one house to the north (^), south (v), east (>), or west (<). After each move, he delivers another present to the house at his new location.

However, the elf back at the north pole has had a little too much eggnog, and so his directions are a little off, and Santa ends up visiting some houses more than once. How many houses receive at least one present?

For example:

    > delivers presents to 2 houses: one at the starting location, and one to the east.
    ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
    ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.

Your puzzle answer was 2081.
*/

use std::{collections::HashSet, fs::File, io::Read, path::Path};
use plotters::prelude::*;

type ResultMain = Result<(), Box<dyn std::error::Error>>;

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn delta(&self) -> (i32, i32)
    {
        match self {
            Direction::North            =>              (0, 1),
            Direction::South            =>              (0, -1),
            Direction::East             =>              (1, 0),
            Direction::West             =>              (-1, 0)
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^'             =>              Ok(Direction::North),
            'v'             =>              Ok(Direction::South),
            '>'             =>              Ok(Direction::East),
            '<'             =>              Ok(Direction::West),
            _               =>              Err(())
        }
    }
}

struct Delivery {
    location: (i32, i32),
    visited: HashSet<(i32, i32)>,
    path: Vec<(i32, i32)>
}

impl Delivery {
    fn new() -> Self
    {
        let mut visited = HashSet::<(i32, i32)>::new();
        visited.insert((0, 0));

        Delivery { location: (0, 0), visited, path: vec![(0, 0)] }
    }

    fn next_location(&mut self, direction: Direction)
    {
        let (dx, dy) = direction.delta();
        
        self.location.0 += dx;
        self.location.1 += dy;

        self.visited.insert(self.location);
        self.path.push(self.location);
    }

    fn length(&self) -> usize
    {
        self.visited.len()
    }
}

fn main() -> ResultMain
{
    let path = Path::new("day3.txt");
    let mut fd = File::open(path)?;
    let mut buf = Vec::new();
    fd.read_to_end(&mut buf)?;

    let input = str::from_utf8(&buf)?;
    let mut delivery = Delivery::new();

    input.chars().for_each(|c| {
        if let Ok(dir) = Direction::try_from(c) {
            delivery.next_location(dir);
        }

        println!("Location : {:?}\nVisited : {:?}\n", delivery.location, delivery.visited);
    });

    let result = delivery.length();

    println!("Result : {}", result);

    if plotters_render(delivery).is_ok() {
        println!("santa_path.png generated");
    }

    Ok(())
}

fn plotters_render(delivery: Delivery) -> ResultMain
{
    let min_x = delivery.path.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = delivery.path.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = delivery.path.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = delivery.path.iter().map(|(_, y)| *y).max().unwrap();

    let root = BitMapBackend::new("santa_path.png", (900, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Santa Path", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_series_labels().draw()?;
    chart.draw_series(LineSeries::new(delivery.path.iter().copied(), RED))?;
    chart.draw_series(
        delivery.visited.iter().map(|(x, y)| {
            Circle::new((*x, *y), 2, GREEN.filled())
        })
    )?;
    chart.draw_series(std::iter::once(Circle::new(delivery.location, 4, BLUE.filled())))?;

    root.present()?;

    Ok(())
}
