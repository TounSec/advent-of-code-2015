/*
--- Part Two ---

You just finish implementing your winning light pattern when you realize you mistranslated Santa's message from Ancient Nordic Elvish.

The light grid you bought actually has individual brightness controls; each light can have a brightness of zero or more. The lights all start at zero.

The phrase turn on actually means that you should increase the brightness of those lights by 1.

The phrase turn off actually means that you should decrease the brightness of those lights by 1, to a minimum of zero.

The phrase toggle actually means that you should increase the brightness of those lights by 2.

What is the total brightness of all lights combined after following Santa's instructions?

For example:

    turn on 0,0 through 0,0 would increase the total brightness by 1.
    toggle 0,0 through 999,999 would increase the total brightness by 2000000.

Your puzzle answer was 15343601.
*/

use std::{collections::HashMap, fs::File, io::Read, path::Path};

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
            Instructions::TurnOn                =>                  lights.turn_on(x1, y1, x2, y2),
            Instructions::TurnOff               =>                  lights.turn_off(x1, y1, x2, y2),
            Instructions::Toggle                =>                  lights.toggle(x1, y1, x2, y2),
        }
    }
}

impl TryFrom<&str> for Instructions {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "turn on"               =>              Ok(Instructions::TurnOn),
            "turn off"              =>              Ok(Instructions::TurnOff),
            "toggle"                =>              Ok(Instructions::Toggle),
            _                       =>              Err(())
        }
    }
}

struct Lights {
    grid: HashMap<(u32, u32), u8>
}

impl Lights {
    fn new() -> Self
    {
       Lights { grid: HashMap::new() } 
    }

    fn turn_on(&mut self, x1: u32, y1: u32, x2: u32, y2: u32)
    {
        for x in x1..=x2 {
            for y in y1..=y2 {
                *self.grid.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    fn turn_off(&mut self, x1: u32, y1: u32, x2: u32, y2: u32)
    {
        for x in x1..=x2 {
            for y in y1..=y2 {
                let v = self.grid.entry((x, y)).or_insert(0);

                if *v > 0 {
                    *v -= 1;
                }
            }
        }
    }

    fn toggle(&mut self, x1: u32, y1: u32, x2: u32, y2: u32)
    {
        for x in x1..=x2 {
            for y in y1..=y2 {
                *self.grid.entry((x, y)).or_insert(0) += 2;
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

    input.lines().for_each(|line| {
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

        if let Ok(ins) = Instructions::try_from(instruction) {
            ins.make_instruction(&mut lights, x1, y1, x2, y2);
        }
    });

    let result: u32 = lights.grid.values().map(|&v| v as u32).sum(); 
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
