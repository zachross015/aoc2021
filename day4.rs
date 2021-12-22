mod util;

use std::io;
use std::fs::File;
use util::read_lines;
use std::collections::HashMap;

// Max board size
const BOARD_SIZE: usize = 25;

// Breakdown of all the win scenarios for different bit board configurations
const WINS: [usize; 10] = [
    0b11111,
    0b1111100000,
    0b111110000000000,
    0b11111000000000000000,
    0b1111100000000000000000000,
    0b1000010000100001000010000,
    0b100001000010000100001000,
    0b10000100001000010000100,
    0b1000010000100001000010,
    0b100001000010000100001,
];


/** Function which determines whether or not a board is in a win state by
 * comparing against all different bit board wins.
 */
fn did_win(bit_board: usize) -> bool {
    for i in WINS {
        if (bit_board & i) == i {
            return true;
        }
    }
    false
}

/** Marks a specific entry as being correct in the given bitboard.
 */
fn mark_correct(board: &mut usize, location: usize) {
    let offset = location % BOARD_SIZE;
    *board = *board | (1 << offset);
}

/* Finds the sum of all values not uncovered in the bingo game. 
 */
fn sum_missing(bit_board: usize, board: HashMap<usize, usize>) -> usize {
    let mut acc = 0;
    for (key, val) in board.iter() {
        if ((bit_board >> val) & 1) == 0 {
            acc += key;
        }
    }
    acc
}


pub fn part1(call_order: Vec<usize>, boards: Vec<HashMap<usize, usize>>) -> usize {
    let mut bit_board: Vec<usize> = vec![0; boards.len()];
    for call in call_order {
        for j in 0..boards.len() {
            if let Some(idx) = boards[j].get(&call) {
                mark_correct(&mut bit_board[j], *idx);
                if did_win(bit_board[j]) {
                    return sum_missing(bit_board[j], boards[j].clone()) * call;
                }
            }
        }
    }
    0
}

pub fn part2(call_order: Vec<usize>, boards: Vec<HashMap<usize, usize>>) -> usize {
    let mut bit_board: Vec<usize> = vec![0; boards.len()];

    let mut has_won: Vec<bool> = vec![false; boards.len()];
    let mut num_winners_remaining: usize = boards.len();

    for call in call_order {
        for j in 0..boards.len() {
            if let Some(idx) = boards[j].get(&call) {
                mark_correct(&mut bit_board[j], *idx);
                if did_win(bit_board[j]) && !has_won[j] {
                    if num_winners_remaining == 1 {
                        return sum_missing(bit_board[j], boards[j].clone()) * call;
                    } else {
                        num_winners_remaining -= 1;
                        has_won[j] = true;
                    }
                }
            }
        }
    }
    0
}


fn parse_input(mut input: io::Lines<io::BufReader<File>>) -> (Vec<usize>, Vec<HashMap<usize, usize>>) {

    // First line of input contains the order of the bingo calls 
    let o_line = input.next();
    if let Some(order_line) = o_line {

        // Parse the call order
        let call_order = order_line.unwrap().split(",")
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        // Break the boards into mappings from the value to its index
        let mut boards: Vec<HashMap<usize, usize>> = Vec::new();
        while let Some(_) = input.next() {
            let mut board_mapping: HashMap<usize, usize> = HashMap::new();
            for i in 0..5 {
                if let Some(line) = input.next() {
                    let vals: Vec<usize> = line.unwrap().split(" ")
                                 .filter(|x| (*x).ne(""))
                                 .filter_map(|x| x.parse::<usize>().ok())
                                 .collect();
                    for j in 0..5 {
                        board_mapping.insert(vals[j], (i * 5) + j);
                    }
                }
            }
            boards.push(board_mapping);
        }
        return (call_order, boards);
    }
    panic!("IDK, something happened with the input.")
}


pub fn main() {
    if let Ok(lines) = read_lines("inputs/day4.txt") {
        let (call_order, boards) = parse_input(lines);
        println!("Part 1: {}", part1(call_order.clone(), boards.clone()));
        println!("Part 2: {}", part2(call_order, boards));
    }    
}
