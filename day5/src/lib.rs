use std::sync::{mpsc};
use std::thread;

pub fn run_simulation(opcodes: &Vec<i64>, inputs: &Vec<i64>) -> Vec<i64> {
    let (input_send, input) = mpsc::channel();
    let (output, output_recieve) = mpsc::channel();

    inputs.iter().for_each(|&i| {input_send.send(i).unwrap();});

    let mut context = intcode::Context::new(opcodes.to_vec(), input, output);
    thread::spawn(move || {
        intcode::run(&mut context);
    });

    output_recieve.iter().collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use intcode;

    #[test]
    fn test_day5_part1_opcode030499() {
        let outputs = run_simulation(&vec!(3, 0, 4, 0, 99), &vec!(42));
        assert_eq!(outputs.len(),1);
        assert_eq!(outputs[0],42);
    }

    #[test]
    fn test_day5_part1_assignment() {
        let memory = intcode::read_program_from_file("input5.txt");

        let outputs = run_simulation(&memory, &vec!(1));
        assert_eq!(*outputs.last().unwrap(), 16225258);
    }

    #[test]
    fn test_day5_part2_assignment() {
        let memory = intcode::read_program_from_file("input5.txt");

        let outputs = run_simulation(&memory, &vec!(5));
        assert_eq!(*outputs.last().unwrap(), 2808771);
    }

    #[test]
    fn test_day5_part2_example1_not_equal() {
        let outputs = run_simulation(&vec!(3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8), &vec!(5));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_day5_part2_example1_equal() {
        let outputs = run_simulation(&vec!(3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8), &vec!(8));
        assert_eq!(outputs, vec!(1));
    }

    #[test]
    fn test_day5_part2_example4_less_than() {
        let outputs = run_simulation(&vec!(3, 3, 1107, -1, 8, 3, 4, 3, 99), &vec!(5));
        assert_eq!(outputs, vec!(1));
    }

    #[test]
    fn test_day5_part2_example4_not_less_than() {
        let outputs = run_simulation(&vec!(3, 3, 1107, -1, 8, 3, 4, 3, 99), &vec!(8));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_day5_part2_jmp_example1_jmp() {
        let outputs = run_simulation(&vec!(3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9), &vec!(0));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_day5_part2_large_example4_less_than() {
        let outputs = run_simulation(&vec!(
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99), &vec!(7));
        assert_eq!(outputs, vec!(999));
    }

    #[test]
    fn test_day5_part2_large_example4_equals() {
        let outputs = run_simulation(&vec!(
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99), &vec!(8));
        assert_eq!(outputs, vec!(1000));
    }

    #[test]
    fn test_day5_part2_large_example4_not_less_than() {
        let outputs = run_simulation(&vec!(
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99), &vec!(9));
        assert_eq!(outputs, vec!(1001));
    }
}
