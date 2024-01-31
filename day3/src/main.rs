use std::fs::File;
use std::io::prelude::*;
use std::env;

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
    let sum = get_schematic_count(&contents);
    //let mut sum = 0;
    //for line in contents.lines() {
        //let ret = get_valid_game(line);

        //sum += ret.0;
        //power_sum += ret.1;
        //println!("Running total: {}, Game power total: {}", sum, power_sum);
    //}

    println!("Sum total: {}", sum);
}

fn is_symbol(ch: char) -> bool {
    !ch.is_ascii_digit() && ch != '.' && ch != '\n'
}

fn get_schematic_count(file: &str) -> usize {
    // Get the line length. We know each line is uniform length
    let line_len = file.find("\n").unwrap();
    let file_len = file.len();
    let chars: Vec<char> = file.chars().collect();
    let lines = file_len / line_len - 1;
    //println!("{line_len}, {file_len}, {lines}");
    let mut sum: usize = 0;

    // For each line, go through all of the characters and find the start/end of each number
    for line in 0..lines {
        let mut start: i32 = -1;
        let mut num: usize = 0;
        let line_start: usize = line * (line_len + 1);
        for ch in 0..=line_len {
            if start < 0 && chars[line_start + ch].is_ascii_digit() {
                start = ch as i32;
                num = chars[line_start + ch].to_digit(10).unwrap() as usize;
            }
            else if start >= 0 {
                if chars[line_start + ch].is_ascii_digit() {
                    //println!("Num {num}");
                    num *= 10;
                    num += chars[line_start + ch].to_digit(10).unwrap() as usize;
                }
                else {
                    // Found the last digit, find the connected symbols
                    // Check line above if there was a previous line
                    // Check line below if not at end of document
                    // Check before start if not beginning of line
                    // Check after end if not end of line
                    //println!("Suspected digit {num}");
                    let end = ch - 1;
                    let mut found_symbol = false;
                    let mut check_start: usize = start as usize;
                    let mut check_end: usize = end;
                    if check_start > 0 {
                        check_start -= 1;
                        if is_symbol(chars[line_start + check_start]) {
                            found_symbol = true
                        }
                    }
                    if check_end < line_len {
                        check_end += 1;
                        if is_symbol(chars[line_start + check_end]) {
                            found_symbol = true
                        }
                    }
                    //println!("({start},{end}) ({check_start},{check_end})");
                    if line > 0 {
                        if get_connected_symbol(&chars, (line - 1) * (line_len + 1), check_start, check_end) {
                            found_symbol = true;
                        }
                    }
                    if line < (lines - 1) {
                        if get_connected_symbol(&chars, (line + 1) * (line_len + 1), check_start, check_end) {
                            found_symbol = true;
                        }
                    }
                    if found_symbol {
                        //println!("Found");
                        sum += num;
                    }
                    num = 0;
                    start = -1;
                }
            }
        }
    }

    sum
}

fn get_connected_symbol(chars: &Vec<char>, line_start: usize, start: usize, end: usize) -> bool {
    let mut ret = false;
    //let mut char_vals: String = String::new();
    for ch in start..=end {
        if is_symbol(chars[line_start + ch]) {
            ret = true;
        }
        //char_vals.push(chars[line_start + ch]);
    }
    //println!("{char_vals}");
    ret
}