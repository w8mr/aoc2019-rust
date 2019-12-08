use std::collections::HashMap;

struct Instruction {
    opcode: Opcode,
    operand_count: usize,
    implementation: fn(Vec<usize>, &mut Context) -> Option<i32>,
}

#[derive(std::fmt::Debug, Copy, Clone)]
enum Opcode {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{{ opcode: {:#?}, operand_count: {} }}", self.opcode, self.operand_count)
    }
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

pub fn part1(opcodes: &Vec<usize>) -> usize {
    let instructions = vec!(
        Instruction { opcode: Opcode::Add, operand_count: 3, implementation: add_implementation },
        Instruction { opcode: Opcode::Multiply, operand_count: 3, implementation: multiply_implementation },
        Instruction { opcode: Opcode::Halt, operand_count: 0, implementation: halt_implementation }
    );
    let instructions: HashMap<usize, &Instruction> = instructions.iter().map(|i| (i.opcode as usize, i)).collect();

    let context = &mut Context { memory: opcodes.to_vec() };
    let mut offset:i32 = 0;

    while true {
        let instruction = &instructions[&context.memory[offset as usize]];

        let x = ((offset as usize + 1)..(offset as usize + 1 + instruction.operand_count)).map(|i| context.memory[i]).collect();

        match (instruction.implementation)(x, context) {
            Some(offset_change) => {
                offset += offset_change;
            }
            None => {
                break
            }
        }
    }

    context.memory[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_opcode99() {
        assert_eq!(part1(&vec!(99)), 99);
    }

    #[test]
    fn test_part1_opcode1() {
        assert_eq!(part1(&vec!(1,5,6,0,99,2,3)), 5);
    }

    #[test]
    fn test_part1_opcode2() {
        assert_eq!(part1(&vec!(2,5,6,0,99,2,3)), 6);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&vec!(1,9,10,3,2,3,11,0,99,30,40,50)), 3500);
    }


}
