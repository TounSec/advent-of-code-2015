/*
--- Part Two ---

The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa, to deliver presents with him.

Santa and Robo-Santa start at the same location (delivering two presents to the same starting house), then take turns moving based on instructions from the elf, who is eggnoggedly reading from the same script as the previous year.

This year, how many houses receive at least one present?

For example:

    ^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
    ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
    ^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.
*/

use std::{collections::HashSet, fs::File, io::Read, path::Path};
use plotters::prelude::*;

type ResultMain = Result<(), Box<dyn std::error::Error>>;

enum Direction {
    North,
    South,
    East,
    West
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
    santa_location: (i32, i32),
    robo_location:  (i32, i32),
    visited:        HashSet<(i32, i32)>,
    santa_path:     Vec<(i32, i32)>,
    robo_path:     Vec<(i32, i32)>
}

impl Delivery {
    fn new() -> Self
    {
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        Delivery {
            santa_location: (0, 0),
            robo_location: (0, 0),
            visited, santa_path: vec![(0, 0)],
            robo_path: vec![(0, 0)]
        }
    }

    fn santa_location(&mut self, santa_direction: Direction)
    {
        let (s_dx, s_dy) = santa_direction.delta();
        
        self.santa_location.0 += s_dx;
        self.santa_location.1 += s_dy;

        self.visited.insert(self.santa_location);

        self.santa_path.push(self.santa_location);
    }

    fn robo_location(&mut self, robo_direction: Direction)
    {
        let (s_dx, s_dy) = robo_direction.delta();
        
        self.robo_location.0 += s_dx;
        self.robo_location.1 += s_dy;

        self.visited.insert(self.robo_location);

        self.robo_path.push(self.robo_location);
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

    let santa_move = input.chars().step_by(2);
    let robo_move = input.chars().skip(1).step_by(2);
    
    santa_move.zip(robo_move).for_each(|(s, r)| {
        if let Ok(s_dir) = Direction::try_from(s) {
            delivery.santa_location(s_dir);
        }

        if let Ok(r_dir) = Direction::try_from(r) {
            delivery.robo_location(r_dir);
        }
    });

    let result = delivery.length();
    println!("Result : {}", result);

    if plotters_render(delivery).is_ok() {
        println!("santa_robo_path.png generated");
    }

    Ok(())
}

fn plotters_render(delivery: Delivery) -> ResultMain
{
    let all_points = delivery
        .santa_path
        .iter()
        .chain(delivery.robo_path.iter());

    let (mut min_x, mut max_x) = all_points.clone().map(|(x, _)| *x).min().zip(
        all_points.clone().map(|(x, _)| *x).max()

    ).unwrap();

    let (mut min_y, mut max_y) = delivery
        .santa_path.iter().chain(delivery.robo_path.iter())
        .map(|(_, y)| *y).min().zip(
            delivery.santa_path.iter().chain(delivery.robo_path.iter())
                .map(|(_, y)| *y).max()

        ).unwrap();

    if min_x == max_x { min_x -= 1; max_x += 1; }
    if min_y == max_y { min_y -= 1; max_y += 1; }

    let root = BitMapBackend::new("santa_robo_path.png", (900, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Santa + Robo Path", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.draw_series(LineSeries::new(
        delivery.santa_path.iter().copied(),
        &RED
    ))?;

    chart.draw_series(LineSeries::new(
        delivery.robo_path.iter().copied(),
        &RGBColor(255, 165, 0)
    ))?;

    chart.draw_series(
        delivery.visited.iter().map(|&(x, y)| Circle::new((x, y), 2, GREEN.filled()))
    )?;

    chart.draw_series(std::iter::once(Circle::new(delivery.santa_location, 4, BLUE.filled())))?;
    chart.draw_series(std::iter::once(Circle::new(delivery.robo_location, 4, MAGENTA.filled())))?;

    root.present()?;

    Ok(())
}
