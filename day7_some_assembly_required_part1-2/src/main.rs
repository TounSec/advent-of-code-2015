/*
--- Day 7: Some Assembly Required ---

This year, Santa brought little Bobby Tables a set of wires and bitwise logic gates! Unfortunately, little Bobby is a little under the recommended age range, and he needs help assembling the circuit.

Each wire has an identifier (some lowercase letters) and can carry a 16-bit signal (a number from 0 to 65535). A signal is provided to each wire by a gate, another wire, or some specific value. Each wire can only get a signal from one source, but can provide its signal to multiple destinations. A gate provides no signal until all of its inputs have a signal.

The included instructions booklet describes how to connect the parts together: x AND y -> z means to connect wires x and y to an AND gate, and then connect its output to wire z.

For example:

    123 -> x means that the signal 123 is provided to wire x.
    x AND y -> z means that the bitwise AND of wire x and wire y is provided to wire z.
    p LSHIFT 2 -> q means that the value from wire p is left-shifted by 2 and then provided to wire q.
    NOT e -> f means that the bitwise complement of the value from wire e is provided to wire f.

Other possible gates include OR (bitwise OR) and RSHIFT (right-shift). If, for some reason, you'd like to emulate the circuit instead, almost all programming languages (for example, C, JavaScript, or Python) provide operators for these gates.

For example, here is a simple circuit:

123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i

After it is run, these are the signals on the wires:

d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456

In little Bobby's kit's instructions booklet (provided as your puzzle input), what signal is ultimately provided to wire a?

Your puzzle answer was 16076.

--- Part Two ---

Now, take the signal you got on wire a, override wire b to that signal, and reset the other wires (including wire a). What new signal is ultimately provided to wire a?

*/

use std::{collections::{HashMap, HashSet}, fs::File, io::Read, path::Path};
use plotters::prelude::*;

type ResultMain = Result<(), Box<dyn std::error::Error>>;

#[derive(Clone)]
enum Operand<'a> {
    Wire(&'a str),
    Value(u16)
}

impl<'a> From<&'a str> for Operand<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            val if val.parse::<u16>().is_ok()       =>          Operand::Value(val.parse().unwrap()),
            _                                       =>          Operand::Wire(value)
        }
    }
}

#[derive(Clone)]
enum Expr<'a> {
    Value(u16),
    Wire(&'a str),
    And(Operand<'a>, Operand<'a>),
    Or(Operand<'a>, Operand<'a>),
    RightShift(Operand<'a>, u16),
    LeftShift(Operand<'a>, u16),
    Not(Operand<'a>),
}

impl<'a> Expr<'a> {
    fn parse_instructions(instructions: &[&'a str]) -> (String, Self)
    {
        match instructions {
            [val, "->", target] if val.parse::<u16>().is_ok()               =>                  (target.to_string(), Expr::Value(val.parse().unwrap())),
            [wire, "->", target]                                            =>                  (target.to_string(), Expr::Wire(wire)),
            [a, "AND", b, "->", target]                                     =>                  (target.to_string(), Expr::And(Operand::from(*a), Operand::from(*b))),
            [a, "OR", b, "->", target]                                      =>                  (target.to_string(), Expr::Or(Operand::from(*a), Operand::from(*b))),
            [a, "RSHIFT", val, "->", target] if val.parse::<u16>().is_ok()  =>                  (target.to_string(), Expr::RightShift(Operand::from(*a), val.parse().unwrap())),
            [a, "LSHIFT", val, "->", target] if val.parse::<u16>().is_ok()  =>                  (target.to_string(), Expr::LeftShift(Operand::from(*a), val.parse().unwrap())),
            ["NOT", a, "->", target]                                        =>                  (target.to_string(), Expr::Not(Operand::from(*a))),
            _                                                               =>                  panic!("Unrecognized pattern : {}", instructions.concat())
        }
    }
}

struct Circuit<'a> {
    rules: HashMap<String, Expr<'a>>,
    result: HashMap<String, u16>,
}

impl<'a> Circuit<'a> {
    fn new() -> Self
    {
        Circuit { rules: HashMap::new(), result: HashMap::new() }
    }

    fn eval_operand(&mut self, op: Operand) -> u16
    {
        match op {
            Operand::Wire(w)            =>              self.eval(w),
            Operand::Value(v)           =>              v 
        }
    }

    fn eval(&mut self, wire: &str) -> u16
    {
        if let Some(&v) = self.result.get(wire) {
            return v;
        }

        let expr = self.rules.get(wire).unwrap().clone();


        let value = match expr {
            Expr::Value(x)                  =>                  x,
            Expr::Wire(w)                   =>                  self.eval(w),
            Expr::And(a, b)                 =>                  self.eval_operand(a) & self.eval_operand(b),
            Expr::Or(a, b)                  =>                  self.eval_operand(a) | self.eval_operand(b),
            Expr::RightShift(a, x)          =>                  self.eval_operand(a)  >> x,
            Expr::LeftShift(a, x)           =>                  self.eval_operand(a) << x,
            Expr::Not(a)                    =>                  !self.eval_operand(a)

        } & 0xFFFF;

        self.result.insert(wire.to_string(), value);
        value
    }
}

fn main() -> ResultMain
{
    let path = Path::new("day7.txt");
    let mut fd = File::open(path)?;
    let mut buf = Vec::new();
    fd.read_to_end(&mut buf)?;

    let input = str::from_utf8(&buf)?;
    let mut circuit = Circuit::new();

    // PART 1
    input.lines().for_each(|line| {
        let instructions: Vec<&str> = line.split_whitespace().collect();
        println!("Instructions : {:?}\n", instructions);

        let (target, expr) = Expr::parse_instructions(&instructions);
        circuit.rules.insert(target, expr);
    });

    let result_a = circuit.eval("a");
    println!("Signal on wire a : {}", result_a);

    // PART 2
    let mut circuit = Circuit::new();

    input.lines().for_each(|line| {
        let instructions: Vec<&str> = line.split_whitespace().collect();
        println!("Instructions : {:?}\n", instructions);

        let (target, expr) = Expr::parse_instructions(&instructions);

        if target == "b" {
            return;
        }

        circuit.rules.insert(target, expr);
    });

    circuit.rules.insert("b".to_string(), Expr::Value(result_a));
    let result_a = circuit.eval("a");
    println!("Final signal on wire a : {}", result_a);

    let (nodes, edges) = build_graph(&circuit.rules);

    let mut highlight_nodes = HashSet::new();
    let mut highlight_edges = HashSet::new();
    let mut visited = HashSet::new();
    collect_path("a", &circuit.rules, &mut highlight_nodes, &mut highlight_edges, &mut visited);

    plot_graph(&nodes, &edges, &highlight_nodes, &highlight_edges)?;


    Ok(())
}


fn build_graph<'a>(rules: &HashMap<String, Expr<'a>>) -> (Vec<(String, usize, usize)>, Vec<(String, String)>) {
    let mut edges = Vec::new();
    let mut dependents: HashMap<String, Vec<String>> = HashMap::new();

    // Collect dependencies
    for (target, expr) in rules {
        let mut sources = Vec::new();
        match expr {
            Expr::Wire(w) => sources.push(w.to_string()),
            Expr::And(a, b) | Expr::Or(a, b) => {
                if let Operand::Wire(w) = a { sources.push(w.to_string()); }
                if let Operand::Wire(w) = b { sources.push(w.to_string()); }
            }
            Expr::RightShift(a, _) | Expr::LeftShift(a, _) | Expr::Not(a) => {
                if let Operand::Wire(w) = a { sources.push(w.to_string()); }
            }
            _ => {}
        }
        for s in &sources {
            edges.push((s.clone(), target.clone()));
        }
        dependents.insert(target.clone(), sources);
    }

    fn dfs(node: &str, deps: &HashMap<String, Vec<String>>, memo: &mut HashMap<String, usize>) -> usize {
        if let Some(&lvl) = memo.get(node) {
            return lvl;
        }
        let lvl = if let Some(srcs) = deps.get(node) {
            if srcs.is_empty() {
                0
            } else {
                1 + srcs.iter().map(|s| dfs(s, deps, memo)).max().unwrap_or(0)
            }
        } else {
            0
        };
        memo.insert(node.to_string(), lvl);
        lvl
    }

    let mut levels: HashMap<String, usize> = HashMap::new();
    for k in rules.keys() {
        dfs(k, &dependents, &mut levels);
    }

    let mut per_level: HashMap<usize, Vec<String>> = HashMap::new();
    for (k, lvl) in &levels {
        per_level.entry(*lvl).or_default().push(k.clone());
    }

    let mut nodes = Vec::new();
    for (lvl, names) in per_level {
        for (idx, name) in names.iter().enumerate() {
            nodes.push((name.clone(), lvl, idx));
        }
    }

    (nodes, edges)
}

fn collect_path<'a>(
    target: &str,
    rules: &HashMap<String, Expr<'a>>,
    highlight_nodes: &mut HashSet<String>,
    highlight_edges: &mut HashSet<(String, String)>,
    visited: &mut HashSet<String>
) {
    if visited.contains(target) {
        return;
    }
    visited.insert(target.to_string());

    highlight_nodes.insert(target.to_string());

    if let Some(expr) = rules.get(target) {
        match expr {
            Expr::Wire(w) => {
                highlight_nodes.insert(w.to_string());
                highlight_edges.insert((w.to_string(), target.to_string()));
                collect_path(w, rules, highlight_nodes, highlight_edges, visited);
            }
            Expr::And(a, b) | Expr::Or(a, b) => {
                for op in [a, b] {
                    if let Operand::Wire(w) = op {
                        highlight_nodes.insert(w.to_string());
                        highlight_edges.insert((w.to_string(), target.to_string()));
                        collect_path(w, rules, highlight_nodes, highlight_edges, visited);
                    }
                }
            }
            Expr::RightShift(a, _) | Expr::LeftShift(a, _) | Expr::Not(a) => {
                if let Operand::Wire(w) = a {
                    highlight_nodes.insert(w.to_string());
                    highlight_edges.insert((w.to_string(), target.to_string()));
                    collect_path(w, rules, highlight_nodes, highlight_edges, visited);
                }
            }
            Expr::Value(_) => {}
        }
    }
}

fn plot_graph(
    nodes: &[(String, usize, usize)],
    edges: &[(String, String)],
    highlight_nodes: &std::collections::HashSet<String>,
    highlight_edges: &std::collections::HashSet<(String, String)>
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("circuit.png", (42000, 2000)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut positions: HashMap<String, (i32, i32)> = HashMap::new();
    for (name, level, idx) in nodes {
        let x = (*level as i32) * 200 + 100;
        let y = (*idx as i32) * 80 + 50;
        positions.insert(name.clone(), (x, y));

        let style = if highlight_nodes.contains(name) {
            ShapeStyle::from(&RED).filled()
        } else {
            ShapeStyle::from(&BLUE).filled()
        };

        root.draw(&Rectangle::new([(x - 30, y - 15), (x + 30, y + 15)], style))?;
        root.draw(&Text::new(name.clone(), (x, y), ("sans-serif", 15).into_font().color(&WHITE)))?;
    }

    for (src, dst) in edges {
        if let (Some((x1, y1)), Some((x2, y2))) = (positions.get(src), positions.get(dst)) {
            let style = if highlight_edges.contains(&(src.clone(), dst.clone())) {
                &RED
            } else {
                &BLACK
            };
            root.draw(&PathElement::new(vec![(*x1, *y1), (*x2, *y2)], style))?;
        }
    }

    root.present()?;
    Ok(())
}

