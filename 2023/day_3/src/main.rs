use std::fs;
use std::collections::HashSet;

type Grid<T> = Vec<Vec<T>>;
type PartNumberIndex = ((usize, usize), usize);
fn main() {
    // Part 1
    let input_grid: Grid<char> = 
        fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(str::to_string)
        .map(
            |s| 
                s.chars()
                .collect()
        )
        .collect();

    let mut symbol_indexes: Vec<(usize, usize)> = vec![];
    for (y, row) in input_grid.iter().enumerate() {
        for (x, character) in row.iter().enumerate() {
            if is_symbol(character) {
                symbol_indexes.push((x, y));
            }
        }
    }

    let mut part_number_index_set:HashSet<PartNumberIndex> = HashSet::new();
    for symbol in symbol_indexes.iter() {
        for digit in adjacent_digit_indexes_from_symbol_index(symbol.clone(), &input_grid).iter() {
            part_number_index_set.insert(part_number_index_from_digit_index(digit.clone(), &input_grid));
        }
    }

    let part_number_sum: i32 =
        part_number_index_set
        .iter()
        .map(
            move |part_number_index| 
            part_number_from_index(part_number_index.0, part_number_index.1, &input_grid)
        )
        .sum();
    println!("{}", part_number_sum);

    // Part 2
    let input_grid_2: Grid<char> = 
        fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(str::to_string)
        .map(
            |s| 
                s.chars()
                .collect()
        )
        .collect();

    let mut star_symbol_indexes: Vec<(usize, usize)> = vec![];
    for (y, row) in input_grid_2.iter().enumerate() {
        for (x, character) in row.iter().enumerate() {
            if character.eq(&'*') {
                star_symbol_indexes.push((x, y));
            }
        }
    }

    let mut gear_ratios: Vec<i32> = vec![];
    for symbol in star_symbol_indexes.iter() {
        let mut symbol_adjacent_part_numbers:HashSet<PartNumberIndex> = HashSet::new();
        for digit in adjacent_digit_indexes_from_symbol_index(symbol.clone(), &input_grid_2).iter() {
            symbol_adjacent_part_numbers.insert(part_number_index_from_digit_index(digit.clone(), &input_grid_2));
        }
        if symbol_adjacent_part_numbers.len() == 2 {
            let part_numbers = 
                symbol_adjacent_part_numbers
                .iter()
                .collect::<Vec<&PartNumberIndex>>();
            let gear_ratio = 
                part_number_from_index(part_numbers[0].0, part_numbers[0].1, &input_grid_2) *
                part_number_from_index(part_numbers[1].0, part_numbers[1].1, &input_grid_2);
            gear_ratios.push(gear_ratio);
        }
    }

    let part_number_sum: i32 =
        gear_ratios
        .iter()
        .sum();
    println!("{}", part_number_sum);
}

fn is_symbol(character: &char) -> bool {
    !(character.is_ascii_digit() || character.eq(&'.'))
}

fn adjacent_digit_indexes_from_symbol_index(index: (usize, usize), grid: &Grid<char>) -> Vec<(usize, usize)> {
    let mut adjacent_digit_indexes = vec![];
    for offset_y in -1..=1 {
        for offset_x in -1..=1 {
            let x = (index.0 as i32 + offset_x) as usize;
            let y = (index.1 as i32 + offset_y) as usize;

            if grid[y][x].is_numeric() {
                adjacent_digit_indexes.push((x, y));
            }
        }
    }
    adjacent_digit_indexes
}

fn part_number_index_from_digit_index(index: (usize, usize), grid: &Grid<char>) -> PartNumberIndex {
    let y = index.1;
    let grid_length = grid[y].len() as i32;
    let mut left_digit_index = index.0;
    let mut right_digit_index = index.0;

    let mut found_first_index = false;
    let mut found_last_index = false;

    loop {
        if left_digit_index as i32 - 1 >= 0 {
            if grid[y][left_digit_index-1].is_numeric() {
                left_digit_index -= 1;
            }
            else {
                found_first_index = true;
            }
        }
        else {
            found_first_index = true;
        }

        if right_digit_index as i32 + 1 < grid_length {
            if grid[y][right_digit_index+1].is_numeric() {
                right_digit_index += 1;
            }
            else {
                found_last_index = true;
            }
        }
        else {
            found_last_index = true;
        }

        if found_first_index && found_last_index {
            break;
        }
    }

    ((left_digit_index, right_digit_index), y)
}

fn part_number_from_index(index: (usize, usize), y: usize, grid: &Grid<char>) -> i32 {
    let mut part_number_string = "".to_string(); 
    for i in index.0..=index.1 {
        part_number_string.push(grid[y][i]);
    } 

    part_number_string
        .parse::<i32>()
        .unwrap()
} 