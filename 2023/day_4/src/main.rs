use std::collections::hash_set::Iter;
use std::fs;
use std::collections::HashSet;

fn main() {
    let card_sum: i32 = 
        fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(
            |card|
                card
                .split(": ")
                .collect::<Vec<&str>>()[1]
                .split(" | ")
                .map(
                    |side|
                        side
                        .split_whitespace()
                )
                .map(HashSet::from_iter)
                .collect::<Vec<HashSet<&str>>>()
        )
        .map(intersection)
        .map(|card| card.len())
        .filter(|len| len > &0)
        .map(|len| (2 as i32).pow((len as u32)-1))
        .sum();

    println!("{}", card_sum);

}

fn intersection(sets: Vec<HashSet<&str>>) -> Vec<&str> {
    sets[0]
    .intersection(&sets[1])
    .map(|num| *num)
    .collect::<Vec<&str>>()
}