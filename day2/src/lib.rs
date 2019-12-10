use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

struct Instruction {
    opcode: usize,
    operand_count: usize,
    implementation: fn(Vec<Parameter>, &mut Context) -> IP,
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{{ opcode: {:#?}, operand_count: {} }}", self.opcode, self.operand_count)
    }
}

#[derive(std::fmt::Debug)]
enum IP {
    Relative(isize),
    Absolute(usize),
    Halt,
}

#[derive(std::fmt::Debug)]
enum Parameter {
    Absolute(usize),
    Immediate(isize),
}

#[derive(std::fmt::Debug)]
struct Context {
    memory: Vec<isize>,
    inputs: Vec<isize>,
    outputs: Vec<isize>,
}

impl Context {
    fn read(& self, parameter: &Parameter) -> isize {
        match parameter {
            Parameter::Absolute(position) => self.memory[*position],
            Parameter::Immediate(value) => *value
        }.clone()
    }

    fn write(&mut self, parameter: &Parameter, value: isize) {
        match parameter {
            Parameter::Absolute(position) => self.memory[*position] = value,
            Parameter::Immediate(_) => panic!("Write in immediate mode is not possible!")
        }
    }

    fn read_input(&mut self) -> isize {
        self.inputs.pop().unwrap()
    }

    fn write_output(&mut self, value: isize) {
        self.outputs.push(value);
    }
}

fn add_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP {
    context.write(&parameters[2], context.read(&parameters[0]) + context.read(&parameters[1]));
    IP::Relative(4)
}

fn multiply_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
    context.write(&parameters[2], context.read(&parameters[0]) * context.read(&parameters[1]));
    IP::Relative(4)
}

fn input_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
    let value = context.read_input();
//    println!("input: {}", value);
    context.write(&parameters[0], value);
    IP::Relative(2)
}

fn output_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
    context.write_output(context.read(&parameters[0]));
    IP::Relative(2)
}

fn jump_not_zero_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
    if context.read(&parameters[0]) != 0 {
        IP::Absolute(context.read(&parameters[1]) as usize)
    } else {
        IP::Relative(3)
    }
}

fn jump_zero_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
    let value = context.read(&parameters[0]);
    let position = context.read(&parameters[1]);
    if value == 0 {
        IP::Absolute(position as usize)
    } else {
        IP::Relative(3)
    }
}

fn less_than_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
    let value = if context.read(&parameters[0]) < context.read(&parameters[1]) {
        1
    } else {
        0
    };
    context.write(&parameters[2], value);
    IP::Relative(4)
}

fn equals_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
    let value = if context.read(&parameters[0]) == context.read(&parameters[1]) {
        1
    } else {
        0
    };
    context.write(&parameters[2], value);
    IP::Relative(4)
}


fn halt_implementation(_parameters: Vec<Parameter>, _context: &mut Context) -> IP  {
    IP::Halt
}

fn split_instruction(opcode: usize) -> (usize, Vec<usize>) {
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
    let mut context = Context { memory: opcodes.to_vec(), inputs: vec!(), outputs: vec!() };
    run(&mut context);
    context.memory[0]
}

pub fn day5(opcodes: &Vec<isize>, inputs: &Vec<isize>) -> Vec<isize> {
    let outputs = Vec::new();
    let mut inputs = inputs.clone();
    inputs.reverse();
    let mut context = Context { memory: opcodes.to_vec(), inputs, outputs };
    run(&mut context);
    context.outputs
}

pub fn recurse(input: Vec<usize>) -> Vec<Vec<usize>> {
    if input.len() == 1 {
        vec!(input)
    } else {
        (0..input.len()).flat_map(|index| -> Vec<Vec<usize>> {
            let mut rec_input = input.clone();
            let item = rec_input.remove(index);
            let recurse = recurse(rec_input);
            recurse.iter().map(|vec| {
                let mut v = vec.clone();
                v.push(item);
                v
            }).collect()
        }).collect()
    }
}

pub fn day7(opcodes: &Vec<isize>) -> (Vec<usize>, isize) {
    recurse((0..5).collect())
        .iter()
        .fold(None, |acc: Option<(Vec<usize>, isize)>, phases| {
            let result = day7_internal(opcodes, &phases);
            match acc {
                Some((p, high)) if high >= result => Some((p, high)),
                _ => Some((phases.clone(), result)),
            }
        }).unwrap()
}

fn day7_internal(opcodes: &Vec<isize>, phases:&Vec<usize>) -> isize {
    let mut input = 0;
    for phase in phases {
        //println!("{}: {}",phase, input);
        let output = day5(opcodes, &vec!(*phase as isize, input));
        input = *output.last().unwrap();
    }
    input
}

fn run(context: &mut Context) {
    let instructions = init_instruction_definitions();
    let mut offset: usize = 0;

    loop {
        let (instruction, parameters) = parse_instruction(&instructions, context, offset);

        match (instruction.implementation)(parameters, context) {
            IP::Relative(offset_change) => {
                offset = (offset as isize + offset_change) as usize;
            }
            IP::Absolute(position) => {
                offset = position;
            }
            IP::Halt => {
                break
            }
        }
    }
  //  println!("{:#?}", context);
}

fn init_instruction_definitions() -> HashMap<usize, Instruction> {
    let instructions = vec!(
        Instruction { opcode: 1, operand_count: 3, implementation: add_implementation },
        Instruction { opcode: 2, operand_count: 3, implementation: multiply_implementation },
        Instruction { opcode: 3, operand_count: 1, implementation: input_implementation },
        Instruction { opcode: 4, operand_count: 1, implementation: output_implementation },
        Instruction { opcode: 5, operand_count: 2, implementation: jump_not_zero_implementation },
        Instruction { opcode: 6, operand_count: 2, implementation: jump_zero_implementation },
        Instruction { opcode: 7, operand_count: 3, implementation: less_than_implementation },
        Instruction { opcode: 8, operand_count: 3, implementation: equals_implementation },
        Instruction { opcode: 99, operand_count: 0, implementation: halt_implementation }
    );
    let mut result = HashMap::new();
    for i in instructions {
        result.insert(i.opcode, i);
    }
    result

//    instructions.iter().map(|&i| (i.opcode, i)).collect()
}

fn parse_instruction<'a>(instructions: &'a HashMap<usize, Instruction>, context: &'a mut Context, offset: usize) -> (&'a Instruction, Vec<Parameter>) {
    let opcode = context.memory[offset] as usize;
    let (opcode, modes) = split_instruction(opcode);
    let instruction = &instructions[&opcode];
    let param_values:Vec<isize> = ((offset + 1)..(offset + 1 + instruction.operand_count)).map(|i| context.memory[i]).collect();

    let parameters = param_values
        .iter()
        .enumerate()
        .map(|(i, param)|
            match modes.get(i) {
                Some(1) => Parameter::Immediate(*param),
                _ => Parameter::Absolute(*param as usize) }
        ).collect();
    //println!("{:#?} {:#?}", instruction, parameters);
    (instruction, parameters)
}

pub fn read_program_from_file(path: &str) -> Vec<isize> {
    let f = File::open(path).unwrap();
    let file = BufReader::new(&f);
    file.lines().next().unwrap().unwrap().split(",").map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
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
        let mut memory = read_program_from_file("input2.txt");
        memory[1]=12;
        memory[2]=02;
        day2(&memory);
        assert_eq!(day2(&memory), 4138687);
    }

    #[test]
    fn test_day2_part2_assignment() {
        let mut memory = read_program_from_file("input2.txt");
        memory[1]=66;
        memory[2]=35;
        day2(&memory);
        assert_eq!(day2(&memory), 19690720);
    }

    #[test]
    fn test_day5_part1_opcode030499() {
        let outputs = day5(&vec!(3, 0, 4, 0, 99), &vec!(42));
        assert_eq!(outputs.len(),1);
        assert_eq!(outputs[0],42);
    }

    #[test]
    fn test_day5_part1_split_opcode() {
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
        let memory = read_program_from_file("input5.txt");

        let outputs = day5(&memory, &vec!(1));
        assert_eq!(*outputs.last().unwrap(), 16225258);
    }

    #[test]
    fn test_day5_part2_assignment() {
        let memory = read_program_from_file("input5.txt");

        let outputs = day5(&memory, &vec!(5));
        assert_eq!(*outputs.last().unwrap(), 2808771);
    }

    #[test]
    fn test_day5_part2_example1_not_equal() {
        let outputs = day5(&vec!(3,9,8,9,10,9,4,9,99,-1,8), &vec!(5));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_day5_part2_example1_equal() {
        let outputs = day5(&vec!(3,9,8,9,10,9,4,9,99,-1,8), &vec!(8));
        assert_eq!(outputs, vec!(1));
    }

    #[test]
    fn test_day5_part2_example4_less_than() {
        let outputs = day5(&vec!(3,3,1107,-1,8,3,4,3,99), &vec!(5));
        assert_eq!(outputs, vec!(1));
    }

    #[test]
    fn test_day5_part2_example4_not_less_than() {
        let outputs = day5(&vec!(3,3,1107,-1,8,3,4,3,99), &vec!(8));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_day5_part2_jmp_example1_jmp() {
        let outputs = day5(&vec!(3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9), &vec!(0));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_day5_part2_large_example4_less_than() {
        let outputs = day5(&vec!(
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99), &vec!(7));
        assert_eq!(outputs, vec!(999));
    }

    #[test]
    fn test_day5_part2_large_example4_equals() {
        let outputs = day5(&vec!(
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99), &vec!(8));
        assert_eq!(outputs, vec!(1000));
    }

    #[test]
    fn test_day5_part2_large_example4_not_less_than() {
        let outputs = day5(&vec!(
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99), &vec!(9));
        assert_eq!(outputs, vec!(1001));
    }

    #[test]
    fn test_recursive1() {
        let rec = recurse(vec!(1));
        assert_eq!(rec.len(), 1);
        assert!((rec.contains(&vec!(1))));
    }

    #[test]
    fn test_recursive2() {
        let rec = recurse(vec!(1,2));
        assert_eq!(rec.len(), 2);
        assert!((rec.contains(&vec!(1,2))));
        assert!((rec.contains(&vec!(2,1))));
    }

    #[test]
    fn test_recursive3() {
        let rec = recurse(vec!(1,2,3));
        assert_eq!(rec.len(), 6);
        assert!((rec.contains(&vec!(1,2,3))));
        assert!((rec.contains(&vec!(1,3,2))));
        assert!((rec.contains(&vec!(2,1,3))));
        assert!((rec.contains(&vec!(2,3,1))));
        assert!((rec.contains(&vec!(3,1,2))));
        assert!((rec.contains(&vec!(3,2,1))));
    }

    #[test]
    fn test_day7_part1_example1_internal() {
        assert_eq!(day7_internal(&vec!(3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0), &vec!(4,3,2,1,0)), 43210);
    }

    #[test]
    fn test_day7_part1_example2_full() {
        assert_eq!(day7(&vec!(
            3,23,3,24,1002,24,10,24,1002,23,-1,23,
            101,5,23,23,1,24,23,23,4,23,99,0,0)), (vec!(0,1,2,3,4), 54321));
    }

    #[test]
    fn test_day7_part1_example3() {
        assert_eq!(day7_internal(&vec!(
            3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
            1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0), &vec!(1,0,4,3,2)), 65210);
    }

    #[test]
    fn test_day7_part1_example3_full() {
        assert_eq!(day7(&vec!(
            3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
            1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0)), (vec!(1,0,4,3,2), 65210));
    }
}
