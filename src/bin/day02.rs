use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Cubes {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Cubes>,
}

fn parse_game(line: &str) -> Game {
    let mut sets: Vec<Cubes> = Vec::new();
    let re = Regex::new(r"Game (\d+)").unwrap();
    let id: u32 = match re.captures(line) {
        Some(x) => x.get(1).map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
        None => 0,
    };
    let raw_sets = line.split(";");

    for raw in raw_sets {
        let re = Regex::new(r"(\d+) red").unwrap();
        let red: u32 = match re.captures(raw) {
            Some(x) => x.get(1).map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            None => 0,
        };
        let re = Regex::new(r"(\d+) green").unwrap();
        let green: u32 = match re.captures(raw) {
            Some(x) => x.get(1).map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            None => 0,
        };
        let re = Regex::new(r"(\d+) blue").unwrap();
        let blue: u32 = match re.captures(raw) {
            Some(x) => x.get(1).map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            None => 0,
        };
        sets.push(Cubes { red, green, blue });
    }

    Game { id, sets }
}

fn solve(bags: Cubes, data: &str) -> u32 {
    data.lines()
        .map(parse_game)
        .filter(|game| {
            eprintln!("game {:?}", game);
            for set in &game.sets {
                if bags.red < set.red || bags.green < set.green || bags.blue < set.blue {
                    eprintln!("filtering out game {}", game.id);
                    return false;
                }
            }
            true
        })
        .map(|game| game.id)
        .sum()
}

fn main() {
    println!("Hello, day 2!");
    let lines = fs::read_to_string("data/day-02.txt").unwrap();
    println!(
        "the answer is {}",
        solve(
            Cubes {
                red: 12,
                green: 13,
                blue: 14
            },
            &lines
        )
    );
}

mod tests {
    use super::*;

    #[test]
    fn test_solve_example_data() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(
            8,
            solve(
                Cubes {
                    red: 12,
                    green: 13,
                    blue: 14
                },
                data
            )
        );
    }
}
