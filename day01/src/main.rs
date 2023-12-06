fn main() {
    let input = include_str!("./input.txt");
    let split_input: Vec<&str> = input.split("\n").collect();
    let mut sum = 0;
    for line in split_input {
        let mut first_digit = '0';
        let mut last_digit = '0';
        let mut is_first_digit_found = false;

        for symbol in line.chars() {
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
        // println!("{}, {}", first_digit, last_digit);
        let result: usize = format!("{}{}", first_digit, last_digit).parse().unwrap();
        sum = sum + result;
    }

    println!("{}", sum);
}
