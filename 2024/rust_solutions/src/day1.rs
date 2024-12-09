use std::fs::File;
use std::io::{self, BufRead};

fn read_lines() -> io::Result<Vec<String>> {
    let file = File::open("input/day1.txt")?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn parse_lines_to_vecs(lines: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();
    for line in &lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let left = parts[0].parse::<i32>().unwrap();
        let right = parts[1].parse::<i32>().unwrap();
        left_numbers.push(left);
        right_numbers.push(right);
    }
    (left_numbers, right_numbers)
}

fn prep_input() -> (Vec<i32>, Vec<i32>) {
    let lines = read_lines().unwrap();
    parse_lines_to_vecs(lines)
}

fn count_occurences(vector: &[i32], value: &i32) -> i32 {
    vector.iter().filter(|&v| v == value).count() as i32
}

fn part1() -> i32 {
    let (mut left_numbers, mut right_numbers) = prep_input();
    left_numbers.sort();
    right_numbers.sort();
    left_numbers
        .iter()
        .enumerate()
        .map(|(index, value)| (value - right_numbers[index]).abs())
        .reduce(|acc, x| acc + x)
        .unwrap()
}

fn part2() -> i32 {
    let (left_numbers, right_numbers) = prep_input();
    left_numbers
        .iter()
        .map(|value| value * count_occurences(&right_numbers, value))
        .reduce(|acc, x| acc + x)
        .unwrap()
}

pub fn solve() {
    println!("Part 1 - {}", part1());
    println!("Part 2 - {}", part2());
}
