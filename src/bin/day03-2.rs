use grid::*;
use regex::Regex;
use std::cmp::{max, min};
use std::fs;

#[derive(Debug)]
struct NumberMatch {
    start: usize,
    end: usize,
    value: u32,
}

fn solve(data: &str) -> u32 {
    let rows = data.lines().count();
    let cols = data.lines().next().unwrap().len();

    let mut sum = 0;
    let number_boundaries: Vec<Vec<NumberMatch>> = data
        .lines()
        .map(|line| {
            let mut found = String::from("");
            let mut start = 0;
            let mut matches: Vec<NumberMatch> = Vec::new();

            for (i, c) in line.char_indices() {
                if '0' <= c && c <= '9' {
                    if found.len() == 0 {
                        start = i;
                    }
                    found.push(c);
                } else if found.len() > 0 {
                    matches.push(NumberMatch {
                        start,
                        end: i - 1,
                        value: found.parse::<u32>().unwrap(),
                    });
                    found = String::from("");
                }
            }
            if found.len() > 0 {
                matches.push(NumberMatch {
                    start,
                    end: cols - 1,
                    value: found.parse::<u32>().unwrap(),
                });
            }
            matches
        })
        .collect();

    for (row, line) in data.lines().enumerate() {
        // quick code to get the result
        // EXERCISE: put all number matches in one vector and use the same comparison for each
        let prev_line = if row > 0 {
            number_boundaries.get(row - 1)
        } else {
            None
        };
        let current = number_boundaries.get(row).unwrap();
        let next_line = if row < rows - 1 {
            number_boundaries.get(row + 1)
        } else {
            None
        };

        for (i, c) in line.char_indices() {
            if c == '*' {
                let mut matches: Vec<&NumberMatch> = Vec::new();
                eprintln!("* found, {} {}", row, i);
                if let Some(prev) = prev_line {
                    eprintln!("prev matches {:?}", prev);
                    for m in prev {
                        if i.abs_diff(m.start) < 2 || i.abs_diff(m.end) < 2 {
                            matches.push(m);
                        }
                    }
                }
                eprintln!("This line matches {:?}", current);
                for m in current {
                    if m.end + 1 == i || i + 1 == m.start {
                        matches.push(m);
                    }
                }
                if let Some(next) = next_line {
                    eprintln!("next matches {:?}", next);
                    for m in next {
                        if i.abs_diff(m.start) < 2 || i.abs_diff(m.end) < 2 {
                            matches.push(m);
                        }
                    }
                }
                eprintln!("found matches {:?}", matches);
                if matches.len() == 2 {
                    sum += matches.get(0).unwrap().value * matches.get(1).unwrap().value;
                }
            }
        }
    }

    sum
}

fn main() {
    println!("Hello, day 3!");
    let lines = fs::read_to_string("data/day-03.txt").unwrap();
    println!("the answer is {}", solve(&lines));
}

mod tests {
    use super::*;

    #[test]
    fn test_solve_example_data() {
        let data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(467835, solve(data));
    }
}
