use std::fs;
struct SetCount {
    red: i32,
    green: i32,
    blue: i32
}
fn main() {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let input = 
        fs::read_to_string("input.txt")
        .unwrap();
    let games = 
        input.split("\n")
        .map(str::to_string);
    let sum_of_ids: i32 = 
        games.clone()
        .map(
            move |game| get_game_value(game, max_red, max_green, max_blue)
        )
        .sum();
    println!("{}", sum_of_ids);

    let sum_of_game_powers: i32 = 
        games
        .map(get_game_power)
        .sum();
    println!("{}", sum_of_game_powers)
}

fn get_game_power(game: String) -> i32 {
    let minimum_game_cube_count =
        game.split(": ")
        .collect::<Vec<&str>>()[1]
        .split("; ")
        .map(get_set_count)
        .fold(
        SetCount { red: 0, green: 0, blue: 0 },
            |acc: SetCount, set: SetCount| SetCount { 
                red: acc.red.max(set.red),  
                green: acc.green.max(set.green),  
                blue: acc.blue.max(set.blue),  
            }        
        );
    minimum_game_cube_count.red * minimum_game_cube_count.green * minimum_game_cube_count.blue
}

fn get_set_count(set: &str) -> SetCount {
    let mut set_count = SetCount { red: 0, green: 0, blue: 0 };

    for color_count in set.split(", ") {
        let count_color_split = 
            color_count.split(" ")
            .collect::<Vec<&str>>();
        let count = 
            count_color_split[0]
            .to_string()
            .parse::<i32>()
            .unwrap();
        let color = count_color_split[1];
        match color {
            "red" => set_count.red = count,
            "green" => set_count.green = count,
            "blue" => set_count.blue = count,
            _ => break
        }
    }

    set_count

}

fn get_game_value(game: String, max_red: i32, max_green: i32, max_blue: i32) -> i32 {
    let id_sets_split = 
        game.split(": ")
        .collect::<Vec<&str>>();
    let id = 
        id_sets_split[0]
        .split(" ")
        .collect::<Vec<&str>>()[1]
        .to_string()
        .parse::<i32>()
        .unwrap();
    let is_valid_game = 
        id_sets_split[1]
        .split("; ")
        .map(
            move |set| is_valid_set(set, max_red, max_green, max_blue)
        )
        .fold(
            true, 
            |acc, is_valid| acc && is_valid
        ); 

    if is_valid_game {
        id
    } else {
        0
    }
}

fn is_valid_set(set: &str, max_red: i32, max_green: i32, max_blue: i32) -> bool {
        set.split(", ")
        .map(
            move |color| is_valid_color_count(color, max_red, max_green, max_blue)
        )
        .fold(
            true, 
            |acc, is_valid| acc && is_valid
        )
}

fn is_valid_color_count(color: &str, max_red: i32, max_green: i32, max_blue: i32) -> bool {
    let count_color_split = 
        color.split(" ")
        .collect::<Vec<&str>>();
    let count = 
        count_color_split[0]
        .to_string()
        .parse::<i32>()
        .unwrap();
    let color = count_color_split[1];

    match color {
        "red" => max_red >= count,
        "green" => max_green >= count,
        "blue" => max_blue >= count,
        _ => false
    }

}