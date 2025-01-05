use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn lines_reader() -> io::Lines<io::BufReader<File>> {
    let file = File::open("input/day3.txt").unwrap();
    io::BufReader::new(file).lines()
}

fn extract_values(valid_string: &String) -> i32 {
    let clean_string = valid_string
        .trim_start_matches("mul(")
        .trim_end_matches(")");

    clean_string
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .product::<i32>()
}

pub fn part1() -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let result: i32 = lines_reader()
        .filter_map(|line| line.ok())
        .flat_map(|line| {
            let line_owned = line.to_string();
            re.find_iter(&line_owned)
                .map(|mat| mat.as_str().to_string())
                .collect::<Vec<_>>()
        })
        .map(|s| extract_values(&s))
        .sum();

    result
}

pub fn part2() -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don\'t\(\)").unwrap();
    let operations: Vec<String> = lines_reader()
        .filter_map(|line| line.ok())
        .flat_map(|line| {
            let line_owned = line.to_string();
            re.find_iter(&line_owned)
                .map(|mat| mat.as_str().to_string())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut filtered_operations = Vec::new();
    let mut skip = false;

    for operation in operations {
        match operation.as_str() {
            "don't()" => skip = true,
            "do()" => skip = false,
            _ if skip && operation.starts_with("mul") => continue, // skip mul operations after "don't()"
            _ => filtered_operations.push(operation),
        }
    }

    filtered_operations.iter().map(extract_values).sum()
}

pub fn solve() {
    println!("Day 3 Part 1 - {}", part1());
    println!("Day 3 Part 2 - {}", part2());
}
