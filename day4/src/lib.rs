use std::collections::HashMap;

struct Digits {
    n: usize,
    divisor: usize,
}

impl Digits {
    fn new(n: usize) -> Self {
        let mut divisor = 10;
        while n >= divisor {
            divisor *= 10;
        }

        Digits { n: n, divisor: divisor }
    }
}

impl Iterator for Digits {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n / self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10;
            v
        }

    }
}

fn matches_part1(number: usize) -> bool {
    is_increasing(number) && has_double(number)
}

fn matches_part2(number: usize) -> bool {
    is_increasing(number) && has_double_which_is_not_triple(number)
}


fn is_increasing(number: usize) -> bool {
    Digits::new(number)
        .fold((true, None),
              |(result, previous), digit: usize|
                  (result && if let Some(previous) = previous { previous <= digit } else { true}, Some(digit))
        ).0
}

fn has_double(number: usize) -> bool {
    frequency(&mut Digits::new(number)).iter().filter(|(key, value)| **value >= 2).count() > 0
}

fn has_double_which_is_not_triple(number: usize) -> bool {
    frequency(&mut Digits::new(number)).iter().filter(|(key, value)| **value == 2).count() > 0
}

fn frequency(iter: &mut dyn Iterator<Item=usize>) -> HashMap<usize, u32>{
    let mut frequency: HashMap<usize, u32> = HashMap::new();
    for item in iter.into_iter() {
        *frequency.entry(item).or_insert(0) += 1;
    }
    frequency
}

pub fn part1(low: usize, high: usize) -> usize {
//    (low..high).filter(matches_part1).for_each(|n| println!("{}",n));
    (low..high).filter(|n| matches_part1(*n)).count()
}

pub fn part2(low: usize, high: usize) -> usize {
//    (low..high).filter(matches_part2).for_each(|n| println!("{}",n));
    (low..high).filter(|n| matches_part2(*n)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_part1() {
        assert_eq!(matches_part1(111111), true);
    }

    #[test]
    fn test_no_match_part1_not_increasing() {
        assert_eq!(matches_part1(223450), false);
    }

    #[test]
    fn test_no_match_part1_no_double() {
        assert_eq!(matches_part1(123789), false);
    }

    #[test]
    fn test_no_match_part1_start_decreasing() {
        assert_eq!(matches_part1(737999), false);
    }

    #[test]
    fn test_match_part2() {
        assert_eq!(matches_part2(112233), true);
    }

    #[test]
    fn test_no_match_part2_triple() {
        assert_eq!(matches_part2(123444), false);
    }

    #[test]
    fn test_no_match_part2_triple_start() {
        assert_eq!(matches_part2(111234), false);
    }

    #[test]
    fn test_match_part2_triple_double() {
        assert_eq!(matches_part2(111233), true);
    }

    #[test]
    fn test_match_part2_quadruple_but_also_double() {
        assert_eq!(matches_part2(111122), true);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(1331, 1345), 8);
        //1333, 1334, 1335, 1336, 1337, 1338, 1339, 1344
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(1331, 1345), 7);
        //1334, 1335, 1336, 1337, 1338, 1339, 1344
    }

    #[test]
    fn test_no_match_1333() {
        assert_eq!(matches_part2(1333), false);
        //TODO: Fix length thing
    }

    #[test]
    fn test_assignment_part1() {
        assert_eq!(part1(246515, 739105), 1048);
    }

    #[test]
    fn test_assignment_part2() {
        assert_eq!(part2(246515, 739105), 677);
    }
}
