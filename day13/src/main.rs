fn main() {
    let mut memory = intcode::read_program_from_file("input13.txt");
    println!("Day 13 part 1: {}", day13::run_simulation_part1(&mut memory, true, 5));
    println!("Day 13 part 2: {}", day13::run_simulation_part2(&mut memory, true, 5));
}

