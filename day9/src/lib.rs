#[cfg(test)]
mod tests {
    #[test]
    fn test_day9_part1_example1() {
        assert_eq!(intcode::run_input_output(&vec!(109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99), &vec!()), vec!(109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99));
    }

    #[test]
    fn test_day9_part1_example2() {
        assert_eq!(intcode::run_input_output(&vec!(1102,34915192,34915192,7,4,7,99,0), &vec!()), vec!(1219070632396864));
    }

    #[test]
    fn test_day9_part1_example3() {
        assert_eq!(intcode::run_input_output(&vec!(104,1125899906842624,99), &vec!()), vec!(1125899906842624));
    }

    #[test]
    fn test_day9_part1_assignment() {
        let memory = intcode::read_program_from_file("input9.txt");
        assert_eq!(intcode::run_input_output(&memory, &vec!(1)), vec!(3235019597));
    }

    #[test]
    fn test_day9_part2_assignment() {
        let memory = intcode::read_program_from_file("input9.txt");
        assert_eq!(intcode::run_input_output(&memory, &vec!(2)), vec!(80274));
    }
}
