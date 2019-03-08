/// Compute the Scrabble score for a word.

#[macro_use] extern crate maplit;

pub fn score(word: &str) -> u64 {
    let map = hashmap!{
        'A' => 1,
        'B' => 3,
        'C' => 3,
        'D' => 2,
        'E' => 1,
        'F' => 4,
        'G' => 2,
        'H' => 4,
        'I' => 1,
        'J' => 8,
        'K' => 5,
        'L' => 1,
        'M' => 3,
        'N' => 1,
        'O' => 1,
        'P' => 3,
        'Q' => 10,
        'R' => 1,
        'S' => 1,
        'T' => 1,
        'U' => 1,
        'V' => 4,
        'W' => 4,
        'X' => 8,
        'Y' => 4,
        'Z' => 10,
    };
    
    word.to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .fold(0u64, |acc, c| acc + map.get(&c).unwrap())

}
