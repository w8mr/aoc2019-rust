use std::io::{BufReader, Error};
use std::io::BufRead;
use std::fs::File;

fn main() {
    let f = File::open("input10.txt").unwrap();
    let file = BufReader::new(&f);
    let raw_lines: Vec<Result<String, Error>> = file.lines().collect();
    let grid_lines: Vec<&str> = raw_lines.iter().map(|l| l.as_ref().unwrap()).map(|s|s.as_str()).collect();
    let winner = day10::part1(grid_lines.clone());
    println!("Day 10 part 1: Asteroid ({},{}) can see {} Astroids", (winner.0).0, (winner.0).1, winner.1);
    let destroyed = day10::part2(grid_lines, 199);
    println!("Day 10 part 2: Asteroid ({},{}) is destroyed as 200th asteroid", destroyed.0, destroyed.1);

}

