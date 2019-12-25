fn main() {
    let memory = intcode::read_program_from_file("input5.txt");

    let outputs = intcode::run_input_output(&memory, &vec!(1));
    let outs:Vec<String> = outputs.iter().map(|n| n.to_string()).collect();
    println!("Day 5 part 1: {}", outs.join(", "));

    let outputs = intcode::run_input_output(&memory, &vec!(5));
    let outs:Vec<String> = outputs.iter().map(|n| n.to_string()).collect();
    println!("Day 5 part 2: {}", outs.join(", "));
}

