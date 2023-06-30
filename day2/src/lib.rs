use std::fs;
use std::io::{self, BufRead, BufReader};


#[derive(Debug)]
pub enum OppOptions {
    A,
    B,
    C,
    None,
}

#[derive(Debug)]
pub enum MeOptions {
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

pub fn game_score(opp: &OppOptions , me: &MeOptions) -> i32 {
    let mut score_points: i32 = 0;

    match me {
        MeOptions::X => {
            score_points += 1;
            match opp {
                OppOptions::A => score_points += 3,
                OppOptions::B => score_points += 0,
                OppOptions::C => score_points += 6,
                OppOptions::None => score_points += 0,
            }
        }
        MeOptions::Y => {
            score_points += 2;
            match opp {
                OppOptions::A => score_points += 6,
                OppOptions::B => score_points += 3,
                OppOptions::C => score_points += 0,
                OppOptions::None => score_points += 0,
            }
        }
        MeOptions::Z => {
            score_points += 3;
            match opp {
                OppOptions::A => score_points += 0,
                OppOptions::B => score_points += 6,
                OppOptions::C => score_points += 3,
                OppOptions::None => score_points += 0,
            }
        }
        MeOptions::None => score_points += 0,
    }

    score_points

}

pub fn game_score_fixed(opp: &OppOptions , me: &MeOptions) -> i32 {
    let mut score_points: i32 = 0;

    match me {
        MeOptions::X => {
            match opp {
                OppOptions::A => score_points += 3,
                OppOptions::B => score_points += 1,
                OppOptions::C => score_points += 2,
                OppOptions::None => score_points += 0,
            }
        }
        MeOptions::Y => {
            match opp {
                OppOptions::A => score_points += 4,
                OppOptions::B => score_points += 5,
                OppOptions::C => score_points += 6,
                OppOptions::None => score_points += 0,
            }
        }
        MeOptions::Z => {
            match opp {
                OppOptions::A => score_points += 8,
                OppOptions::B => score_points += 9,
                OppOptions::C => score_points += 7,
                OppOptions::None => score_points += 0,
            }
        }
        MeOptions::None => score_points += 0,
    }

    score_points

}




/*
fn parse_list(content: &str) -> Vec<Vec<i32>> {
    let chunked_content: Vec<&str> = content.trim().split("\n\n").collect();

    let chunked_numbers: Vec<Vec<i32>> = chunked_content
        .iter()
        .map(|chunk| {
            let numbers: Vec<i32> = chunk
                .split("\n")
                .map(|entry| {
                    let out: i32 = entry.parse().unwrap();
                    out
                })
                .collect();
            numbers
        })
        .collect();

    chunked_numbers
}
*/
