fn main() {
    let memory = intcode::read_program_from_file("input7.txt");
    let (phases, highest) = day7::run_simulation(&memory, (0..5).collect());
    println!("Day 7 part 1: phases {:?}, high {}", phases, highest);

    let (phases, highest) = day7::run_simulation(&memory, (5..10).collect());
    println!("Day 7 part 2: phases {:?}, high {}", phases, highest);
}

