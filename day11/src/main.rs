fn main() {
    let memory = intcode::read_program_from_file("input11.txt");
    println!("Day 11 part 1: {}", day11::run_simulation_part1(&memory, day11::Color::Black));

    println!("Day 11 part 2: ");
    let lines = day11::run_simulation_part2(&memory, day11::Color::White);
    for line in lines {
        let str:String = line.into_iter().collect();
        println!("{}", str);
    }
}

