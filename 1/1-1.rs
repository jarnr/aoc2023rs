use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input.txt");
    
    let mut sum = 0;
    for line in &lines {
        sum += get_value(line);
    }

    println!("{}", sum);
}

// shamelessly ripped from Rust By Example
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn get_value(line: &str) -> i32 {
    let first: i32 = get_first_digit(line);
    let last: i32 = get_last_digit(line);
    
    10 * first + last
}

fn get_first_digit(line: &str) -> i32 {
    for c in line.chars() {
        match c {
            '1' => return 1,
            '2' => return 2,
            '3' => return 3,
            '4' => return 4,
            '5' => return 5,
            '6' => return 6,
            '7' => return 7,
            '8' => return 8,
            '9' => return 9,
            _ => continue,
        }
    }
    return 0;
}

fn get_last_digit(line: &str) -> i32 {
    for c in line.chars().rev() {
        match c {
            '1' => return 1,
            '2' => return 2,
            '3' => return 3,
            '4' => return 4,
            '5' => return 5,
            '6' => return 6,
            '7' => return 7,
            '8' => return 8,
            '9' => return 9,
            _ => continue,
        }
    }
    return 0;
}