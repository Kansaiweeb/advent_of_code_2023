fn main() {
    let input = include_str!("./input.txt");
    let split_input: Vec<&str> = input.split("\n").collect();
    let mut sum = 0;
    for line in split_input {
        let (first, last) = get_first_and_last_digits(line);
        let result: usize = format!("{}{}", first, last).parse().unwrap();
        sum = sum + result;
    }
    println!("{}", sum);
}

const DIGIT_STRINGS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn get_first_and_last_digits(line: &str) -> (char, char) {
    let mut parsed_line: String = line.to_string();
    for digit in 0..10 {
        parsed_line = parsed_line.replace(
            DIGIT_STRINGS[digit],
            &format!(
                "{}{}{}",
                DIGIT_STRINGS[digit], DIGITS[digit], DIGIT_STRINGS[digit]
            ),
        );
    }
    let mut first_digit = '0';
    let mut last_digit = '0';
    let mut is_first_digit_found = false;
    // println!("{}", parsed_line);
    for symbol in parsed_line.chars() {
        if symbol.is_numeric() {
            if !is_first_digit_found {
                is_first_digit_found = true;
                first_digit = symbol;
                last_digit = symbol;
            } else {
                last_digit = symbol;
            }
        }
    }
    // println!("{}{}", first_digit, last_digit);
    return (first_digit, last_digit);
}
