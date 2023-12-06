use std::fs;
use std::collections::HashMap;

struct RangedMaps {
    name: String,
    source_range_start: Vec<u32>,
    source_destination_range_map: HashMap<u32, (u32, u32)>
}
impl RangedMaps {
    fn register_range_map(&mut self, dest: u32, src: u32, range: u32) {
        self.source_range_start.push(src);
        self.source_range_start.sort();
        
        self.source_destination_range_map.insert(src, (dest, range));
    }

    fn get_destination(&self, src: u32) -> u32 {
        let mut destination = src;
        let mut left_index: usize = 0;
        let mut right_index = self.source_range_start.len() - 1;
        let source_range;
        let mut i = (right_index + left_index) / 2;
        loop {
            if i == self.source_range_start.len() - 1 {
                source_range = self.source_range_start[i];
                break;
            }
            else if left_index == right_index {
                source_range = self.source_range_start[left_index];
                break;
            }
            else if i == 0 && src < self.source_range_start[i] {
                return src;
            }
            else if src >= self.source_range_start[i] && src < self.source_range_start[i+1] {
                source_range = self.source_range_start[i];
                break;
            }

            if src < self.source_range_start[i] {
                right_index = i;
            }
            else if src >= self.source_range_start[i+1] {
                left_index = i+1;
            }

            i = (right_index + left_index) / 2;
        }

        let destination_range = 
            self.source_destination_range_map
            .get(&source_range)
            .unwrap();

        let destination_range_start = destination_range.0;
        let range = destination_range.1;
        let source_range_diff = src - source_range;

        if source_range_diff <= range-1 {
            destination = destination_range_start + source_range_diff;
        }

        destination
    }
}

fn main() {
    let input = 
        fs::read_to_string("sample.txt")
        .unwrap();

    let mut input_segments = input.split("\n\n");

    let seeds = 
        input_segments.next().unwrap()
        .split(": ").last().unwrap()
        .split(" ")
        .map(|num| num.parse::<u32>().unwrap());

    let mut maps: Vec<RangedMaps> = vec![];
    for segment in input_segments {

        let mut segment_parts = segment.split(":\n");

        let mut map = RangedMaps {
            name: segment_parts.next().unwrap().to_string(),
            source_range_start: vec![],
            source_destination_range_map: HashMap::new()
        };


        for range_map in segment_parts.next().unwrap().split("\n") {
            let params = 
                range_map
                .split(" ")
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            map.register_range_map(params[0], params[1], params[2]);
        }

        maps.push(map);
    }

    let mut min = u32::MAX;
    for seed in seeds.clone() {
        let mut temp = seed;
        for map in maps.iter() {
            temp = map.get_destination(temp);
        }
        if temp < min {
            min = temp;
        }
    }

    println!("{}", min);

    // Part 2
    // let seeds_2: Vec<u32> = seeds.collect();
    // let mut seeds_range: Vec<u32> = (seeds_2[0]..seeds_2[0]+seeds_2[1]).collect::<Vec<u32>>();
    // seeds_range.append(&mut (seeds_2[2]..seeds_2[2]+seeds_2[3]).collect::<Vec<u32>>());

    // println!("{}", seeds_range.len());
    // let mut min = u32::MAX;
    // for seed in seeds_range {
    //     let mut temp = seed;
    //     for map in maps.iter() {
    //         temp = map.get_destination(temp);
    //     }
    //     if temp < min {
    //         min = temp;
    //     }
    // }

    // println!("{}", min);

}
