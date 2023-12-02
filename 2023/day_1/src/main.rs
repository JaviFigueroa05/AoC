use std::fs;

fn main() {
    let calibration_values_sum: i32 = 
        fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(str::to_string)
        .map(spelled_number_converter)
        .map(get_calibration_value)
        .sum();

    print!("{}", calibration_values_sum);
}

fn spelled_number_converter(line: String) -> String {
    line.replace("one",     "o1e")
        .replace("two",     "t2o")
        .replace("three",   "t3ree")
        .replace("four",    "f4ur")
        .replace("five",    "f5ve")
        .replace("six",     "s6x")
        .replace("seven",   "s7ven")
        .replace("eight",   "e8ght")
        .replace("nine",    "n9ne")
}

fn get_calibration_value(line: String) -> i32 {
    let mut head_index: usize = 0;
    let mut tail_index: usize = line.len() - 1;
    let mut head: char = '0';
    let mut tail: char = '0';
    let mut found_first_digit = false;
    let mut found_last_digit = false;
    let line_bytes = line.as_bytes();
    
    loop {
        if !found_first_digit {
            head = line_bytes[head_index] as char;   
            if head.is_numeric() {
                found_first_digit = true;
            }
            else {
                head_index += 1;
            }
        }
        if !found_last_digit {
            tail = line_bytes[tail_index] as char;
            if tail.is_numeric() {
                found_last_digit = true;
            }
            else {
                tail_index -= 1;
            }
        }
        if found_first_digit && found_last_digit {
            break;
        }
    }

    format!("{}{}", head, tail).parse::<i32>().unwrap()
}