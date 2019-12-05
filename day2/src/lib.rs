pub fn part1(opcodes: &Vec<i32>) -> i32 {
    *opcodes.get(0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(&vec!(99)), 99);
    }
}
