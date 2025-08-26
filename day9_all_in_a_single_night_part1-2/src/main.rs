/*
--- Day 9: All in a Single Night ---

Every year, Santa manages to deliver all of his presents in a single night.

This year, however, he has some new locations to visit; his elves have provided him the distances between every pair of locations. He can start and end at any two (different) locations he wants, but he must visit each location exactly once. What is the shortest distance he can travel to achieve this?

For example, given the following distances:

London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141

The possible routes are therefore:

Dublin -> London -> Belfast = 982
London -> Dublin -> Belfast = 605
London -> Belfast -> Dublin = 659
Dublin -> Belfast -> London = 659
Belfast -> Dublin -> London = 605
Belfast -> London -> Dublin = 982

The shortest of these is London -> Dublin -> Belfast = 605, and so the answer is 605 in this example.

What is the distance of the shortest route?
*/

use std::{collections::{HashMap, HashSet}, fs::File, io::Read, path::Path};

type ResultMain = Result<(), Box<dyn std::error::Error>>;

struct Graph {
    pair_loc: HashMap<(String, String), u32>,
}

impl Graph {
    fn new() -> Self
    {
        Graph { pair_loc: HashMap::new() }
    }

    fn insert_pair_loc(&mut self, route_slice: &[&str])
    {
        let (from, to) = (route_slice[0], route_slice[2]);
        let dist = route_slice[4].parse::<u32>().unwrap();

        self.pair_loc.insert((from.to_string(), to.to_string()), dist);
        self.pair_loc.insert((to.to_string(), from.to_string()), dist);
    }

    fn cities(&self) -> Vec<String>
    {
        let mut set = HashSet::new();
        for (a, b) in self.pair_loc.keys() {
            set.insert(a.clone());
            set.insert(b.clone());
        }
        
        set.into_iter().collect()
    }

    fn path_distance(&self, path: &[String]) -> u32
    {
        path.windows(2)
            .map(|w| self.pair_loc[&(w[0].clone(), w[1].clone())])
            .sum()
    }

    fn shortest_path(&self) -> u32
    {
        let cities = self.cities();
        let mut used = vec![false; cities.len()];
        let mut current = Vec::new();
        let mut best = u32::MAX;

        fn backtrack(
            graph: &Graph,
            cities: &Vec<String>,
            used: &mut [bool],
            current: &mut Vec<String>,
            best: &mut u32
        )
        {
            if current.len() == cities.len() {
                let dist = graph.path_distance(current);
                if dist < *best { *best = dist; }

                return;
            }

            for i in 0..cities.len() {
                if !used[i] {
                    used[i] = true;
                    current.push(cities[i].clone());
                    backtrack(graph, cities, used, current, best);
                    current.pop();
                    used[i] = false;
                }
            }
        }

        backtrack(self, &cities, &mut used, &mut current, &mut best);
        best
    }

    fn longest_path(&self) -> u32
    {
        let cities = self.cities();
        let mut used = vec![false; cities.len()];
        let mut current: Vec<String> = Vec::new();
        let mut best = u32::MIN;

        fn backtrack(
            graph: &Graph,
            cities: &Vec<String>,
            used: &mut [bool],
            current: &mut Vec<String>,
            best: &mut u32
        )
        {
            if current.len() == cities.len() {
                let dist = graph.path_distance(current);
                if dist > *best { *best = dist; }

                return;
            }

            for i in 0..cities.len() {
                if !used[i] {
                    used[i] = true;
                    current.push(cities[i].clone());
                    backtrack(graph, cities, used, current, best);
                    current.pop();
                    used[i] = false;
                }
            }
        }

        backtrack(self, &cities, &mut used, &mut current, &mut best);
        best
    }
}

fn main() -> ResultMain
{
    let path = Path::new("day9.txt");
    let mut fd = File::open(path)?;
    let mut buf = Vec::new();
    fd.read_to_end(&mut buf)?;

    let input = str::from_utf8(&buf)?;
    let mut graph = Graph::new();

    input.lines().for_each(|line| {
        let vec_route = line.split_whitespace().collect::<Vec<&str>>();
        println!("{:?}", vec_route);

        graph.insert_pair_loc(&vec_route);
        println!("{:?}\n", graph.pair_loc);
        
    });

    let shortest = graph.shortest_path();
    println!("Shortest route = {}", shortest);

    let longest = graph.longest_path();
    println!("Longest route = {}", longest);
    
    Ok(())
}
