/*
--- Day 6: Probably a Fire Hazard ---

Because your neighbors keep defeating you in the holiday house decorating contest year after year, you've decided to deploy one million lights in a 1000x1000 grid.

Furthermore, because you've been especially nice this year, Santa has mailed you instructions on how to display the ideal lighting configuration.

Lights in your grid are numbered from 0 to 999 in each direction; the lights at each corner are at 0,0, 0,999, 999,999, and 999,0. The instructions include whether to turn on, turn off, or toggle various inclusive ranges given as coordinate pairs. Each coordinate pair represents opposite corners of a rectangle, inclusive; a coordinate pair like 0,0 through 2,2 therefore refers to 9 lights in a 3x3 square. The lights all start turned off.

To defeat your neighbors this year, all you have to do is set up your lights by doing the instructions Santa sent you in order.

For example:

    turn on 0,0 through 999,999 would turn on (or leave on) every light.
    toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning off the ones that were on, and turning on the ones that were off.
    turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.

After following the instructions, how many lights are lit?

Your puzzle answer was 400410.
*/

use std::{collections::HashSet, fs::File, io::Read, path::Path};
use plotters::prelude::*;

type ResultMain = Result<(), Box<dyn std::error::Error>>;

enum Instructions {
    TurnOn,
    TurnOff,
    Toggle
}

impl Instructions {
    fn make_instruction(&self, lights: &mut Lights, x1: u32, y1: u32, x2: u32, y2: u32)
    {
        match self {
            Instructions::TurnOn             =>              lights.turn_on(x1, x2, y1, y2),
            Instructions::TurnOff            =>              lights.turn_off(x1, x2, y1, y2),
            Instructions::Toggle             =>              lights.toggle(x1, x2, y1, y2)
        }
    }
}

impl TryFrom<&str> for Instructions {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "turn on"               =>              Ok(Instructions::TurnOn),
            "turn off"              =>              Ok(Instructions::TurnOff),
            "toggle"                =>              Ok(Self::Toggle),
            _                       =>              Err(())
        }
    }
}

struct Lights {
    grid: HashSet<(u32, u32)>
}

impl Lights {
    fn new() -> Self
    {
        Lights { grid: HashSet::new() }
    }

    fn turn_on(&mut self, x1: u32, x2: u32, y1: u32, y2: u32)
    {
        for i in x1..=x2 {
            for j in y1..=y2 {
                self.grid.insert((i, j));
            }
        }
    }

    fn turn_off(&mut self, x1: u32, x2: u32, y1: u32, y2: u32)
    {
        for i in x1..=x2 {
            for j in y1..=y2 {
                self.grid.remove(&(i, j));
            }
        }
    }

    fn toggle(&mut self, x1: u32, x2: u32, y1: u32, y2: u32)
    {
        for i in x1..=x2 {
            for j in y1..=y2 {
                if !self.grid.insert((i, j)) {
                    self.grid.remove(&(i, j));
                }
            }
        }
    }
}

fn main() -> ResultMain
{
    let path = Path::new("day6.txt");
    let mut fd = File::open(path)?;
    let mut buf = Vec::new();
    fd.read_to_end(&mut buf)?;

    let input = str::from_utf8(&buf)?;
    let mut lights = Lights::new();

    input
        .lines()
        .for_each(|line| {
            let line_splited = line.split(' ').collect::<Vec<&str>>();
            // println!("Splited : {:?}", line_splited);

            let (instruction, (x1, y1), (x2, y2)) = match line_splited.as_slice() {
                ["toggle", from, "through", to]             =>              {
                    let instruction = "toggle";
                    let (x1, y1) = parse_coord(from);
                    let (x2, y2) = parse_coord(to);

                    (instruction, (x1, y1), (x2, y2))
                },
                ["turn", "on", from, "through", to]         =>              {
                    let instruction = "turn on";
                    let (x1, y1) = parse_coord(from);
                    let (x2, y2) = parse_coord(to);

                    (instruction, (x1, y1), (x2, y2))
                },
                ["turn", "off", from, "through", to]        =>              {
                    let instruction = "turn off";
                    let (x1, y1) = parse_coord(from);
                    let (x2, y2) = parse_coord(to);

                    (instruction, (x1, y1), (x2, y2))
                },
                _                                           =>              ("", (0, 0), (0, 0))
            };

            // println!("Instruction : {}\nFrom : [{}, {}]\nTo : [{}, {}]\n", instruction, x1, y1, x2, y2);

            if let Ok(ins) = Instructions::try_from(instruction) {
                ins.make_instruction(&mut lights, x1, y1, x2, y2);
            }
        });

    let result = lights.grid.len();
    println!("Result : {}", result);

    Ok(())
}

fn parse_coord(coord: &str) -> (u32, u32)
{
    let parts = coord
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    (parts[0], parts[1])
}
