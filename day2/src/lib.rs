use std::fs;
use std::io::{self, BufRead, BufReader};


#[derive(Debug)]
pub enum Opponent {
    A,
    B,
    C,
    None,
}

#[derive(Debug)]
pub enum Player {
    X,
    Y,
    Z,
    None,
}


pub fn read_file_contents(file_path: &str) -> Result<impl Iterator<Item = io::Result<String>>, io::Error> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

pub fn game_score(opp: &Opponent , me: &Player) -> i32 {
    let mut score_points: i32 = 0;

    match me {
        Player::X => {
            score_points += 1;
            match opp {
                Opponent::A => score_points += 3,
                Opponent::B => score_points += 0,
                Opponent::C => score_points += 6,
                Opponent::None => score_points += 0,
            }
        }
        Player::Y => {
            score_points += 2;
            match opp {
                Opponent::A => score_points += 6,
                Opponent::B => score_points += 3,
                Opponent::C => score_points += 0,
                Opponent::None => score_points += 0,
            }
        }
        Player::Z => {
            score_points += 3;
            match opp {
                Opponent::A => score_points += 0,
                Opponent::B => score_points += 6,
                Opponent::C => score_points += 3,
                Opponent::None => score_points += 0,
            }
        }
        Player::None => score_points += 0,
    }

    score_points

}

pub fn game_score_fixed(opp: &Opponent , me: &Player) -> i32 {
    let mut score_points: i32 = 0;

    match me {
        Player::X => {
            match opp {
                Opponent::A => score_points += 3,
                Opponent::B => score_points += 1,
                Opponent::C => score_points += 2,
                Opponent::None => score_points += 0,
            }
        }
        Player::Y => {
            match opp {
                Opponent::A => score_points += 4,
                Opponent::B => score_points += 5,
                Opponent::C => score_points += 6,
                Opponent::None => score_points += 0,
            }
        }
        Player::Z => {
            match opp {
                Opponent::A => score_points += 8,
                Opponent::B => score_points += 9,
                Opponent::C => score_points += 7,
                Opponent::None => score_points += 0,
            }
        }
        Player::None => score_points += 0,
    }

    score_points

}
