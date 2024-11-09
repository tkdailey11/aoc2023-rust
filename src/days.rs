use std::time::Instant;
use crate::utils::file::read_lines_to_vec;

pub(crate) mod day1;
pub(crate) mod template;
pub(crate) mod day2;

pub fn run_day(day_number: i32,
               part1: fn(Vec<String>) -> i32,
               part1_expected: i32,
               part1_filename: &str,
               part2: fn(Vec<String>) -> i32,
               part2_expected: i32,
               part2_filename: &str) {

    println!("Day{}:", day_number);
    run_day_part(day_number, part1_filename, part1, part1_expected);
    run_day_part(day_number, part2_filename, part2, part2_expected);
}

fn run_day_part(day_number: i32, filename: &str, func: fn(Vec<String>) -> i32, expected: i32) {
    let file_path = format!("input/day{}/{}", day_number, filename);
    let lines = read_lines_to_vec(file_path).unwrap_or(Vec::new());

    let start = Instant::now();
    let a1 = func(lines);
    let duration = start.elapsed();
    if a1 == expected {
        println!("    PASSED - {:?}", duration);
    } else {
        println!("    FAILED - {} - {:?}", a1, duration);
    }
}
