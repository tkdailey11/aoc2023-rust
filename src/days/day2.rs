use std::cmp::max;
use std::collections::HashMap;
use crate::days::run_day;

struct Draw {
    draws: Vec<CubeSet>
}

struct CubeSet {
    num: i32,
    color: String
}

pub fn run() {
    let day_number: i32 = 2;

    let part1_expected = 2268;
    let part1_filename = "input.txt";

    let part2_expected = 63542;
    let part2_filename = "input.txt";

    run_day(day_number, run_part_1, part1_expected, part1_filename, run_part_2, part2_expected, part2_filename);
}
fn run_part_1(lines: Vec<String>) -> i32 {
    let game_map = lines_to_game_map(lines);

    let mut sum = 0;

    game_map.iter().for_each(|game| {
        let mut game_possible = true;
        game.1.iter().for_each(|draw| {
            draw.draws.iter().for_each(|d| {
                match d.color.as_str() {
                    "red" => { game_possible &= d.num <= 12; },
                    "green" => { game_possible &= d.num <= 13; },
                    "blue" => { game_possible &= d.num <= 14; },
                    _ => { game_possible = false }
                }
            })
        });

        if game_possible {
            sum += game.0
        }
    });

    sum
}

fn run_part_2(lines: Vec<String>) -> i32 {
    let game_map = lines_to_game_map(lines);

    let mut sum = 0;

    game_map.iter().for_each(|game| {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        game.1.iter().for_each(|draw| {
            draw.draws.iter().for_each(|d| {
                match d.color.as_str() {
                    "red" => { max_red = max(max_red, d.num); },
                    "green" => { max_green = max(max_green, d.num); },
                    "blue" => { max_blue = max(max_blue, d.num); },
                    _ => { }
                }
            })
        });

        sum += max_red * max_green * max_blue;
    });

    sum
}

fn lines_to_game_map(lines: Vec<String>) -> HashMap<i32, Vec<Draw>> {
    let mut res: HashMap<i32, Vec<Draw>> = HashMap::new();

    lines.iter().for_each(|line| {
        let mut parts = line.split(": ");
        let game_id = parts.nth(0).unwrap().split(" ").nth(1).unwrap().parse::<i32>().unwrap();

        let mut draws: Vec<Draw> = Vec::new();
        parts.nth(0).unwrap().split("; ").for_each(|draw| {
            let draw_parts = draw.split(", ");
            let mut draw_list: Vec<CubeSet> = Vec::new();
            draw_parts.for_each(|part| {
                let mut cube_parts = part.split(" ");
                let cube = CubeSet{num: cube_parts.nth(0).unwrap().parse::<i32>().unwrap(), color: cube_parts.nth(0).unwrap().to_string()};
                draw_list.push(cube);
            });

            draws.push(Draw {draws: draw_list});
        });

        res.insert(game_id, draws);
    });

    res
}