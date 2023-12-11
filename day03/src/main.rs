use std::vec;

fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<_> = input.lines().collect();
    let width = lines.get(0).unwrap().len();
    let map: Vec<char> = input.chars().collect();
    println!("{}", width);
    // //545150

    // //541458
    let map: Vec<&char> = map.iter().filter(|c| **c != '\n').collect();
    print_map(&map, &width);
    let part_numbers: Vec<(usize, usize, usize)> = find_part_numbers(&map, &width);
    let wrong_part_numbers = find_wrong_part_numbers(&part_numbers, &map, &width);
    let part_numbers_sum: usize = part_numbers.iter().map(|pn| pn.2).sum();
    let wrong_part_numbers_sum: usize = wrong_part_numbers.iter().map(|pn| pn.2).sum();
    println!(
        "{:?}",
        wrong_part_numbers
            .iter()
            .map(|f| f.2)
            .collect::<Vec<usize>>()
    );
    println!("{}", width);
    println!("total sum {:?}", part_numbers_sum);
    println!(
        "correct sum {:?}",
        part_numbers_sum - wrong_part_numbers_sum
    );
}

fn find_wrong_part_numbers(
    part_numbers: &Vec<(usize, usize, usize)>,
    map: &Vec<&char>,
    width: &usize,
) -> Vec<(usize, usize, usize)> {
    println!("{:?}", part_numbers);
    part_numbers
        .iter()
        .cloned()
        .filter(|part_number| !check_part_number(*part_number, &map, &width))
        .collect()
}

fn check_part_number(part_number: (usize, usize, usize), map: &Vec<&char>, width: &usize) -> bool {
    let (start_index, end_index, ..) = part_number;
    let mut results: Vec<bool> = vec![];
    for i in start_index..=end_index {
        results.push(check_index_for_special_neighbour(i, map, width));
    }
    !results
        .iter()
        .cloned()
        .filter(|result| *result)
        .collect::<Vec<bool>>()
        .is_empty()
}

fn check_index_for_special_neighbour(index: usize, map: &Vec<&char>, width: &usize) -> bool {
    let mut indices_to_check: Vec<usize> = vec![];
    let is_in_last_or_first_row = index + 1 <= *width || map.len() - index + 1 <= *width;
    let is_in_last_or_first_column = (index + 1) % width == 1 || (index + 1) % width == 0;
    //first and last row
    if is_in_last_or_first_row {
        //first row
        if index < *width {
            println!("index<width {}", index);
            indices_to_check.push(index + width);
            if index != 0 {
                indices_to_check.push(index - 1);
                indices_to_check.push(index + width - 1);
            }
            if index != *width {
                indices_to_check.push(index + 1);
                indices_to_check.push(index + 1 + width);
            }
        //last row
        } else {
            indices_to_check.push(index - width);
            if index != map.len() - width {
                println!("-index ass{}", index);
                indices_to_check.push(index - 1);
                indices_to_check.push(index - width - 1);
            }
            if index + 1 != map.len() {
                println!("index ass{}", index);
                indices_to_check.push(index + 1);
                indices_to_check.push(index - width + 1)
            }
        }
    }
    //inner part of map
    if !is_in_last_or_first_column && !is_in_last_or_first_row {
        let mut indices = vec![
            index - 1,
            index + 1,
            index - width,
            index - width + 1,
            index - width - 1,
            index + width,
            index + width + 1,
            index - 1 + width,
        ];
        indices_to_check.append(&mut indices)
    }

    println!(
        "index = {}, last or first row= {}, last or first col = {}, to check - {:?}",
        index, is_in_last_or_first_row, is_in_last_or_first_column, indices_to_check
    );
    for index1 in indices_to_check
        .iter()
        .filter(|index| *index + 1 < map.len())
    {
        let cell = map
            .get(*index1)
            .expect(&format!("error at index {}", index1));
        if !(cell.is_ascii_digit() || cell.eq(&&'.')) {
            return true;
        }
    }

    return false;
}

fn find_part_numbers(map: &Vec<&char>, width: &usize) -> Vec<(usize, usize, usize)> {
    let mut part_numbers: Vec<(usize, usize, usize)> = vec![];
    let mut is_being_processed: bool = false;
    let mut start_index = 0;
    let mut intermediate_part_number = vec![];
    // let mut part_numbers: (usize, usize, usize);
    for (index, cell) in map.iter().enumerate() {
        if cell.is_ascii_digit() {
            if !is_being_processed {
                is_being_processed = true;
                start_index = index;
            }
            intermediate_part_number.push(**cell);
        }
        println!("index {}, modulus {}", index, (index + 1) % width);
        if ((index + 1) % width == 0)
            || (!cell.is_ascii_digit() && is_being_processed)
            || index == map.len() - 1
        {
            if (intermediate_part_number.is_empty()) {
                continue;
            }
            let a = if (index + 1) % width == 0 {
                index
            } else {
                index - 1
            };
            is_being_processed = false;
            part_numbers.push((
                start_index,
                a,
                intermediate_part_number
                    .iter()
                    .cloned()
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            ));
            intermediate_part_number.clear();
        }
    }

    part_numbers
}

fn print_map(map: &Vec<&char>, width: &usize) {
    for (index, cell) in map.iter().enumerate() {
        print!("{}", cell);
        if (index + 1) % width == 0 {
            println!();
        }
    }
}
