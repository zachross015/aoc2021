use std::fs;


fn get_input() -> Vec<i32> {

    // Read the file contents
    let contents = fs::read_to_string("inputs/day1.txt")
        .expect("Something went wrong reading the file.");

    // Split the string and parse into integers. There tends to be a \n at the
    // end of the file, so we filter out the bad results.
    contents.split("\n")
        .filter_map(|x| x.parse::<i32>().ok())
        .collect()
}

fn num_increasing(inp: Vec<i32>) -> i32 {
    let mut i = 0;
    for j in 1..inp.len() {
        if inp[j] > inp[j - 1] {
            i = i + 1;
        }
    }
    i
}

fn sliding_sum(inp: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut prev_sum = inp[0] + inp[1] + inp[2];
    for j in 3..inp.len() {
        let new_sum = prev_sum - inp[j - 3] + inp[j];
        if new_sum > prev_sum {
            i = i + 1;
        }
        prev_sum = new_sum;
    }
    i
}

pub fn main() {

    let input = get_input();
    
    // let part1 = num_increasing(input);
    // println!("Part 1 results: {}", part1);

    let part2 = sliding_sum(input);
    println!("Part 2 results: {}", part2);
}
