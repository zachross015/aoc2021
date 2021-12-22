mod util;

use std::io;
use std::fs::File;
use std::collections::LinkedList;
use util::read_lines;

const MAX_LENGTH: u64 = 11;

pub fn partition(list: LinkedList<u64>, bit_index: u64) -> (LinkedList<u64>, LinkedList<u64>) {
    let mut left_list: LinkedList<u64> = LinkedList::new();
    let mut right_list: LinkedList<u64> = LinkedList::new();

    for item in list.iter() {
        let val = (item >> bit_index) & 1;
        if val == 1 {
            left_list.push_back(*item);
        } else {
            right_list.push_back(*item);
        }
    }

    (left_list, right_list)
}

pub fn find_oxygen_generator_rating(list: LinkedList<u64>) -> u64 {
    let mut i = MAX_LENGTH;
    let mut modified_list = list;
    while modified_list.len() != 1 {
        let (one_list, zero_list) = partition(modified_list, i);
        if one_list.len() >= zero_list.len() {
            modified_list = one_list;
        } else {
            modified_list = zero_list;
        }

        // Should never run if the program always reduces to 1 element.
        if i > 0 {
            i = i - 1;
        }
    }
    *modified_list.front().unwrap()
}

pub fn find_co2_scrubber_rating(list: LinkedList<u64>) -> u64 {
    let mut i = MAX_LENGTH;
    let mut modified_list = list;
    while modified_list.len() != 1 {
        let (one_list, zero_list) = partition(modified_list, i);
        if one_list.len() < zero_list.len() {
            modified_list = one_list;
        } else {
            modified_list = zero_list;
        }

        // Should never run if the program always reduces to 1 element.
        if i > 0 {
            i = i - 1;
        }
    }
    *modified_list.front().unwrap()
}

pub fn part2(input: Vec<u64>) -> u64 {
    let mut list = LinkedList::new(); 
    list.extend(input);
    // let (ones, zeros) = partition(list, 4);
    // println!("{:?} {:?}", ones, zeros);
    let o_rating = find_oxygen_generator_rating(list.clone());
    let co2_rating = find_co2_scrubber_rating(list);
    o_rating * co2_rating
}


fn parse_input(input: io::Lines<io::BufReader<File>>) -> Vec<u64> {
    input.filter_map(|x| match x {
        Ok(y) => u64::from_str_radix(&y, 2).ok(),
        Err(_) => None
    }).collect()
}


pub fn main() {
    if let Ok(lines) = read_lines("inputs/day3.txt") {
        let inp = parse_input(lines);
        println!("Part 2: {}", part2(inp));
    }    
}
