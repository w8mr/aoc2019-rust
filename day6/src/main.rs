use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use day6::*;

fn main() {
    let f = File::open("input6.txt").unwrap();
    let file = BufReader::new(&f);
    let orbits: Vec<_> = file.lines().map(|s| s.unwrap()).collect();
    println!("Day6 part 1: {}", part1(&orbits));
    println!("Day6 part 2: {}", part2(&orbits));
}
