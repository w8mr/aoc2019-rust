fn matches_part1(number: &i32) -> bool {
    is_increasing(number) && has_double(number)
}

fn matches_part2(number: &i32) -> bool {
    is_increasing(number) && has_double_which_is_not_triple(number)
}

fn is_increasing(number: &i32) -> bool {
    let mut remainder = *number;
    let mut result = true;
    let mut previous: Option<i32> = None;
    for _i in 0..6 {
        let digit = remainder % 10;
        remainder = remainder / 10;
        let new = match previous {
            Some(prev) if prev < digit => false,
            _ => true
        };
        result = result && new;
//        println!("{} {} {:#?} {} {}", digit, remainder, previous, result, new);
        previous = Some(digit);
    }
    return result;
}

fn has_double(number: &i32) -> bool {
    count_doubles(number).iter().find(|n| **n >= 2).is_some()
}

fn has_double_which_is_not_triple(number: &i32) -> bool {
    count_doubles(number).iter().find(|n| **n == 2).is_some()
}

fn count_doubles(number: &i32) -> Vec<i32> {
    let mut remainder = *number;
    let mut result = Vec::new();
    let mut previous: Option<i32> = None;
    let mut double_count = 1;
    for _i in 0..6 {
        let digit = remainder % 10;
        remainder = remainder / 10;
        match previous {
            Some(prev) if prev == digit => {
                double_count = double_count + 1;
            }
            Some(_) => {
                result.push(double_count);
                double_count = 1
            }
            None => {
                double_count = 1
            }
        };
        previous = Some(digit);
    }
    result.push(double_count);
    println!("{:#?}", result);
    result
}


pub fn part1(low: i32, high: i32) -> usize {
//    (low..high).filter(matches_part1).for_each(|n| println!("{}",n));
    (low..high).filter(matches_part1).count()
}

pub fn part2(low: i32, high: i32) -> usize {
//    (low..high).filter(matches_part2).for_each(|n| println!("{}",n));
    (low..high).filter(matches_part2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_part1() {
        assert_eq!(matches_part1(&111111), true);
    }

    #[test]
    fn test_no_match_part1_not_increasing() {
        assert_eq!(matches_part1(&223450), false);
    }

    #[test]
    fn test_no_match_part1_no_double() {
        assert_eq!(matches_part1(&123789), false);
    }

    #[test]
    fn test_no_match_part1_start_decreasing() {
        assert_eq!(matches_part1(&737999), false);
    }

    #[test]
    fn test_match_part2() {
        assert_eq!(matches_part2(&112233), true);
    }

    #[test]
    fn test_no_match_part2_triple() {
        assert_eq!(matches_part2(&123444), false);
    }

    #[test]
    fn test_no_match_part2_triple_start() {
        assert_eq!(matches_part2(&111234), false);
    }

    #[test]
    fn test_match_part2_triple_double() {
        assert_eq!(matches_part2(&111233), true);
    }

    #[test]
    fn test_match_part2_quadruple_but_also_double() {
        assert_eq!(matches_part2(&111122), true);
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
        assert_eq!(matches_part2(&1333), false);
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
