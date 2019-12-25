use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::sync::mpsc::{Receiver, Sender};


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
    Relative(i64),
    Absolute(usize),
    Halt,
}

#[derive(std::fmt::Debug)]
enum Parameter {
    Absolute(usize),
    Immediate(i64),
    Relative(i64),
}

#[derive(std::fmt::Debug)]
pub struct Context {
    memory: Vec<i64>,
    input: Receiver<i64>,
    output: Sender<i64>,
    relative_base: usize,
}

fn calc_position_and_resize(context: &mut Context, parameter: &Parameter) -> usize {
    let position = match parameter {
        Parameter::Absolute(position) => *position,
        Parameter::Relative(position) => (context.relative_base as i64 + *position) as usize,
        Parameter::Immediate(_) => 0,
    };
    if position >= context.memory.len() {
        context.memory.resize(position + 1, 0);
    }
    position
}

impl Context {
    pub fn new(memory: Vec<i64>, input: Receiver<i64>, output: Sender<i64>) -> Context {
        Context {
            memory,
            input,
            output,
            relative_base: 0
        }
    }

    pub fn read(&mut self, position: usize) -> i64 {
        self.memory[position]
    }

    pub fn write(&mut self, position: usize, value: i64) {
        self.memory[position] = value;
    }

    fn read_memory_parameter(&mut self, parameter: &Parameter) -> i64 {
        let position= calc_position_and_resize(self, parameter);
        match parameter {
            Parameter::Absolute(_) | Parameter::Relative(_) => self.read(position),
            Parameter::Immediate(value) => *value
        }.clone()
    }

    fn write_memory_parameter(&mut self, parameter: &Parameter, value: i64) {
        let position= calc_position_and_resize(self, parameter);
        match parameter {
            Parameter::Absolute(_) | Parameter::Relative(_) => self.write(position, value),
            Parameter::Immediate(_) => panic!("Write in immediate mode is not possible!")
        }
    }

    fn read_input(&mut self) -> i64 {
        let i = self.input.recv().unwrap();
//        println!("read {}", i);
        i
    }

    fn write_output(&mut self, value: i64) {
//        println!("write {}", value);
        self.output.send(value).unwrap();
    }
}

fn add_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP {
    let value = context.read_memory_parameter(&parameters[0]) + context.read_memory_parameter(&parameters[1]);
    context.write_memory_parameter(&parameters[2], value);
    IP::Relative(4)
}

fn multiply_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
    let value = context.read_memory_parameter(&parameters[0]) * context.read_memory_parameter(&parameters[1]);
    context.write_memory_parameter(&parameters[2], value);
    IP::Relative(4)
}

fn input_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
    let value = context.read_input();
//    println!("input: {}", value);
    context.write_memory_parameter(&parameters[0], value);
    IP::Relative(2)
}

fn output_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
    let value = context.read_memory_parameter(&parameters[0]);
    context.write_output(value);
    IP::Relative(2)
}

fn jump_not_zero_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
    if context.read_memory_parameter(&parameters[0]) != 0 {
        IP::Absolute(context.read_memory_parameter(&parameters[1]) as usize)
    } else {
        IP::Relative(3)
    }
}

fn jump_zero_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
    let value = context.read_memory_parameter(&parameters[0]);
    let position = context.read_memory_parameter(&parameters[1]);
    if value == 0 {
        IP::Absolute(position as usize)
    } else {
        IP::Relative(3)
    }
}

fn less_than_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
    let value = if context.read_memory_parameter(&parameters[0]) < context.read_memory_parameter(&parameters[1]) {
        1
    } else {
        0
    };
    context.write_memory_parameter(&parameters[2], value);
    IP::Relative(4)
}

fn equals_implementation(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
    let value = if context.read_memory_parameter(&parameters[0]) == context.read_memory_parameter(&parameters[1]) {
        1
    } else {
        0
    };
    context.write_memory_parameter(&parameters[2], value);
    IP::Relative(4)
}

fn adjust_relative_base(parameters: Vec<Parameter>, context: &mut Context) -> IP  {
//    println!("arb {:#?} {:#?}", context, parameters);
    context.relative_base = (context.relative_base as i64 + context.read_memory_parameter(&parameters[0])) as usize;
    IP::Relative(2)
}


fn halt_implementation(_parameters: Vec<Parameter>, _context: &mut Context) -> IP  {
    IP::Halt
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
        Instruction { opcode: 9, operand_count: 1, implementation: adjust_relative_base },
        Instruction { opcode: 99, operand_count: 0, implementation: halt_implementation }
    );
    let mut result = HashMap::new();
    for i in instructions {
        result.insert(i.opcode, i);
    }
    result
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

pub fn run(context: &mut Context) {
    let instructions = init_instruction_definitions();
    let mut offset: usize = 0;

    loop {
        let (instruction, parameters) = parse_instruction(&instructions, context, offset);

        match (instruction.implementation)(parameters, context) {
            IP::Relative(offset_change) => {
                offset = (offset as i64 + offset_change) as usize;
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

fn parse_instruction<'a>(instructions: &'a HashMap<usize, Instruction>, context: &'a mut Context, offset: usize) -> (&'a Instruction, Vec<Parameter>) {
    let opcode = context.memory[offset] as usize;
    let (opcode, modes) = split_instruction(opcode);
    let instruction = &instructions[&opcode];
    let param_values:Vec<i64> = ((offset + 1)..(offset + 1 + instruction.operand_count)).map(|i| context.memory[i]).collect();

    let parameters = param_values
        .iter()
        .enumerate()
        .map(|(i, param)|
            match modes.get(i) {
                Some(1) => Parameter::Immediate(*param),
                Some(2) => Parameter::Relative(*param),
                _ => Parameter::Absolute(*param as usize) }
        ).collect();
    //println!("{:#?} {:#?}", instruction, parameters);
    (instruction, parameters)
}

pub fn read_program_from_file(path: &str) -> Vec<i64> {
    let f = File::open(path).unwrap();
    let file = BufReader::new(&f);
    file.lines().next().unwrap().unwrap().split(",").map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_part1_split_opcode() {
        let (opcode, modes) = split_instruction(1002);
        assert_eq!(opcode, 2);
        assert_eq!(modes.len(), 2);
        assert_eq!(modes[0], 0);
        assert_eq!(modes[1], 1);
    }



}
