mod util;

use std::io;
use std::fs::File;
use std::cmp::max;
use util::read_lines;

#[derive(Debug, Copy, Clone)]
pub struct Line {
    begin: (usize, usize),
    end: (usize, usize),
}

impl Line {

    pub fn from(x1: usize, y1: usize, x2: usize, y2: usize) -> Line {
        Line {
            begin: (x1, y1),
            end: (x2, y2),
        }
    }

    pub fn is_straight(&self) -> bool {
        self.begin.0 == self.end.0 || self.begin.1 == self.end.1
    }

    /** Component of Bresenham's line drawing algorithm where y = mx + b
     */
    fn low_coordinate_range(&self, begin: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
        let dx = (end.0 as i64) - (begin.0 as i64);
        let mut dy = (end.1 as i64) - (begin.1 as i64);
        let mut yi: i64 = 1;
        if dy < 0 {
            yi = -1;
            dy = -dy;
        }
        let mut D = (2 * (dy as i64)) - (dx as i64);
        let mut y = begin.1 as i64;

        let mut plot: Vec<(usize, usize)> = Vec::new();
        for x in (begin.0 as i64)..((end.0 + 1) as i64) {
            plot.push((x as usize, y as usize));
            if D > 0 {
                y = y + yi;
                D = D + (2 * (dy - dx));
            } else {
                D = D + (2 * dy);
            }
        }
        plot
    }

    /** Component of Bresenham's line drawing algorithm where x = my + b
     */
    fn high_coordinate_range(&self, begin: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
        let mut dx = (end.0 as i64) - (begin.0 as i64);
        let dy = (end.1 as i64) - (begin.1 as i64);
        let mut xi: i64 = 1;
        if dx < 0 {
            xi = -1;
            dx = -dx;
        }
        let mut D = (2 * (dx as i64)) - (dy as i64);
        let mut x = begin.0 as i64;

        let mut plot: Vec<(usize, usize)> = Vec::new();
        for y in (begin.1 as i64)..((end.1 + 1) as i64) {
            plot.push((x as usize, y as usize));
            if D > 0 {
                x = x + xi;
                D = D + (2 * (dx - dy));
            } else {
                D = D + (2 * dx);
            }
        }
        plot
    }

    /** Bresenham's line drawing algorithm my bitches. Miracles of math!
     */
    pub fn coordinate_range(&self) -> Vec<(usize, usize)> {
        if ((self.end.1 as i64) - (self.begin.1 as i64)).abs() < ((self.end.0 as i64) - (self.begin.0 as i64)).abs() {
            if self.begin.0 > self.end.0 {
                return self.low_coordinate_range(self.end, self.begin);
            } else {
                return self.low_coordinate_range(self.begin, self.end);
            }
        } else {
            if self.begin.1 > self.end.1 {
                return self.high_coordinate_range(self.end, self.begin);
            } else {
                return self.high_coordinate_range(self.begin, self.end);
            }
        }
    }

}

pub fn parse_input(mut input: io::Lines<io::BufReader<File>>) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    while let Some(Ok(line)) = input.next() {
        let parts: Vec<&str> = line.split(&[',', ' '][..]).collect();
        let l = Line::from(
            parts[0].parse::<usize>().unwrap(),
            parts[1].parse::<usize>().unwrap(),
            parts[3].parse::<usize>().unwrap(),
            parts[4].parse::<usize>().unwrap(),
        );
        lines.push(l);
    }
    lines
}

pub fn part1(lines: Vec<Line>) -> usize {
    let straight_lines: Vec<Line> = lines.into_iter()
        .filter(|x| x.is_straight())
        .collect();
    let x_max = 1 + straight_lines.iter().fold(0, |acc, l| max(max(acc, l.begin.0), l.end.0));
    let y_max = 1 + straight_lines.iter().fold(0, |acc, l| max(max(acc, l.begin.1), l.end.1));
    let mut map: Vec<usize> = vec![0; x_max * y_max];
    for line in straight_lines {
        for c in line.coordinate_range() {
            map[c.0 + (c.1 * x_max)] += 1;
        }
    }
    let lower_bound: usize = 2;
    map.into_iter().filter(|x| x >= &lower_bound).count()
}

pub fn part2(lines: Vec<Line>) -> usize {
    let x_max = 1 + lines.iter().fold(0, |acc, l| max(max(acc, l.begin.0), l.end.0));
    let y_max = 1 + lines.iter().fold(0, |acc, l| max(max(acc, l.begin.1), l.end.1));
    let mut map: Vec<usize> = vec![0; x_max * y_max];
    for line in lines {
        for c in line.coordinate_range() {
            map[c.0 + (c.1 * x_max)] += 1;
        }
    }
    let lower_bound: usize = 2;
    map.into_iter().filter(|x| x >= &lower_bound).count()
}


pub fn main() {
    if let Ok(input) = read_lines("inputs/day5.txt") {
        let lines = parse_input(input);
        println!("Part 1: {}", part1(lines.clone()));
        println!("Part 2: {}", part2(lines));
    }    
}
