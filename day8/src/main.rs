use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let f = File::open("input8.txt").unwrap();
    let file = BufReader::new(&f);
    let lines: Vec<_> = file.lines().collect();
    println!("Day 8 part 1: {}", day8::day8_part1(lines[0].as_ref().unwrap(), 25, 6));

    let f = File::open("input8.txt").unwrap();
    let file = BufReader::new(&f);
    let lines: Vec<_> = file.lines().collect();
    println!("Day 8 part 2:");
    for line in day8::day8_part2(lines[0].as_ref().unwrap(), 25, 6) {
        println!("{}", line);

    }
}