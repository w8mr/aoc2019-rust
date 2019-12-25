fn main() {
    let memory = intcode::read_program_from_file("input5.txt");

    let outputs = day5::run_simulation(&memory, &vec!(1));
    let outs:Vec<String> = outputs.iter().map(|n| n.to_string()).collect();
    println!("Day 5 part 1: {}", outs.join(", "));

    let outputs = day5::run_simulation(&memory, &vec!(5));
    let outs:Vec<String> = outputs.iter().map(|n| n.to_string()).collect();
    println!("Day 5 part 2: {}", outs.join(", "));
}

