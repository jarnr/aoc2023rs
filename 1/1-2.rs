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
    let first: i32 = get_first_num(line);
    let reversed_line = &reverse_str(line);
    let last: i32 = get_last_num(reversed_line);
    
    10 * first + last
}

fn get_first_num(line: &str) -> i32 {
    let ll: usize = line.len();
    if ll == 0 { return 0; }

    let s: &str = &line[0..1];
    let t: &str;
    if ll > 2 { t = &line[0..3]; } else { t = line; }
    let u: &str;
    if ll > 3 { u = &line[0..4]; } else { u = line; }
    let v: &str;
    if ll > 4 { v = &line[0..5]; } else { v = line; }

    match s {
        "1" => return 1,
        "2" => return 2,
        "3" => return 3,
        "4" => return 4,
        "5" => return 5,
        "6" => return 6,
        "7" => return 7,
        "8" => return 8,
        "9" => return 9,
        _ => { },
    }

    match t {
        "one" => return 1,
        "two" => return 2,
        "six" => return 6,
        _ => { },

    }

    match u {
        "four" => return 4,
        "five" => return 5,
        "nine" => return 9,
        _ => { },
    }

    match v {
        "three" => return 3,
        "seven" => return 7,
        "eight" => return 8,
        _ => { },
    }

    return get_first_num(&line[1..]);
}

fn get_last_num(line: &str) -> i32 {
    let ll: usize = line.len();
    if ll == 0 { return 0; }

    let s: &str = &line[0..1];
    let t: &str;
    if ll > 2 { t = &line[0..3]; } else { t = line; }
    let u: &str;
    if ll > 3 { u = &line[0..4]; } else { u = line; }
    let v: &str;
    if ll > 4 { v = &line[0..5]; } else { v = line; }

    match s {
        "1" => return 1,
        "2" => return 2,
        "3" => return 3,
        "4" => return 4,
        "5" => return 5,
        "6" => return 6,
        "7" => return 7,
        "8" => return 8,
        "9" => return 9,
        _ => { },
    }

    match t {
        "eno" => return 1,
        "owt" => return 2,
        "xis" => return 6,
        _ => { },

    }

    match u {
        "ruof" => return 4,
        "evif" => return 5,
        "enin" => return 9,
        _ => { },
    }

    match v {
        "eerht" => return 3,
        "neves" => return 7,
        "thgie" => return 8,
        _ => { },
    }

    return get_last_num(&line[1..]);
}


fn reverse_str(input: &str) -> String {
    input.chars().rev().collect()
}
