use std::fs::File;
use std::io::{self, BufRead};

fn part1() -> i32 {
    let file = File::open("input/day2.txt").unwrap();
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    let mut safe_reports = 0;
    for line in lines {
        let mut direction = 0;
        let mut all_safe = true;

        for values in line
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .windows(2)
        {
            let a = values[0].parse::<i32>().unwrap();
            let b = values[1].parse::<i32>().unwrap();
            let result = a - b;

            if (1..=3).contains(&result.abs()) {
                if direction == 0 {
                    if result < 0 {
                        direction = 1;
                    } else {
                        direction = -1;
                    }
                } else if (direction == 1 && result > 0) || (direction == -1 && result < 0) {
                    all_safe = false;
                    break;
                }
            } else {
                all_safe = false;
                break;
            }
        }

        if all_safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

pub fn solve() {
    println!("Part 1 - {}", part1());
    // println!("Part 2 - {}", part2());
}
