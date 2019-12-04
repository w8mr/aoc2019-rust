use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);
    let wires: Vec<_> = file.lines().map(|l| l.unwrap()).collect();
    println!("Day 1: {}",day3::part1(wires));
}