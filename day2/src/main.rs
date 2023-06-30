
use rock_paper_scissors::{Player, Opponent};


fn main() {
    let mut moves: Vec<(&Opponent, &Player)>;
    moves = vec![];
    let mut total_score: i32 = 0;
    let mut total_score_fixed: i32 = 0;
    let file_path = "input.txt";
    match rock_paper_scissors::read_file_contents(file_path) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(content) => {
                        let (opp, me) = content.split_once(' ').unwrap();
                        let opp_enum: &Opponent;
                        let me_enum: &Player;
                        match opp {
                            "A" => opp_enum = &Opponent::A,
                            "B" => opp_enum = &Opponent::B,
                            "C" => opp_enum = &Opponent::C,
                            _ => opp_enum = &Opponent::None,
                        }
                        match me {
                            "X" => me_enum = &Player::X,
                            "Y" => me_enum = &Player::Y,
                            "Z" => me_enum = &Player::Z,
                            _ => me_enum = &Player::None,
                        }
                        
                        moves.push((
                            &opp_enum,
                            &me_enum,
                        ));
                        total_score += rock_paper_scissors::game_score(&opp_enum, &me_enum);
                        total_score_fixed += rock_paper_scissors::game_score_fixed(&opp_enum, &me_enum);
                    }
                    Err(error) => {
                        eprintln!("Error reading line: {}", error);
                    }
                }
            }
        }
        Err(error) => eprintln!("{}", error),
    }
    println!("{:?}", moves);
    println!("Total Score {:?}", total_score);
    println!("Total Score Fixed: {:?}", total_score_fixed);
}

