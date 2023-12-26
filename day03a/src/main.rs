use std::{collections::HashMap, vec};

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
    let correct_part_numbers = find_wrong_part_numbers(&part_numbers, &map, &width);
    let non_gears_part_numbers = correct_part_numbers
        .iter()
        .filter(|part_number| part_number.3.is_none())
        .collect::<Vec<&(usize, usize, usize, Option<usize>)>>();
    let gears_part_numbers: Vec<&(usize, usize, usize, Option<usize>)> = correct_part_numbers
        .iter()
        .filter(|part_number| part_number.3.is_some())
        .collect();
    let non_gears_sum: usize = non_gears_part_numbers
        .iter()
        .map(|part_number| part_number.2)
        .sum();
    let gears_sum = {
        let mut gears_map: HashMap<usize, Vec<usize>> = HashMap::new();
        for gear in gears_part_numbers.clone() {
            println!("{:?}", gear);
            let gear_coord = &gear.3.unwrap();
            if gears_map.contains_key(gear_coord) {
                gears_map.get_mut(gear_coord).unwrap().push(gear.2);
            } else {
                gears_map.insert(gear.3.unwrap(), vec![gear.2]);
            }
        }
        let mut sum = 0;
        for gear in gears_map.values() {
            if gear.len() == 1 {
                continue;
            }
            sum = sum + gear.iter().cloned().reduce(|a, b| (a * b)).unwrap();
            println!("{}", sum);
        }
        println!("{:?}", gears_map);
        sum
    };
    println!("gears sum: {}", gears_sum);
    println!("{}", non_gears_sum + gears_sum);

    // let part_numbers_sum: usize = part_numbers.iter().map(|pn| pn.2).sum();
    // let wrong_part_numbers_sum: usize = wrong_part_numbers.iter().map(|pn| pn.2).sum();
    // println!(
    //     "{:?}",
    //     wrong_part_numbers
    //         .iter()
    //         .map(|f| (f.2, f.3))
    //         .collect::<Vec<(usize, Option<usize>)>>()
    // );
    // println!("{}", width);
    // println!("total sum {:?}", part_numbers_sum);
    // println!(
    //     "correct sum {:?}",
    //     part_numbers_sum - wrong_part_numbers_sum
    // );
}

fn find_wrong_part_numbers(
    part_numbers: &Vec<(usize, usize, usize)>,
    map: &Vec<&char>,
    width: &usize,
) -> Vec<(usize, usize, usize, Option<usize>)> {
    println!("{:?}", part_numbers);
    part_numbers
        .iter()
        .cloned()
        // .filter(|part_number| !check_part_number(*part_number, &map, &width).0)
        .map(|part_number| {
            (
                part_number.0,
                part_number.1,
                part_number.2,
                check_part_number(part_number, map, width),
            )
        })
        .filter(|part_number| part_number.3 .0)
        .map(|part_number| {
            (
                part_number.0,
                part_number.1,
                part_number.2,
                part_number.3 .1,
            )
        })
        .collect()
}

fn check_part_number(
    part_number: (usize, usize, usize),
    map: &Vec<&char>,
    width: &usize,
) -> (bool, Option<usize>) {
    let (start_index, end_index, ..) = part_number;
    let mut results: Vec<(bool, Option<usize>)> = vec![];
    for i in start_index..=end_index {
        results.push(check_index_for_special_neighbour(i, map, width));
    }
    let special_char_indices = results
        .iter()
        .filter(|result| result.1.is_some())
        .map(|result| result.1)
        .collect::<Vec<Option<usize>>>();
    let special_char_index = if special_char_indices.is_empty() {
        None
    } else {
        *special_char_indices.get(0).expect("Syuka bliyat")
    };
    return (
        !results
            .iter()
            .cloned()
            .filter(|result| result.0)
            .map(|result| result.0)
            .collect::<Vec<bool>>()
            .is_empty(),
        special_char_index,
    );
}

fn check_index_for_special_neighbour(
    index: usize,
    map: &Vec<&char>,
    width: &usize,
) -> (bool, Option<usize>) {
    let mut indices_to_check: Vec<usize> = vec![];
    let is_in_last_or_first_row = index + 1 <= *width || map.len() - index + 1 <= *width;
    let is_in_last_or_first_column = (index + 1) % width == 1 || (index + 1) % width == 0;
    //first and last row
    if is_in_last_or_first_row {
        //first row
        if index < *width {
            // println!("index<width {}", index);
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
                // println!("-index ass{}", index);
                indices_to_check.push(index - 1);
                indices_to_check.push(index - width - 1);
            }
            if index + 1 != map.len() {
                // println!("index ass{}", index);
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

    // println!(
    //     "index = {}, last or first row= {}, last or first col = {}, to check - {:?}",
    //     index,
    //     is_in_last_or_first_row, is_in_last_or_first_column, indices_to_check
    // );
    for index1 in indices_to_check
        .iter()
        .filter(|index| *index + 1 < map.len())
    {
        let cell = map
            .get(*index1)
            .expect(&format!("error at index {}", index1));
        if !(cell.is_ascii_digit() || cell.eq(&&'.')) {
            let mut special_char_coord: Option<usize> = None;
            if cell.eq(&&'*') {
                special_char_coord = Some(*index1)
            };
            return (true, special_char_coord);
        }
    }

    return (false, None);
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
        // println!("index {}, modulus {}", index, (index + 1) % width);
        if ((index + 1) % width == 0)
            || (!cell.is_ascii_digit() && is_being_processed)
            || index == map.len() - 1
        {
            if intermediate_part_number.is_empty() {
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
