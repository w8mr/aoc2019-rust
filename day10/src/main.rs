use std::io::{BufReader, Error};
use std::io::BufRead;
use std::fs::File;

fn main() {
    let f = File::open("input10.txt").unwrap();
    let file = BufReader::new(&f);
    let raw_lines: Vec<Result<String, Error>> = file.lines().collect();
    let grid_lines: Vec<&str> = raw_lines.iter().map(|l| l.as_ref().unwrap()).map(|s|s.as_str()).collect();
    let winner = day10::part1(grid_lines);
    println!("Day 10 part 1: Astroid ({},{}) can see {} Astroids", (winner.0).0, (winner.0).1, winner.1);
}