use std::fs;
use std::collections::HashSet;

fn main() {
    // Part 1
    let input_lines = 
        fs::read_to_string("input.txt")
        .unwrap();

    let card_winning_numbers = 
        input_lines
        .split("\n")
        .map(
            |card|
                card
                .split(": ")
                .collect::<Vec<&str>>()[1]
                .split(" | ")
                .map(|side| side.split_whitespace())
                .map(HashSet::from_iter)
                .collect::<Vec<HashSet<&str>>>()
        )
        .map(intersection)
        .map(|card| card.len());
        
    let card_sum: i32 = 
        card_winning_numbers
        .clone()
        .filter(|len| len > &0)
        .map(|len| (2 as i32).pow((len-1) as u32))
        .sum();

    println!("{}", card_sum);

    // Part 2
    let mut card_copies = vec![1; 209];
    for (i, matches) in card_winning_numbers.enumerate() {
        for offset in 1..=matches {
            card_copies[i+offset] += card_copies[i];
        }
    }
    let total_copies: i32 = 
        card_copies
        .iter()
        .sum();

    println!("{}", total_copies);

}

fn intersection(sets: Vec<HashSet<&str>>) -> Vec<&str> {
    sets[0]
    .intersection(&sets[1])
    .map(|num| *num)
    .collect::<Vec<&str>>()
}