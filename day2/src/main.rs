use std::env::args;

fn main() {
    let arg = args().skip(1).next().expect("expected an argument");

    match arg.as_str() {
        "one" => part_one(),
        "two" => part_two(),
        _ => eprintln!("invalid argument"),
    }
}

fn part_one() {
    let input = std::fs::read_to_string("./puzzle2/input").expect("failed to open input");

    let safe = input
        .lines()
        .map(|l| l.split(" "))
        .filter_map(|r| is_report_safe(r).then_some(()))
        .count();

    println!("{}", safe);
}

fn is_report_safe<'a>(report: impl Iterator<Item = &'a str>) -> bool {
    let mut previous_level: Option<i32> = None;
    let mut sign: Option<i32> = None;

    for level in report {
        let level = level.parse::<i32>().expect("invalid level");

        if let Some(prev_level) = previous_level {
            let diff = prev_level - level;
            let current_sign = diff.signum();

            if !(1..=3).contains(&diff.abs()) || sign.is_some_and(|s| current_sign != s) {
                return false;
            }

            if sign.is_none() {
                sign = Some(current_sign);
            }
        }

        previous_level = Some(level);
    }

    true
}

fn part_two() {
    let input = std::fs::read_to_string("./puzzle2/input").expect("failed to open input");

    let safe = input
        .lines()
        .map(|l| l.split(" "))
        .filter_map(|r| is_report_safe_with_tolerance(r.collect()).then_some(()))
        .count();

    println!("{}", safe);
}

fn is_report_safe_with_tolerance(report: Vec<&str>) -> bool {
    if is_report_safe(report.iter().map(|r| *r)) {
        return true;
    }

    for skip in 0..report.len() {
        if is_report_safe(
            report
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != skip)
                .map(|(_, r)| *r),
        ) {
            return true;
        }
    }

    false
}
