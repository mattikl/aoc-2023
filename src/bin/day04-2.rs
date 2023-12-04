use std::fs;

fn solve(data: &str) -> usize {
    let mut cards: Vec<usize> = Vec::new();
    for (card, line) in data.lines().enumerate() {
        let mut packs = line.split(&[':', '|']).skip(1);
        let winning: Vec<&str> = packs.next().unwrap().split_ascii_whitespace().collect();
        let mine: Vec<&str> = packs.next().unwrap().split_ascii_whitespace().collect();

        eprintln!("winning {:?} mine {:?}", winning, mine);
        let winning_numbers: Vec<u32> = mine
            .into_iter()
            .filter(|num| winning.contains(num))
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        let count = winning_numbers.len();
        eprintln!("card {card}, {count} matching");
        let pack_count = cards
            .clone()
            .into_iter()
            .filter(|c| *c == card)
            .collect::<Vec<usize>>()
            .len();
        for _i in 0..1 + pack_count {
            for j in card + 1..=(card + count) {
                cards.push(j);
            }
        }
        cards.push(card);
    }
    cards.len()
}

fn main() {
    println!("Hello, day 4!");
    let lines = fs::read_to_string("data/day-04.txt").unwrap();
    println!("the answer is {}", solve(&lines));
}

mod tests {
    use super::*;

    #[test]
    fn test_solve_example_data() {
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, solve(data));
    }
}
