use crate::days::run_day;
use crate::utils::string::{reverse_string, substring_from_index};

pub fn run() {
    let day_number: i32 = 1;

    let part1_expected = 55477;
    let part1_filename = "input.txt";

    let part2_expected = 54431;
    let part2_filename = "input.txt";

    run_day(day_number, run_part_1, part1_expected, part1_filename, run_part_2, part2_expected, part2_filename);
}

fn run_part_1(lines: Vec<String>) -> i32 {
    let mut sum = 0;

    for (_index, line) in lines.iter().enumerate() {
        let first = first_digit(line, false);
        let last = last_digit(line, false);

        let result = format!("{}{}", first, last);

        sum += result.parse::<i32>().unwrap_or(0);
    }

    sum
}

fn run_part_2(lines: Vec<String>) -> i32 {
    let mut sum = 0;

    for (_index, line) in lines.iter().enumerate() {
        let first = first_digit(line, true);
        let last = last_digit(line, true);

        let result = format!("{}{}", first, last);

        sum += result.parse::<i32>().unwrap_or(0);
    }

    sum
}

fn first_digit(input: &str, allow_string: bool) -> i32 {

    // Iterate over the characters in the string
    for (index, ch) in input.char_indices() {
        // Check if the character is a digit
        if ch.is_digit(10) {
            return ch.to_digit(10).unwrap_or(0) as i32;  // Convert the digit to an integer
        }

        if allow_string {
            match ch {
                'o' => if substring_from_index(input, index).starts_with("one") { return 1 },
                't' =>
                    if substring_from_index(input, index).starts_with("two") { return 2 } else if substring_from_index(input, index).starts_with("three") { return 3 },
                'f' => if substring_from_index(input, index).starts_with("four") { return 4 } else if substring_from_index(input, index).starts_with("five") { return 5 },
                's' => if substring_from_index(input, index).starts_with("six") { return 6 } else if substring_from_index(input, index).starts_with("seven") { return 7 },
                'e' => if substring_from_index(input, index).starts_with("eight") { return 8 },
                'n' => if substring_from_index(input, index).starts_with("nine") { return 9 },
                _ => {}
            }
        }
    }
    0  // Return 0 if no digit is found
}

fn last_digit(input: &str, allow_string: bool) -> i32 {
    if !allow_string {
        let reversed = reverse_string(input);
        for ch in reversed.chars() {
            if ch.is_digit(10) {
                return ch.to_digit(10).unwrap_or(0) as i32;
            }
        }
    }

    let mut last = 0;
    for (index, ch) in input.char_indices() {
        // Check if the character is a digit
        if ch.is_digit(10) {
            last = ch.to_digit(10).unwrap_or(0) as i32;  // Convert the digit to an integer
        }

        match ch {
            'o' => if substring_from_index(input, index).starts_with("one") { last = 1 },
            't' =>
                if substring_from_index(input, index).starts_with("two") { last = 2 } else if substring_from_index(input, index).starts_with("three") { last = 3 },
            'f' => if substring_from_index(input, index).starts_with("four") { last = 4 } else if substring_from_index(input, index).starts_with("five") { last = 5 },
            's' => if substring_from_index(input, index).starts_with("six") { last = 6 } else if substring_from_index(input, index).starts_with("seven") { last = 7 },
            'e' => if substring_from_index(input, index).starts_with("eight") { last = 8 },
            'n' => if substring_from_index(input, index).starts_with("nine") { last = 9 },
            _ => {}
        }
    }

    last
}
