use std::collections::HashMap;


#[derive(std::fmt::Debug)]
struct Instruction {
    opcode: usize,
    operand_count: usize,
    implementation: fn(Vec<usize>, &mut Context) -> Option<i32>,
}

enum Opcode {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

fn add_implementation(operands: Vec<usize>, context: &mut Context) -> Option<i32> {
    context.memory[operands[2]] =context.memory[operands[0]]+context.memory[operands[1]];
    Some(4)
}

fn multiply_implementation(operands: Vec<usize>, context: &mut Context) -> Option<i32>  {
    //println!("{:#?}", context);
    context.memory[operands[2]] = context.memory[operands[0]]*context.memory[operands[1]];
    Some(4)
}

fn halt_implementation(operands: Vec<usize>, context: &mut Context) -> Option<i32>  {
    None
}

struct Context {
    memory: Vec<usize>
}

pub fn part1(opcodes: Vec<usize>) -> usize {
    let instructions = vec!(
        Instruction { opcode: 1, operand_count: 3, implementation: add_implementation },
        Instruction { opcode: 2, operand_count: 3, implementation: multiply_implementation },
        Instruction { opcode: 99, operand_count: 0, implementation: halt_implementation }
    );
    let instructions: HashMap<usize, &Instruction> = instructions.iter().map(|i| (i.opcode, i)).collect();

    let context = &mut Context { memory: opcodes };
    let offset = 0;
    let instruction = &instructions[&context.memory[offset]];

    let x = ((offset + 1)..(offset + instruction.operand_count)).map(|i| context.memory[i]).collect();

    (instruction.implementation)(x, context);

    context.memory[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_opcode99() {
        assert_eq!(part1(vec!(99)), 99);
    }

    #[test]
    fn test_part1_opcode1() {
        assert_eq!(part1(vec!(1,2,3,0)), 5);
    }

    #[test]
    fn test_part1_opcode2() {
        assert_eq!(part1(vec!(2,2,3,0)), 6);
    }
}
