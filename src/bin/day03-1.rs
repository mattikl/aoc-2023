use grid::*;
use std::fs;

fn solve(data: &str) -> u32 {
    let rows = data.lines().count();
    let cols = data.lines().next().unwrap().len();
    let mut grid: Grid<bool> = Grid::new(rows, cols);

    for (row, line) in data.lines().enumerate() {
        for (i, c) in line.char_indices() {
            if !(c == '.' || ('0' <= c && c <= '9')) {
                let mut col_iter = grid.iter_row_mut(row);
                let cell = col_iter.nth(i);
                *cell.unwrap() = true;
            }
        }
    }

    let mut sum = 0;
    for (n, line) in data.lines().enumerate() {
        let mut found = String::from("");
        let mut start = 0;

        // EXERCISE: use peek() to handle end of number once
        for (i, c) in line.char_indices() {
            if c.is_ascii_digit() {
                if found.len() == 0 {
                    start = i;
                }
                found.push(c);
            } else if found.len() > 0 {
                let mut adjacent = false;

                let left = if start > 0 { start - 1 } else { 0 };
                let top = if n > 0 { n - 1 } else { 0 };
                for ln in top..=n + 1 {
                    for j in left..=i {
                        if *grid.get(ln, j).unwrap_or(&false) {
                            adjacent = true;
                        }
                    }
                }

                if adjacent {
                    sum += found.parse::<u32>().unwrap();
                }
                found = String::from("");
            }
        }
        if found.len() > 0 {
            let mut adjacent = false;

            let left = if start > 0 { start - 1 } else { 0 };
            let top = if n > 0 { n - 1 } else { 0 };
            for ln in top..=n + 1 {
                for j in left..=rows {
                    if *grid.get(ln, j).unwrap_or(&false) {
                        adjacent = true;
                    }
                }
            }
            if adjacent {
                sum += found.parse::<u32>().unwrap();
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
.......58.
.^592.....
......755*
..........
.664+598..";
        assert_eq!(4361, solve(data));
    }
}
