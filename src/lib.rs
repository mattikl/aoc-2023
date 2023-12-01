use std::collections::HashMap;

// day 01 (part 2, `get_first_digit` was used for part one)
//
// Super verbose code as I was mostly refamiliarizing myself with Rust
// It would be a good exercise in Rust to write `get_first_digit_v2`
// and `get_last_digit_v2` as one function.

fn get_first_digit(line: &str) -> Option<char> {
    for c in line.chars() {
        if '1' <= c && c <= '9' {
            return Some(c);
        }
    }
    None
}

fn get_first_digit_v2(line: &str) -> Option<char> {
    let spelled_out_numbers = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    let mut tested_part = String::from("");
    for c in line.chars() {
        if '1' <= c && c <= '9' {
            return Some(c);
        }

        tested_part.push(c);
        let mut tested_word: std::str::Chars<'_> = tested_part.chars();

        // eprintln!("tested part {} {:?}", tested_part, tested_word.size_hint());
        loop {
            // eprintln!("tested word = {:?}", tested_word);
            if spelled_out_numbers.contains_key(tested_word.as_str()) {
                return Some(*spelled_out_numbers.get(tested_word.as_str()).unwrap());
            }
            tested_word.next(); // remove first letter

            if tested_word.size_hint().1.unwrap() < 3 {
                break;
            }
        }
    }
    None
}

fn get_last_digit_v2(line: &str) -> Option<char> {
    let spelled_out_numbers = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    let mut tested_part = String::from("");
    for c in line.chars().rev() {
        if '1' <= c && c <= '9' {
            return Some(c);
        }

        tested_part = c.to_string() + &tested_part;
        let mut tested_word: std::str::Chars<'_> = tested_part.chars();

        eprintln!("tested part {} {:?}", tested_part, tested_word.size_hint());
        loop {
            eprintln!("tested word = {:?}", tested_word);
            if spelled_out_numbers.contains_key(tested_word.as_str()) {
                return Some(*spelled_out_numbers.get(tested_word.as_str()).unwrap());
            }
            tested_word.next_back(); // remove last letter

            if tested_word.size_hint().1.unwrap() < 3 {
                break;
            }
        }
    }
    None
}

fn get_calibration_value(line: &str) -> u32 {
    let first = get_first_digit_v2(line);
    let last = get_last_digit_v2(line);

    eprintln!("{}{}", first.unwrap(), last.unwrap());

    return match (first, last) {
        (Some(m), Some(n)) => format!("{}{}", m, n).parse::<u32>().unwrap(),
        _ => 0,
    };
}

pub fn get_calibration_sum(lines: &str) -> u32 {
    let vals = lines.split("\n").map(get_calibration_value).into_iter();

    vals.sum()
}

mod tests {
    use super::*;

    #[test]
    fn test_first_digit() {
        let c = get_first_digit("1aaba2");
        assert_eq!(Some('1'), c);
        let c = get_first_digit("a901b");
        assert_eq!(Some('9'), c);
    }

    #[test]
    fn test_calibration_value() {
        let x = get_calibration_value("1aaba2");
        assert_eq!(12, x);
        let x = get_calibration_value("12");
        assert_eq!(12, x);
        let x = get_calibration_value("baa9abb");
        assert_eq!(99, x);
        let x = get_calibration_value("bar82baz");
        assert_eq!(82, x);
    }

    #[test]
    fn test_calibration_sum() {
        let sum = get_calibration_sum("baa9abb\nbar82baz");
        assert_eq!(99 + 82, sum);
    }

    #[test]
    fn test_get_first_digit_v2() {
        assert_eq!(Some('8'), get_first_digit_v2("eighthree"));
    }

    #[test]
    fn test_get_last_digit_v2() {
        assert_eq!(Some('3'), get_last_digit_v2("eighthree"));
    }
}
