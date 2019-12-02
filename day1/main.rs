use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
                
fn main() {   
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);
    let lines: Vec<_> = file.lines().collect();
    let numbers: Vec<i32> = lines.iter().map(|s| s.as_ref().unwrap().parse().unwrap()).collect();
    println!("ansmer day 1: {}", numbers.iter().map(|i| calc_fuel(i)).fold(0, |acc, i| acc + i));
    println!("ansmer day 2: {}", numbers.iter().map(|i| calc_fuel_recursive(i)).fold(0, |acc, i| acc + i));
}   

fn calc_fuel(mass:&i32) -> i32 {
    return mass / 3 - 2;
}

fn calc_fuel_recursive(mass:&i32) -> i32 {
   let fuel = calc_fuel(mass);
    return match fuel {
        f if f <= 0 => 0,
        _ => fuel + calc_fuel_recursive(&fuel)
    }
}