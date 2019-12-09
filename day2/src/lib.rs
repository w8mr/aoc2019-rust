use std::collections::HashMap;
use std::borrow::Borrow;

struct Instruction {
    opcode: Opcode,
    operand_count: usize,
    implementation: fn(Vec<isize>, Vec<isize>, &mut Context) -> Option<isize>,
}

#[derive(std::fmt::Debug, Copy, Clone)]
enum Opcode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    Halt = 99,
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{{ opcode: {:#?}, operand_count: {} }}", self.opcode, self.operand_count)
    }
}

fn read(parameters: &Vec<isize>, modes: &Vec<isize>, context: &Context, index: usize) -> isize {
    let mode = match modes.get(index as usize) {
        Some(mode) => *mode,
        None => 0 as isize
    };
    if mode == 0 {
        context.memory[parameters[index] as usize]
    } else {
        parameters[index]
    }
}

fn add_implementation(parameters: Vec<isize>, modes: Vec<isize>, context: &mut Context) -> Option<isize> {
    context.memory[parameters[2] as usize] = read(&parameters, &modes, context, 0) + read(&parameters, &modes, context, 1);
    Some(4)
}


fn multiply_implementation(parameters: Vec<isize>, modes: Vec<isize>, context: &mut Context) -> Option<isize>  {
    context.memory[parameters[2] as usize] = read(&parameters, &modes, context, 0) * read(&parameters, &modes, context, 1);
    Some(4)
}

fn input_implementation(parameters: Vec<isize>, modes: Vec<isize>, context: &mut Context) -> Option<isize>  {
    context.memory[parameters[0] as usize] = context.inputs.pop().unwrap();
    Some(2)
}

fn output_implementation(parameters: Vec<isize>, modes: Vec<isize>, context: &mut Context) -> Option<isize>  {
    context.outputs.push(read(&parameters, &modes, context, 0));
    Some(2)
}


fn halt_implementation(parameters: Vec<isize>, modes: Vec<isize>, context: &mut Context) -> Option<isize>  {
    None
}

#[derive(std::fmt::Debug)]
struct Context<'a> {
    memory: Vec<isize>,
    inputs: Vec<isize>,
    outputs: &'a mut Vec<isize>,
}

fn split_instruction(opcode: isize) -> (isize, Vec<isize>) {
    let mut opcode = opcode;
    let basic_opcode = opcode % 100;
    opcode /= 100;
    let mut modes = Vec::new();

    while opcode > 0 {
        modes.push(opcode % 10);
        opcode /= 10;
    }
    (basic_opcode, modes)
}

pub fn day2(opcodes: &Vec<isize>) -> isize {
    let mut outputs = &mut Vec::new();
    day5(opcodes, &vec!(), outputs)
}

pub fn day5(opcodes: &Vec<isize>, inputs: &Vec<isize>, outputs: &mut Vec<isize>) -> isize {
    let instructions = vec!(
        Instruction { opcode: Opcode::Add, operand_count: 3, implementation: add_implementation },
        Instruction { opcode: Opcode::Multiply, operand_count: 3, implementation: multiply_implementation },
        Instruction { opcode: Opcode::Input, operand_count: 1, implementation: input_implementation },
        Instruction { opcode: Opcode::Output, operand_count: 1, implementation: output_implementation },
        Instruction { opcode: Opcode::Halt, operand_count: 0, implementation: halt_implementation }
    );
    let instructions: HashMap<isize, &Instruction> = instructions.iter().map(|i| (i.opcode as isize, i)).collect();
    let context = &mut Context { memory: opcodes.to_vec(), inputs: inputs.to_vec(), outputs: outputs };
    let mut offset: usize = 0;

    loop {
        let (modes, instruction, parameters) = parse_instruction(&instructions, &context, offset);

        match (instruction.implementation)(parameters, modes, context) {
            Some(offset_change) => {
                offset = (offset as isize + offset_change) as usize;
            }
            None => {
                break
            }
        }
    }

  //  println!("{:#?}", context);
    context.memory[0]
}

fn parse_instruction<'a>(instructions: &'a HashMap<isize, &Instruction>, context: &&'a mut Context, offset: usize) -> (Vec<isize>, &'a &'a Instruction, Vec<isize>) {
    let opcode = &context.memory[offset];
    let (opcode, modes) = split_instruction(*opcode);
    let instruction = &instructions[&opcode];
    let parameters = ((offset + 1)..(offset + 1 + instruction.operand_count)).map(|i| context.memory[i]).collect();
    (modes, instruction, parameters)
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;

    use super::*;

    #[test]
    fn test_day2_part1_opcode99() {
        assert_eq!(day2(&vec!(99)), 99);
    }

    #[test]
    fn test_day2_part1_opcode1() {
        assert_eq!(day2(&vec!(1, 5, 6, 0, 99, 2, 3)), 5);
    }

    #[test]
    fn test_day2_part1_opcode2() {
        assert_eq!(day2(&vec!(2, 5, 6, 0, 99, 2, 3)), 6);
    }

    #[test]
    fn test_day2_part1_example() {
        assert_eq!(day2(&vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50)), 3500);
    }

    #[test]
    fn test_day2_part1_assignment() {
        let f = File::open("input2.txt").unwrap();
        let file = BufReader::new(&f);
        let mut memory: Vec<_> = file.lines().next().unwrap().unwrap().split(",").map(|s| s.parse().unwrap()).collect();

        memory[1]=12;
        memory[2]=02;
        day2(&memory);
        assert_eq!(day2(&memory), 4138687);
    }

    #[test]
    fn test_day2_part2_assignment() {
        let f = File::open("input2.txt").unwrap();
        let file = BufReader::new(&f);
        let mut memory: Vec<_> = file.lines().next().unwrap().unwrap().split(",").map(|s| s.parse().unwrap()).collect();

        memory[1]=66;
        memory[2]=35;
        day2(&memory);
        assert_eq!(day2(&memory), 19690720);
    }

    #[test]
    fn test_day5_part1_opcode030499() {
        let mut outputs = &mut Vec::new();
        assert_eq!(day5(&vec!(3, 0, 4, 0, 99), &vec!(42), outputs), 42);
        assert_eq!(outputs.len(),1);
        assert_eq!(outputs[0],42);
    }

    #[test]
    fn test_split_opcode() {
        let (opcode, modes) = split_instruction(1002);
        assert_eq!(opcode, 2);
        assert_eq!(modes.len(), 2);
        assert_eq!(modes[0], 0);
        assert_eq!(modes[1], 1);
    }

    #[test]
    fn test_day5_part1_opcode1002() {
        assert_eq!(day2(&vec!(1002,4,3,4,33)),1002);
    }

    #[test]
    fn test_day5_part1_opcode_negative() {
        assert_eq!(day2(&vec!(1101,100,-1,4,0)),1101);
    }

    #[test]
    fn test_day5_part1_assignment() {
        let f = File::open("input5.txt").unwrap();
        let file = BufReader::new(&f);
        let mut memory: Vec<_> = file.lines().next().unwrap().unwrap().split(",").map(|s| s.parse().unwrap()).collect();

        let mut outputs = &mut Vec::new();
        day5(&memory, &vec!(1), outputs);
        assert_eq!(*outputs.last().unwrap(), 16225258);
    }
}
