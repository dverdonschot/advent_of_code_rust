use day3::{
    char_to_enum, compare_both_halves, compare_three_lines, priorities_scores, read_file_contents,
    split_string_in_half,
};

fn main() {
    let file_path = "input.txt";
    //task 1 variables
    let mut task1_total_priority_score: u32 = 0;
    // task 2 variables
    let mut one_to_three: u8 = 0;
    let mut team_vec = Vec::new();
    let mut task2_total_teams_score: u32 = 0;
    // processing scores
    match read_file_contents(file_path) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(content) => {
                        // task 1
                        let (first_half, second_half) = split_string_in_half(&content);
                        let duplicate_char_result = compare_both_halves(first_half, second_half);
                        let priorities_results =
                            priorities_scores(char_to_enum(duplicate_char_result));
                        task1_total_priority_score += priorities_results;
                        //task 2
                        one_to_three += 1;
                        team_vec.push(content);
                        if one_to_three == 3 {
                            let team_char = compare_three_lines(
                                team_vec[1].as_str(),
                                team_vec[1].as_str(),
                                team_vec[2].as_str(),
                            );
                            let team_score = priorities_scores(char_to_enum(team_char));
                            task2_total_teams_score += team_score;
                            team_vec.pop();
                            team_vec.pop();
                            team_vec.pop();
                            one_to_three = 0;
                        }
                    }
                    Err(error) => {
                        eprintln!("Error reading line: {}", error);
                    }
                };
            }
            println!(
                "Total Priorities Score Task 1 : {} \nTotal Teams Score Task 2 : {}",
                task1_total_priority_score, task2_total_teams_score
            );
        }
        Err(error) => {
            eprintln!("Error reading lines: {}", error)
        }
    }
}
