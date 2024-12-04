use std::env::args;

fn main() {
    let arg = args().skip(1).next().expect("expected an argument");

    match arg.as_str() {
        "one" => part_one(),
        "two" => part_two(),
        _ => eprintln!("invalid argument"),
    }
}

fn read_file() -> (Vec<i32>, Vec<i32>) {
    let input = std::fs::read_to_string("./puzzle1/input").expect("failed to open input");

    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let (l, r) = line.split_once("   ").expect("invalid input");
        let l = l.parse::<i32>().expect("invalid input");
        let r = r.parse::<i32>().expect("invalid input");

        left.push(l);
        right.push(r);
    }

    (left, right)
}

fn part_one() {
    let (mut left, mut right) = read_file();

    left.sort();
    right.sort();

    let sum: i32 = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("{}", sum);
}

fn part_two() {
    let (left, right) = read_file();

    let sum: i32 = left
        .into_iter()
        .map(|l| l * right.iter().filter(|r| **r == l).count() as i32)
        .sum();

    println!("{}", sum);
}
