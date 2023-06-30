
use rock_paper_scissors::{MeOptions, OppOptions};


fn main() {
    let mut moves: Vec<(&OppOptions, &MeOptions)>;
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
                        let opp_enum: &OppOptions;
                        let me_enum: &MeOptions;
                        match opp {
                            "A" => opp_enum = &OppOptions::A,
                            "B" => opp_enum = &OppOptions::B,
                            "C" => opp_enum = &OppOptions::C,
                            _ => opp_enum = &OppOptions::None,
                        }
                        match me {
                            "X" => me_enum = &MeOptions::X,
                            "Y" => me_enum = &MeOptions::Y,
                            "Z" => me_enum = &MeOptions::Z,
                            _ => me_enum = &MeOptions::None,
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

