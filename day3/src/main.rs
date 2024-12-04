use std::{env::args, fs};

use regex::Regex;

fn main() {
    let arg = args().skip(1).next().expect("expected an argument");

    match arg.as_str() {
        "one" => part_one(),
        "two" => part_two(),
        _ => eprintln!("invalid argument"),
    }
}

fn part_one() {
    let input = fs::read_to_string("input").expect("error reading input");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum: i32 = re
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| (a.parse::<i32>().expect("invalid input"), b.parse::<i32>().expect("invalid input")))
        .map(|(a, b)| a * b)
        .sum();

    println!("sum: {}", sum);
}

fn part_two() {
    let input = fs::read_to_string("input").expect("error reading input");

    let re = Regex::new(r"don't\(\)(.|\s)*?do\(\)").unwrap();

    let processed = re.replace_all(&input, "");

    let re = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();

    let sum: i32 = re
        .captures_iter(&processed)
        .map(|c| c.extract())
        .map(|(_, [a, b])| (a.parse::<i32>().expect("invalid input"), b.parse::<i32>().expect("invalid input")))
        .map(|(a, b)| a * b)
        .sum();

    println!("sum: {}", sum);
}
