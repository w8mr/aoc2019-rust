use std::collections::HashMap;
use std::cmp::Ordering::{Greater, Less};
use std::cmp::{Ordering, min};

#[derive(std::fmt::Debug)]
struct Reaction {
    input:Vec<ReactionPart>,
    output:Vec<ReactionPart>,
}

impl Reaction {
    fn new(input: Vec<ReactionPart>, output: Vec<ReactionPart>) -> Reaction {
        Reaction { input, output }
    }

    fn needs(&self, reaction: &Reaction) -> bool {
        self.input.iter().any(|rp| reaction.output.iter().any(|rp1| rp1.element == rp.element))
    }
}


#[derive(std::fmt::Debug)]
struct ReactionPart {
    count:u64,
    element:String
}

impl ReactionPart {
    fn new(count: u64, element: &str) -> ReactionPart {
        ReactionPart { count, element: element.to_string() }
    }
}

/*#[derive(std::fmt::Debug, std::hash::Hash, std::cmp::Eq)]
struct Element {
    name: String
}
*/
fn parse_reaction_part(reaction_part: &str) -> ReactionPart {
    let mut parts = reaction_part.split(" ");
    let count = parts.next().unwrap().parse().unwrap();
    let element = parts.next().unwrap();
    ReactionPart::new(count, element)
}

fn parse_reaction(reaction: &str) -> Reaction {
    let mut parts = reaction.split(" => ");
    let mut sub_parts = parts.next().unwrap().split(", ");
    let input: Vec<ReactionPart> = sub_parts.map(|sub_part| parse_reaction_part(sub_part)).collect();
    let output = vec!(parse_reaction_part(parts.next().unwrap()));
    Reaction::new(input, output)
}
fn parse_reactions(rections: &Vec<&str>) -> HashMap<String, Reaction> {
    rections.iter().map(|reaction| parse_reaction(reaction)).map(|reaction|(reaction.output[0].element.clone(), reaction)).collect()
}

fn count(reactions: &HashMap<String, Reaction>, needed: &str, needed_for: &str, needed_count: u64, excess: &mut HashMap<String, u64>) -> u64 {
    println!("needed_for: {}, needed_count: {}", needed_for, needed_count);
    let reaction = reactions.get(&needed_for.to_string()).unwrap();
    reaction.input.iter().map(|reaction_part| {
        let (reaction_count, overflow) = div_round_up_overflow(needed_count, reaction.output[0].count);
        println!("reaction_part.element: {}, reaction_part.count: {}, reaction_count {}, overflow {}", reaction_part.element, reaction_part.count, reaction_count, overflow);
        if reaction_part.element == needed.to_string() {
            if overflow > 0 {
                let cuurent_excess = excess.remove(needed_for).unwrap_or(0);

                excess.insert(needed_for.to_string(), overflow+ cuurent_excess);
                println!("needed {}, current_excess {}, overflow {}", needed, cuurent_excess, overflow);
            }
            reaction_part.count * reaction_count
        } else {
            let excess_for_input = excess.remove(reaction_part.element.as_str()).unwrap_or(0);
            let needed_for_input = reaction_part.count * reaction_count;
            if needed_for_input >= excess_for_input {
                println!("reaction: excess_for_input {}, needed_for_input {}",excess_for_input, needed_for_input);
                count(reactions, needed, reaction_part.element.as_str(), needed_for_input-excess_for_input, excess)
            } else {
                println!("excess: excess_for_input {}, needed_for_input {}",excess_for_input, needed_for_input);
                excess.insert(reaction_part.element.clone(), excess_for_input - needed_for_input);
                0
            }
        }
    }).sum()
}

fn div_round_up_overflow(a: u64, b: u64) -> (u64, u64) {
    let m = a % b;
    if m == 0 {
        (a / b, 0)
    } else {
        (a / b + 1, b - m)
    }
}

fn check_needed_inputs(possible_reactions: &HashMap<String, &Reaction>, reaction: &Reaction) -> Option<Reaction> {
    println!("reaction: {:#?}", reaction);
    let x:Vec<Option<(ReactionPart, ReactionPart)>> = reaction.input.iter().map(|rp| match possible_reactions.get(&rp.element) {
        Some(reaction_inner) => {
            let c = div_round_up_overflow(rp.count, reaction_inner.output[0].count);
            Some((
                ReactionPart::new(c.0*reaction_inner.output[0].count, reaction_inner.output[0].element.as_str()),
                ReactionPart::new(1, reaction.output[0].element.as_str())))
        },
        None => None
    }).collect();
    println!("check_needed_inputs {:#?}", x);
    None
}

fn can_produce(possible_reactions: &mut Vec<(Reaction, Reaction)>, reactions: &mut HashMap<String, Reaction>) {
    let possible_elements: HashMap<String, &Reaction> = possible_reactions.iter().map(|r| (r.0.output[0].element.clone(), &r.1)).collect();
    println!("possible_elements: {:#?}", possible_elements);
    let x:Vec<Option<Reaction>> = reactions.values().map(|r| check_needed_inputs(&possible_elements, r)).collect();
    println!("{:#?}", x);

}

pub fn part2(reactions: &Vec<&str>) -> u64 {
    let existing_elements = 1000000000000u64;
    let mut guess= 1;
    let mut previous = 3;
    let mut iteration = 0;
    while (guess as i64-previous as i64).abs() > 1 && iteration < 50 {
        let mut answer = method3(reactions,"ORE", "FUEL", guess);
        previous = guess;
        guess = (existing_elements as u128 * guess as u128 / answer as u128) as u64;
        println!("Guess {}, previous {}, answer {}", guess, previous, answer);
        iteration += 1;
    }
    min(guess, previous)
}


pub fn part1_try3(reactions: &Vec<&str>) -> u64 {
    method3(reactions, "ORE", "FUEL", 1)
}

fn method3(reactions: &Vec<&str>, existing: &str, needed: &str, count_needed: u64) -> u64 {
    let mut reactions = parse_reactions(&reactions);
    let mut vec: Vec<&Reaction> = reactions.values().collect();
    let sorted_reactions = order_by_possible_reactions(&mut vec, existing);
//    println!("try3 {:#?}", sorted_reactions);
    let mut count: HashMap<&str, u64> = HashMap::new();
    count.insert(needed, count_needed);
    sorted_reactions.iter().for_each(|reaction| {
        let reaction_count = div_round_up_overflow(count.get(reaction.output[0].element.as_str()).unwrap_or(&0u64).clone(), reaction.output[0].count).0;
        reaction.input.iter().for_each(|rp| {
            *count.entry(rp.element.as_str()).or_insert(0) += reaction_count * rp.count;
        });
//        println!("Reaction {} count {:#?}", reaction.output[0].element, count);
    });
    count.get(existing).unwrap_or(&0u64).clone()
}

fn order_by_possible_reactions<'a>(vec: &mut Vec<&'a Reaction>, possible_element: &str) -> Vec<&'a Reaction> {
    let mut elements_possible = vec!(possible_element);
    let mut sorted_reactions: Vec<&Reaction> = Vec::new();
    while vec.len() > 0 {
        let mut index = 0;
        while index != vec.len() {
            if vec[index].input.iter().all(|rp|
                elements_possible.contains(&rp.element.as_str())) {
                let reaction = vec.remove(index);
                sorted_reactions.push(reaction);
                elements_possible.push(reaction.output[0].element.as_str())
            } else {
                index += 1;
            }
        }
    }
    sorted_reactions.reverse();
    sorted_reactions
}

pub fn part1_try2(reactions: &Vec<&str>) -> u64 {
    let mut reactions = parse_reactions(reactions);

    let mut posible_reactions = vec!((Reaction::new(
        vec!(ReactionPart::new(1, "ORE")),
        vec!(ReactionPart::new(1, "ORE"))
    ), Reaction::new(
        vec!(ReactionPart::new(1, "ORE")),
        vec!(ReactionPart::new(1, "ORE"))
    )));
    can_produce(&mut posible_reactions, &mut reactions);
    0
}

pub fn part1(reactions: &Vec<&str>) -> u64 {
    let reactions = parse_reactions(reactions);
    //println!("{:#?}", reactions) ;
    count(&reactions, "ORE", "FUEL", 1, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_try3_example1() {
        assert_eq!(part1_try3(&example1()), 31);
    }

    #[test]
    fn part1_try3_example2() {
        assert_eq!(part1_try3(&example2()), 165);
    }

    #[test]
    fn part1_try3_example3() {
        assert_eq!(part1_try3(&example3()), 13312);
    }

    #[test]
    fn part1_try3_example4() {
        assert_eq!(part1_try3(&example4()), 180697);
    }

    #[test]
    fn part1_try3_example5() {
        assert_eq!(part1_try3(&example5()), 2210736);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2(&example3()), 82892753);
    }

    #[test]
    fn part2_example4() {
        assert_eq!(part2(&example4()), 5586022);
    }

    #[test]
    fn part2_example5() {
        assert_eq!(part2(&example5()), 460664);
    }

    #[test]
    fn part1_try2_example1() {
        assert_eq!(part1_try2(&example1()), 31);
    }

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&example1()), 31);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&example2()), 165);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1(&example3()), 13312);
    }

//    #[test]
    fn part1_example4() {
        assert_eq!(part1(&example4()), 180697);
    }

//    #[test]
    fn part1_example5() {
        assert_eq!(part1(&example5()), 2210736);
    }


    #[test]
    fn div_round_up_overflow_overflow() {
        assert_eq!(div_round_up_overflow(100,80), (2,60));
    }

    #[test]
    fn div_round_up_overflow_no_overflow() {
        assert_eq!(div_round_up_overflow(160,80), (2,0));
    }

    #[test]
    fn count_non_recursive_without_excess() {
        let reactions = parse_reactions(&example2());
        let mut excess:HashMap<String, u64> = HashMap::new();
        assert_eq!(count(&reactions, "ORE", "A", 2, &mut excess), 9);
        assert_eq!(*excess.get("A").unwrap_or(&0),0);
    }


    #[test]
    fn count_non_recursive_with_excess() {
        let reactions = parse_reactions(&example2());
        let mut excess:HashMap<String, u64> = HashMap::new();
        assert_eq!(count(&reactions, "ORE", "A", 1, &mut excess), 9);
        assert_eq!(*excess.get("A").unwrap_or(&0),1);
    }
    #[test]
    fn count_recursive_with_excess() {
        let reactions = parse_reactions(&example2());
        let mut excess:HashMap<String, u64> = HashMap::new();
        assert_eq!(count(&reactions, "ORE", "AB", 1, &mut excess), 34);
        assert_eq!(*excess.get("A").unwrap_or(&0),1);
        assert_eq!(*excess.get("B").unwrap_or(&0),2);
        assert_eq!(*excess.get("ORE").unwrap_or(&0),0);
    }

    fn example1() -> Vec<&'static str> {
        vec!(
            "10 ORE => 10 A",
            "1 ORE => 1 B",
            "7 A, 1 B => 1 C",
            "7 A, 1 C => 1 D",
            "7 A, 1 D => 1 E",
            "7 A, 1 E => 1 FUEL",
        )
    }

    fn example2() -> Vec<&'static str> {
        vec!(
            "9 ORE => 2 A",
            "8 ORE => 3 B",
            "7 ORE => 5 C",
            "3 A, 4 B => 1 AB",
            "5 B, 7 C => 1 BC",
            "4 C, 1 A => 1 CA",
            "2 AB, 3 BC, 4 CA => 1 FUEL",
        )
    }

    fn example3() -> Vec<&'static str> {
        vec!(
            "157 ORE => 5 NZVS",
            "165 ORE => 6 DCFZ",
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
            "179 ORE => 7 PSHF",
            "177 ORE => 5 HKGWZ",
            "7 DCFZ, 7 PSHF => 2 XJWVT",
            "165 ORE => 2 GPVTF",
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        )
    }

    fn example4() -> Vec<&'static str> {
        vec!(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
            "17 NVRVD, 3 JNWZP => 8 VPVL",
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
            "22 VJHF, 37 MNCFX => 5 FWMGM",
            "139 ORE => 4 NVRVD",
            "144 ORE => 7 JNWZP",
            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
            "145 ORE => 6 MNCFX",
            "1 NVRVD => 8 CXFTF",
            "1 VJHF, 6 MNCFX => 4 RFSQX",
            "176 ORE => 6 VJHF",
        )
    }

    fn example5() -> Vec<&'static str> {
        vec!(
            "171 ORE => 8 CNZTR",
            "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
            "114 ORE => 4 BHXH",
            "14 VRPVC => 6 BMBT",
            "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
            "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
            "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
            "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
            "5 BMBT => 4 WPTQ",
            "189 ORE => 9 KTJDG",
            "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
            "12 VRPVC, 27 CNZTR => 2 XDBXC",
            "15 KTJDG, 12 BHXH => 5 XCVML",
            "3 BHXH, 2 VRPVC => 7 MZWV",
            "121 ORE => 7 VRPVC",
            "7 XCVML => 6 RJRHP",
            "5 BHXH, 4 VRPVC => 5 LTCX",
        )
    }


}



