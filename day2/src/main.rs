fn main() {
    let mut memory = intcode::read_program_from_file("input2.txt");
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
}

