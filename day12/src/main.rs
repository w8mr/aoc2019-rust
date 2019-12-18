use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use day12::day12_part1;

fn main() {
    let f = File::open("input12.txt").unwrap();
    let file = BufReader::new(&f);
    let lines: Vec<_> = file.lines().collect();
    let numbers: Vec<Vec<i32>> = lines.iter().map(|line| day12::parse_input_line(line.as_ref().unwrap().as_str())).collect();
    println!("Day 12 part 1: {}", day12_part1(numbers, 1000));
}