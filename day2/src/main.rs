fn main() {
    let mut memory = day2::read_program_from_file("input2.txt");
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

    let memory = day2::read_program_from_file("input5.txt");

    let outputs = day2::day5(&memory, &vec!(1));
    let outs:Vec<String> = outputs.iter().map(|n| n.to_string()).collect();
    println!("Day 5 part 1: {}", outs.join(", "));

    let outputs = day2::day5(&memory, &vec!(5));
    let outs:Vec<String> = outputs.iter().map(|n| n.to_string()).collect();
    println!("Day 5 part 2: {}", outs.join(", "));

    let memory = day2::read_program_from_file("input7.txt");
    let (phases, highest) = day2::day7(&memory, (0..5).collect());
    println!("Day 7 part 1: phases {:?}, high {}", phases, highest);

    let (phases, highest) = day2::day7(&memory, (5..10).collect());
    println!("Day 7 part 2: phases {:?}, high {}", phases, highest);

    let memory = day2::read_program_from_file("input9.txt");
    let outputs = day2::day5(&memory, &vec!(1));
    let outs:Vec<String> = outputs.iter().map(|n| n.to_string()).collect();
    println!("Day 9 part 1: {}", outs.join(", "));

    let outputs = day2::day5(&memory, &vec!(2));
    let outs:Vec<String> = outputs.iter().map(|n| n.to_string()).collect();
    println!("Day 9 part 2: {}", outs.join(", "));

}

