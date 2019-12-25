use std::sync::{mpsc};
use std::thread;

use intcode::*;

pub fn day2(opcodes: &Vec<i64>) -> i64 {
    let (_, input) = mpsc::channel();
    let (output, _) = mpsc::channel();
    let mut context = intcode::Context::new(opcodes.to_vec(), input, output);
    thread::spawn(move || {
        run(&mut context);
        context.read(0)
    }).join().unwrap_or_default()
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
}
