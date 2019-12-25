use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let f = File::open("input14.txt").unwrap();
    let file = BufReader::new(&f);
    let reactions: Vec<String> = file.lines().map(|l| l.unwrap()).collect();
    let reactions= reactions.iter().map(|r| r.as_str()).collect();
    println!("Day14 part1 {}", day14::part1(&reactions));
    println!("Day14 part2 {}", day14::part2(&reactions));
}