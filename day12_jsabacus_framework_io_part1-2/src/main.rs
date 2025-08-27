/*
--- Day 12: JSAbacusFramework.io ---

Santa's Accounting-Elves need help balancing the books after a recent order. Unfortunately, their accounting software uses a peculiar storage format. That's where you come in.

They have a JSON document which contains a variety of things: arrays ([1,2,3]), objects ({"a":1, "b":2}), numbers, and strings. Your first job is to simply find all of the numbers throughout the document and add them together.

For example:

    [1,2,3] and {"a":2,"b":4} both have a sum of 6.
    [[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.
    {"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.
    [] and {} both have a sum of 0.

You will not encounter any strings containing numbers.

What is the sum of all numbers in the document?
*/

use std::{fs::File, io::Read, path::Path};
use serde_json::Value;

type ResultMain = Result<(), Box<dyn std::error::Error>>;

fn main() -> ResultMain
{
    let path = Path::new("day12.txt");
    let mut fd = File::open(path)?;
    let mut buf = Vec::new();
    fd.read_to_end(&mut buf)?;

    let input = str::from_utf8(&buf)?;

    let mut v = serde_json::from_str(input)?;
    let mut result = sum_json(&v);
    println!("Result part 1 : {}", result);

    v = serde_json::from_str(input)?;
    result = sum_objects(&v);
    println!("Result part 2 : {}", result);

    Ok(())
}

fn sum_json(v: &Value) -> i64
{
    match v {
        Value::Number(n)            =>              n.as_i64().unwrap_or(0),
        Value::Array(a)             =>              a.iter().map(sum_json).sum(),
        Value::Object(o)            =>              o.values().map(sum_json).sum(),
        _                           =>              0
    }
}

fn sum_objects(v: &Value) -> i64
{
    match v {
        Value::Number(n)            =>              n.as_i64().unwrap_or(0),
        Value::Array(a)             =>              a.iter().map(sum_objects).sum(),
        Value::Object(o)            =>              {
            if o.values().any(|val| val == "red") {
                0

            } else {
                o.values().map(sum_objects).sum()
            }
        },
        _                           =>              0
    }
}
