use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let f = File::open("input2.txt").unwrap();
    let file = BufReader::new(&f);
    let mut memory: Vec<_> = file.lines().next().unwrap().unwrap().split(",").map(|s| s.parse().unwrap()).collect();
    memory[1]=12;
    memory[2]=02;
    println!("Day 2 part 1: {}",day2::day2(&memory));

//1202 4138687
//1302 4426687 288000
//1402 4714687 576000

//1203 4138688 1
//1303 4426688 288001

//0000 682685

//19690720
//720 - 685 = 35
//19690 - 682 = 19008
//19008 / 288 = 66

// 6635


    memory[1]=66;
    memory[2]=35;
    println!("Day 2 part 2: {}",day2::day2(&memory));

    let f = File::open("input5.txt").unwrap();
    let file = BufReader::new(&f);
    let memory: Vec<_> = file.lines().next().unwrap().unwrap().split(",").map(|s| s.parse().unwrap()).collect();

    let outputs = &mut Vec::new();
    day2::day5(&memory, &vec!(1), outputs);
    let outs:Vec<String> = outputs.iter().map(|n| n.to_string()).collect();
    println!("Day 5 part 1: {}", outs.join(", "));

    let outputs = &mut Vec::new();
    day2::day5(&memory, &vec!(5), outputs);
    let outs:Vec<String> = outputs.iter().map(|n| n.to_string()).collect();
    println!("Day 5 part 2: {}", outs.join(", "));

}

