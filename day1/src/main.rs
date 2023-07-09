fn main() {
    let input = include_str!("../input.txt");
    println!("The top Gatherer is: {}", top_gatherer(input));
    println!(
        "The cumulative of the 3 top gathers is: {}",
        top3_gatherers(input)
    );
}

/* Based on https://rust-school.io/writing/aoc2022-day1/ */

#[test]
fn test() {
    let input = include_str!("../input.txt");
    assert_eq!(top_gatherer(input), 24000);
    assert_eq!(top3_gatherers(input), 45000);
}

fn top_gatherer(inputlist: &str) -> i32 {
    inputlist
        .split("\n\n")
        .map(|elf| elf.lines().map(|cals| cals.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap_or(0)
}

fn top3_gatherers(inputlist: &str) -> i32 {
    let mut top3: Vec<i32> = inputlist
        .split("\n\n")
        .map(|elf| elf.lines().map(|cals| cals.parse::<i32>().unwrap()).sum())
        .collect();
    top3.sort();
    top3.into_iter().rev().take(3).sum()
}
