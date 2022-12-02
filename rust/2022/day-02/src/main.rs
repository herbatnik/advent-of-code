extern crate core;

use std::io::{BufRead, BufReader};

fn main() {
    let file_path = "./input";
    let part_one_result = part_one(file_path);
    let part_two_result = part_two(file_path);
    println!("part one: {part_one_result:?}");
    println!("part two: {part_two_result:?}");
}

fn part_one(file_path: &str) -> i32 {
    read_matches(file_path)
        .into_iter()
        .fold(0, |acc, next| acc + next.play_part_one())
}

fn part_two(file_path: &str) -> i32 {
    read_matches(file_path)
        .into_iter()
        .fold(0, |acc, next| acc + next.play_part_two())
}

#[derive(Debug, PartialEq)]
enum Signs {
    Rock,
    Paper,
    Scissors,
}

enum Strategy {
    Win,
    Draw,
    Loose
}

enum Instruction {
    X,
    Y,
    Z
}

impl Into<Strategy> for Instruction {
    fn into(self) -> Strategy {
        match self {
            Instruction::X => Strategy::Loose,
            Instruction::Y => Strategy::Draw,
            Instruction::Z => Strategy::Win
        }
    }
}

impl Into<Signs> for Instruction {
    fn into(self) -> Signs {
        match self {
            Instruction::X => Signs::Rock,
            Instruction::Y => Signs::Paper,
            Instruction::Z => Signs::Scissors
        }
    }
}

impl Signs {
    pub fn points(&self) -> i32 {
        match &self {
            Signs::Rock => 1,
            Signs::Paper => 2,
            Signs::Scissors => 3,
        }
    }
    
    pub fn looses_with(&self) -> Signs {
        match &self {
            Signs::Rock => Signs::Paper,
            Signs::Paper => Signs::Scissors,
            Signs::Scissors => Signs::Rock,
        }
    }
    
    pub fn wins_with(&self) -> Signs {
        match &self {
            Signs::Rock => Signs::Scissors,
            Signs::Paper => Signs::Rock,
            Signs::Scissors => Signs::Paper,
        }
    }
    
    pub fn draws_with(&self) -> Signs {
        match &self {
            Signs::Rock => Signs::Rock,
            Signs::Paper => Signs::Paper,
            Signs::Scissors => Signs::Scissors,
        }
    }
}

struct Game {
    opponent: Signs,
    player: Instruction,
}

impl Game {
    const WIN_POINTS: i32 = 6;
    const DRAW_POINTS: i32 = 3;
    const LOOSE_POINTS: i32 = 0;
    
    fn play_part_one(self) -> i32 {
        let player_sign: Signs = self.player.into();
        player_sign.points() + match (self.opponent, player_sign) {
            (o, p) if o.draws_with() == p => Game::DRAW_POINTS,
            (o, p) if o.looses_with() == p => Game::WIN_POINTS,
            (o, p) if o.wins_with() == p => Game::LOOSE_POINTS,
            (op, pl) => panic!("invalid pair: {op:?} vs {pl:?}"),
        }
    }

    fn play_part_two(self) -> i32 {
        match self.player.into() { 
            Strategy::Loose => Game::LOOSE_POINTS + &self.opponent.wins_with().points(),
            Strategy::Draw => Game::DRAW_POINTS + &self.opponent.draws_with().points(),
            Strategy::Win => Game::WIN_POINTS + &self.opponent.looses_with().points(),
        }
    }
}

fn read_matches(file_path: &str) -> impl Iterator<Item=Game> {
    let file = std::fs::File::open(file_path).expect("failed to read file");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("failed to read line"))
        .map(parse_line)
}

fn parse_line(line: String) -> Game {
    let split: [&str; 2] = line.split(' ').collect::<Vec<&str>>().try_into().expect("expected 2 entries per line");
    let opponent = match split[0] {
        "A" => Signs::Rock,
        "B" => Signs::Paper,
        "C" => Signs::Scissors,
        char => panic!("unknown sign {char:?}'"),
    };
    let player = match split[1] {
        "X" => Instruction::X,
        "Y" => Instruction::Y,
        "Z" => Instruction::Z,
        char => panic!("unknown sign {char:?}'"),
    };
    Game { opponent, player }
}
