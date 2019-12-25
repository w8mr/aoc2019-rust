fn main() {
    let memory = intcode::read_program_from_file("input9.txt");
    let outputs = intcode::run_input_output(&memory, &vec!(1));
    let outs:Vec<String> = outputs.iter().map(|n| n.to_string()).collect();
    println!("Day 9 part 1: {}", outs.join(", "));

    let outputs = intcode::run_input_output(&memory, &vec!(2));
    let outs:Vec<String> = outputs.iter().map(|n| n.to_string()).collect();
    println!("Day 9 part 2: {}", outs.join(", "));

}

