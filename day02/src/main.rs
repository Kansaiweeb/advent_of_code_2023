use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");
    let split_input: Vec<&str> = input.split('\n').collect();
    let mut games = vec![];
    for game in split_input {
        games.push(Game::from_str(game).unwrap());
    }
    let id_sum: usize = games
        .iter()
        .filter(|game| game.is_possible)
        .map(|game| game.id)
        .sum();
    println!("{}", id_sum);
}

const BLUE_COUNT: usize = 14;
const RED_COUNT: usize = 12;
const GREEN_COUNT: usize = 13;

#[derive(Debug)]
struct Game {
    id: usize,
    is_possible: bool,
}

#[derive(Debug)]
struct GameResult {
    is_possible: bool,
}

impl FromStr for GameResult {
    type Err = ParseGameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dice_count: Vec<&str> = s.split(',').map(|s| s.trim()).collect();
        let mut green: Option<usize> = None;
        let mut blue: Option<usize> = None;
        let mut red: Option<usize> = None;

        for dice in dice_count {
            let mut dice_iter = dice.split(' ');
            let count = dice_iter.next().unwrap();
            let colour = dice_iter.next().unwrap();
            if colour.eq("green") {
                green = Some(count.parse().unwrap());
                continue;
            }
            if colour.eq("red") {
                red = Some(count.parse().unwrap());
                continue;
            }
            if colour.eq("blue") {
                blue = Some(count.parse().unwrap());
            }
        }
        let mut is_possible = true;

        if blue.is_some() && blue.unwrap() > BLUE_COUNT {
            is_possible = false;
        }
        if red.is_some() && red.unwrap() > RED_COUNT {
            is_possible = false;
        }
        if green.is_some() && green.unwrap() > GREEN_COUNT {
            is_possible = false;
        }
        Ok(GameResult { is_possible })
    }
}

impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_string, game_results) = s
            .strip_prefix("Game ")
            .and_then(|s| s.split_once(':'))
            .ok_or(ParseGameError)?;
        let mut game_results_parsed: Vec<GameResult> = vec![];

        for game_result in game_results.split(';').map(|s| s.trim()) {
            game_results_parsed.push(GameResult::from_str(game_result).unwrap());
        }

        let impossible_games_count = game_results_parsed
            .iter()
            .filter(|res| !res.is_possible)
            .count();
        let mut is_possible = true;
        if impossible_games_count > 0 {
            is_possible = false;
        }

        let id: usize = id_string.parse().map_err(|_| ParseGameError)?;
        Ok(Game { id, is_possible })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;
