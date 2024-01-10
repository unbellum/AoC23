use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::cmp::max;

fn main() {
    // Check arguments for validity, check number of inputs, do not panic when not supplied an input
    if env::args().len() != 2 {
        println!("Please pass in the path to the input text file");
        return;
    }
    let filename = env::args().nth(1).unwrap();
    println!("Opening input file: {}", filename);

    // Open the input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Get the first and last digit of each line and sum them
    let mut sum = 0;
    let mut power_sum = 0;
    for line in contents.lines() {
        let ret = get_valid_game(line);

        sum += ret.0;
        power_sum += ret.1;
        println!("Running total: {}, Game power total: {}", sum, power_sum);
    }

    println!("Sum total: {}", sum);
}

fn get_valid_game(line: &str) -> (usize, usize) {
    // game is only valid if it contains at least 12 red, 13 green, and 14 blue
    // Example: Game 2: 1 green, 7 red; 1 green, 9 red, 3 blue; 4 blue, 5 red
    let (game_num_str, hands) = line.split_once(':').unwrap();
    let mut game_num = game_num_str.split_once(' ').unwrap().1.parse::<usize>().unwrap();
    // hands = 1 green, 7 red; 1 green, 9 red, 3 blue; 4 blue, 5 red
    let mut red: usize = 0;
    let mut green: usize = 0;
    let mut blue: usize = 0;
    for hand in hands.split(';') {
        let (red1, green1, blue1) = process_hand_string(hand);
        red = max(red, red1);
        green = max(green, green1);
        blue = max(blue, blue1);
    }
    println!("Game {game_num}: {red}, {green}, {blue}");
    if red > 12 || green > 13 || blue > 14 {
        game_num = 0;
    }
    (game_num, red * green * blue)
}

fn process_hand_string(hand: &str) -> (usize, usize, usize) {
    let mut red: usize = 0;
    let mut green: usize = 0;
    let mut blue: usize = 0;
    for count_str in hand.split(',') {
        let (count, color) = count_str.trim().split_once(' ').unwrap(); // Now "1" "green"
        let count_usize = count.parse::<usize>().unwrap();
        match color {
            "red" => red = count_usize,
            "green" => green = count_usize,
            "blue" => blue = count_usize,
            _ => ()
        }
    }
    (red, green, blue)
}