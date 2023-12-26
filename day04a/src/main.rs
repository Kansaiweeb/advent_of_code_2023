use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");
    let lines = input.lines();
    let mut games: Vec<Game> = lines
        .into_iter()
        .map(Game::from_str)
        .filter_map(|game| game.ok())
        .collect();
    for i in 0..games.len() {
        let current = games[i].clone();
        for gycha in &mut games[i + 1..i + 1 + current.points] {
            gycha.copies += current.copies;
        }
    }
    println!(
        "{:?}",
        games
            .iter()
            .map(|game| (game.id, game.copies))
            .collect::<Vec<(usize, usize)>>()
    );
    let points: usize = games.iter().map(|game| game.copies).sum();
    println!("{}", points);
}

#[derive(Debug, Clone)]
struct Game {
    id: usize,
    points: usize,
    copies: usize,
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

        let mut points = 0;
        for number in &winning_numbers {
            if scratch_numbers.contains(number) {
                points += 1;
            }
        }
        println!("id - {}, points - {}", id_string, points);
        Ok(Game {
            id: id_string.trim().parse().map_err(|_| ParseGameError)?,
            points,
            copies: 1,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;
