//use crate::lints::NonCamelCaseType;
use std::fs;
use std::io::{self, BufRead, BufReader};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Priorities {
    a,
    b,
    c,
    d,
    e,
    f,
    g,
    h,
    i,
    j,
    k,
    l,
    m,
    n,
    o,
    p,
    q,
    r,
    s,
    t,
    u,
    v,
    w,
    x,
    y,
    z,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    None,
}

pub fn read_file_contents(
    file_path: &str,
) -> Result<impl Iterator<Item = io::Result<String>>, io::Error> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

pub fn split_string_in_half(string: &String) -> (&str, &str) {
    let half_length = &string.len() / 2;

    let first_half = &string[..half_length];
    let second_half = &string[half_length..];

    (first_half, second_half)
}

pub fn compare_both_halves(first_half: &str, second_half: &str) -> char {
    let both_halves: Vec<&str> = vec![first_half, second_half];
    let chars_second_half: Vec<Vec<char>> = both_halves
        .iter()
        .skip(1)
        .map(|set| set.chars().collect::<Vec<_>>())
        .collect();
    let duplicate_char = both_halves[0]
        .chars()
        .find(|compare| chars_second_half.iter().all(|set2| set2.contains(compare)))
        .expect(
            "No duplicates found", //format!("No Duplicates are found in: {:?}", both_halves.as_str());
        );
    duplicate_char
}

pub fn find_common_chars(s1: &str, s2: &str, s3: &str) -> Vec<char> {
    let set1: std::collections::HashSet<_> = s1.chars().collect();
    let set2: std::collections::HashSet<_> = s2.chars().collect();
    let set3: std::collections::HashSet<_> = s3.chars().collect();

    let common_chars = set1
        .intersection(&set2)
        .filter(|&ch| set3.contains(ch))
        .cloned()
        .collect::<Vec<_>>();

    common_chars
}

pub fn char_to_enum(duplicate_char: char) -> Priorities {
    match duplicate_char {
        'a' => Priorities::a,
        'b' => Priorities::b,
        'c' => Priorities::c,
        'd' => Priorities::d,
        'e' => Priorities::e,
        'f' => Priorities::f,
        'g' => Priorities::g,
        'h' => Priorities::h,
        'i' => Priorities::i,
        'j' => Priorities::j,
        'k' => Priorities::k,
        'l' => Priorities::l,
        'm' => Priorities::m,
        'n' => Priorities::n,
        'o' => Priorities::o,
        'p' => Priorities::p,
        'q' => Priorities::q,
        'r' => Priorities::r,
        's' => Priorities::s,
        't' => Priorities::t,
        'u' => Priorities::u,
        'v' => Priorities::v,
        'w' => Priorities::w,
        'x' => Priorities::x,
        'y' => Priorities::y,
        'z' => Priorities::z,
        'A' => Priorities::A,
        'B' => Priorities::B,
        'C' => Priorities::C,
        'D' => Priorities::D,
        'E' => Priorities::E,
        'F' => Priorities::F,
        'G' => Priorities::G,
        'H' => Priorities::H,
        'I' => Priorities::I,
        'J' => Priorities::J,
        'K' => Priorities::K,
        'L' => Priorities::L,
        'M' => Priorities::M,
        'N' => Priorities::N,
        'O' => Priorities::O,
        'P' => Priorities::P,
        'Q' => Priorities::Q,
        'R' => Priorities::R,
        'S' => Priorities::S,
        'T' => Priorities::T,
        'U' => Priorities::U,
        'V' => Priorities::V,
        'W' => Priorities::W,
        'X' => Priorities::X,
        'Y' => Priorities::Y,
        'Z' => Priorities::Z,
        _ => Priorities::None,
    }
}

pub fn priorities_scores(priorities: Priorities) -> u32 {
    match priorities {
        Priorities::a => 1,
        Priorities::b => 2,
        Priorities::c => 3,
        Priorities::d => 4,
        Priorities::e => 5,
        Priorities::f => 6,
        Priorities::g => 7,
        Priorities::h => 8,
        Priorities::i => 9,
        Priorities::j => 10,
        Priorities::k => 11,
        Priorities::l => 12,
        Priorities::m => 13,
        Priorities::n => 14,
        Priorities::o => 15,
        Priorities::p => 16,
        Priorities::q => 17,
        Priorities::r => 18,
        Priorities::s => 19,
        Priorities::t => 20,
        Priorities::u => 21,
        Priorities::v => 22,
        Priorities::w => 23,
        Priorities::x => 24,
        Priorities::y => 25,
        Priorities::z => 26,
        Priorities::A => 27,
        Priorities::B => 28,
        Priorities::C => 29,
        Priorities::D => 30,
        Priorities::E => 31,
        Priorities::F => 32,
        Priorities::G => 33,
        Priorities::H => 34,
        Priorities::I => 35,
        Priorities::J => 36,
        Priorities::K => 37,
        Priorities::L => 38,
        Priorities::M => 39,
        Priorities::N => 40,
        Priorities::O => 41,
        Priorities::P => 42,
        Priorities::Q => 43,
        Priorities::R => 44,
        Priorities::S => 45,
        Priorities::T => 46,
        Priorities::U => 47,
        Priorities::V => 48,
        Priorities::W => 49,
        Priorities::X => 50,
        Priorities::Y => 51,
        Priorities::Z => 52,
        Priorities::None => 0,
    }
}
