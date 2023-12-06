use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() -> std::io::Result<()> {
    // Check arguments for validity, check number of inputs, do not panic when not supplied an input
    if env::args().len() != 2 {
        println!("Please pass in the path to the input text file");
        return Ok(());
    }
    let filename = env::args().nth(1).unwrap();
    println!("Opening input file: {}", filename);

    // Open the input file
    // TODO: Properly handle file not found case, print a good error message, and exit
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Get the first and last digit of each line and sum them
    let mut sum = 0;
    for line in contents.lines() {
        // First part
        //sum += get_digits_part1(line);

        // Second part
        sum += get_digits_part2(line);

        println!("Running total: {}", sum);
    }

    println!("Sum total: {}", sum);

    Ok(())
}

// Used for the first part only
#[allow(dead_code)]
fn get_digits_part1(line: &str) -> i32 {
    let mut first = 0;
    let mut last = 0;
    let bytes = line.as_bytes();
    for ch in bytes.iter() {
        if ch.is_ascii_digit() {
            let digit = ch - b'0';
            if first == 0 {
                first = digit as i32;
                last = digit as i32;
            }
            else {
                last = digit as i32;
            }
        }
    }

    println!("{} = {}{}", line, first, last);

    first * 10 + last
}

// Used for the second part only
#[allow(dead_code)]
fn get_digits_part2(line: &str) -> i32 {
    println!("\tInput line: {}", line);
    let first = get_first_digit(line);
    let second = get_last_digit(line, first);
    println!("Digits: {}{}", first.1 / 10, second);
    first.1 + second
}

fn get_first_digit(line: &str) -> (usize, i32) {
    let mut digit: (usize, i32) = (line.len(), 0);

    // Search for the first instance of a spelled out digit, save the earliest
    let val = line.find("one");
    digit = is_match(digit, val, 1, true);
    let val = line.find("two");
    digit = is_match(digit, val, 2, true);
    let val = line.find("three");
    digit = is_match(digit, val, 3, true);
    let val = line.find("four");
    digit = is_match(digit, val, 4, true);
    let val = line.find("five");
    digit = is_match(digit, val, 5, true);
    let val = line.find("six");
    digit = is_match(digit, val, 6, true);
    let val = line.find("seven");
    digit = is_match(digit, val, 7, true);
    let val = line.find("eight");
    digit = is_match(digit, val, 8, true);
    let val = line.find("nine");
    digit = is_match(digit, val, 9, true);

    // No zero
    //let val = line.find("zero");
    //digit = is_match(digit, val, 0, true);

    // Now find the first numerical digit and compare
    let mut val = get_numerical_digit(line, true);
    if val.0 < digit.0 {
        val.1 *= 10;
        return val;
    }
    
    digit.1 *= 10;
    digit
}

fn is_match(digit: (usize, i32), val: Option<usize>, num: i32, first: bool) -> (usize, i32) {
    match val {
        Some(pos) => {
            let mut check = pos >= digit.0;
            if first {
                check = pos < digit.0;
            }
            if check {
                println!("\tSpelled digit: {}, {}", pos, num);
                println!("\tReplaced pos: {}", digit.0);
                return (pos, num);
            }
        },
        None => ()
    }
    digit
}

fn get_numerical_digit(line: &str, first: bool) -> (usize, i32) {
    let mut last = (0, 0);
    let bytes = line.as_bytes();
    for ch in bytes.iter().enumerate() {
        if ch.1.is_ascii_digit() {
            let digit = ch.1 - b'0';
            if first  {
                println!("\tFirst numerical digit: {}, {}", ch.0, digit);
                return (ch.0, digit as i32);
            }
            last = (ch.0, digit as i32);
        }
    }

    println!("\tLast numerical digit: {}, {}", last.0, last.1);
    last
}

fn get_last_digit(line: &str, first: (usize, i32)) -> i32 {
    let mut digit: (usize, i32) = (0, 0);

    // Search for the last instance of a spelled out digit, save the earliest
    let val = line.rfind("one");
    digit = is_match(digit, val, 1, false);
    let val = line.rfind("two");
    digit = is_match(digit, val, 2, false);
    let val = line.rfind("three");
    digit = is_match(digit, val, 3, false);
    let val = line.rfind("four");
    digit = is_match(digit, val, 4, false);
    let val = line.rfind("five");
    digit = is_match(digit, val, 5, false);
    let val = line.rfind("six");
    digit = is_match(digit, val, 6, false);
    let val = line.rfind("seven");
    digit = is_match(digit, val, 7, false);
    let val = line.rfind("eight");
    digit = is_match(digit, val, 8, false);
    let val = line.rfind("nine");
    digit = is_match(digit, val, 9, false);

    // No zero
    //let val = line.rfind("zero");
    //digit = is_match(digit, val, 0, false);

    // Now find the last numerical digit and compare
    let val = get_numerical_digit(line, false);
    if val.0 >= digit.0 && val.0 >= first.0 && val.1 != 0 {
        return val.1;
    }
    
    println!("\tInput: {}", line);

    if first.0 >= digit.0 {
        return first.1 / 10;
    }
    digit.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(get_digits_part2("eighthree"), 83);
        assert_eq!(get_digits_part2("oneoneight"), 18);
        assert_eq!(get_digits_part2("sevenine"), 79);
        assert_eq!(get_digits_part2("2twonemg"), 21);
        assert_eq!(get_digits_part2("3twone"), 31);
        assert_eq!(get_digits_part2("4twoneight"), 48);
        assert_eq!(get_digits_part2("twoneight4twoneightwone"), 21);
        assert_eq!(get_digits_part2("two3two"), 22);
        assert_eq!(get_digits_part2("3two3"), 33);
        assert_eq!(get_digits_part2("36twonine"), 39);
        assert_eq!(get_digits_part2("3nqqgfone"), 31);
        assert_eq!(get_digits_part2("mkfone4ninefour"), 14);
        assert_eq!(get_digits_part2("kgnprzeight7nine"), 89);
        assert_eq!(get_digits_part2("xgjjmnlvznf2nineltmsevenine"), 29);
        assert_eq!(get_digits_part2("five"), 55);
        assert_eq!(get_digits_part2("3rxgts"), 33);
    }
}