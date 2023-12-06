use std::fs;

fn main() {
    let input_strings = 
        fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let times = vec![
        input_strings[0]
        .split(":")
        .last().unwrap()
        // .split_whitespace() PART 1
        // .map(|num| num.parse::<f32>().unwrap())
        // .collect::<Vec<f32>>();
        .replace(" ", "")
        .parse::<f32>().unwrap()
    ];

    let distances= vec![  
        input_strings[1]
        .split(":")
        .last().unwrap()
        // .split_whitespace()
        // .map(|num| num.parse::<f32>().unwrap())
        // .collect::<Vec<f32>>();
        .replace(" ", "")
        .parse::<f32>().unwrap()
    ];

    println!("{:?}", times);
    println!("{:?}", distances);

    let mut better_recors_amounts: Vec<i32> = vec![];
    const EPSILON: f32 = 0.1;
    for i in 0..times.len() {
        let root_1 = 
            (-times[i] + (((times[i]).powf(2.0) - (4.0*(distances[i] + EPSILON)))).sqrt()) / -2.0;
        let root_2 = 
            (-times[i] - (((times[i]).powf(2.0) - (4.0*(distances[i] + EPSILON)))).sqrt()) / -2.0;

        let better_recors_amount = (root_2.floor() - root_1.floor()) as i32;
        println!("roots: {} {}", root_1, root_2);
        println!("{}", better_recors_amount);
        better_recors_amounts.push(better_recors_amount);
    }

    let product: i32 = better_recors_amounts.iter().product();
    println!("{}", product);
}
