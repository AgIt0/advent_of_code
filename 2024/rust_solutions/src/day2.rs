use std::fs::File;
use std::io::{self, BufRead};

fn lines_reader() -> io::Lines<io::BufReader<File>> {
    let file = File::open("input/day2.txt").unwrap();
    io::BufReader::new(file).lines()
}

fn check_requirements(result: i32, direction: i32) -> bool {
    if (1..=3).contains(&result.abs()) {
        if (direction == 1 && result > 0) || (direction == -1 && result < 0) {
            return false;
        }
    } else {
        return false;
    }

    true
}

fn check_safe_report(values: &Vec<&str>) -> bool {
    let mut direction = 0;

    for values in values.windows(2) {
        let a = values[0].parse::<i32>().unwrap();
        let b = values[1].parse::<i32>().unwrap();
        let result = a - b;
        if direction == 0 {
            if result < 0 {
                direction = 1;
            } else {
                direction = -1;
            }
        }
        if !check_requirements(result, direction) {
            return false;
        }
    }
    true
}

fn count_safe_reports(allow_single_bad_level: Option<bool>) -> i32 {
    let allow_single_bad_level = allow_single_bad_level.unwrap_or(false);

    let mut safe_reports = 0;
    for line in lines_reader() {
        let line = line.unwrap();
        let values = line.split_whitespace().collect::<Vec<&str>>();
        let all_safe = check_safe_report(&values);

        if !all_safe && allow_single_bad_level {
            for i in 0..values.len() {
                let mut values = values.clone();
                values.remove(i);
                if check_safe_report(&values) {
                    safe_reports += 1;
                    break;
                }
            }
        }

        if all_safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

fn part1() -> i32 {
    count_safe_reports(None)
}

fn part2() -> i32 {
    count_safe_reports(Some(true))
}

pub fn solve() {
    println!("Part 1 - {}", part1());
    println!("Part 2 - {}", part2());
}
