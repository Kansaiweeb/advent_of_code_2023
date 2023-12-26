use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");
    let lines = input.lines();
    let games: Vec<Game> = lines
        .into_iter()
        .map(Game::from_str)
        .filter_map(|game| game.ok())
        .collect();
    let points_sum: usize = games.iter().map(|game| game.calculate_points()).sum();
    println!("{:?}", points_sum);
}

#[derive(Debug)]
struct Game {
    id: usize,
    numbers: Vec<usize>,
    winning_numbers: Vec<usize>,
}

impl Game {
    fn calculate_points(&self) -> usize {
        let mut points = 0;
        for number in &self.winning_numbers {
            if self.numbers.contains(number) {
                points += 1;
            }
        }
        if points == 0 {
            0
        } else {
            (2_usize).pow(points - 1)
        }
    }
}

impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_string, numbers_string) = s
            .strip_prefix("Card ")
            .and_then(|s| s.split_once(':'))
            .ok_or(ParseGameError)?;
        let (scratch_numbers_string, winning_numbers_string) =
            numbers_string.split_once('|').ok_or(ParseGameError)?;

        let scratch_numbers: Vec<usize> = scratch_numbers_string
            .split_whitespace()
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
        let winning_numbers: Vec<usize> = winning_numbers_string
            .split_whitespace()
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
        Ok(Game {
            id: id_string.trim().parse().map_err(|_| ParseGameError)?,
            numbers: scratch_numbers,
            winning_numbers,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;
