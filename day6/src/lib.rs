use std::collections::HashMap;

pub fn part1(orbits: &Vec<String>) -> i32{
    calc_hash_sum_total(calc_hash_sum_for_object(read_orbits(orbits)))
}

pub fn part2(orbits: &Vec<String>) -> i32 {
    distance(read_orbits(orbits), "YOU".to_string(), "SAN".to_string())
}

fn read_orbits(orbits: &Vec<String>) -> HashMap<String, String> {
    orbits.iter().map(|orbit| { let parts:Vec<&str> = orbit.split(")").collect(); (parts[1].to_string(), parts[0].to_string()) }).collect()
}

fn calc_hash_sum_for_object_map(orbits: &HashMap<String, String>) -> HashMap<String, i32> {
    let mut counts: HashMap<String, i32> = HashMap::new();
    counts.insert("COM".to_string(),0);
    for orbit in orbits {
        calc_hash_sum_for_object_internal(&mut counts, &orbits, orbit);
    }
    counts
}


fn calc_hash_sum_for_object(orbits: HashMap<String, String>) -> Vec<(String, i32)> {
    let counts: &mut HashMap<String, i32> = &mut HashMap::new();
    counts.insert("COM".to_string(),0);
    orbits.iter().map(|orbit| (orbit.0.clone(), calc_hash_sum_for_object_internal(counts, &orbits, orbit))).collect()
}

fn calc_hash_sum_for_object_internal(counts: &mut HashMap<String, i32>, orbits: &HashMap<String, String>, orbit: (&String, &String)) -> i32 {
    //println!("{} {}", orbit.0, orbit.1);
    let option = counts.get(orbit.1);
    let count = match option {
        Some(count) => {
            *count
        }
        None => {
            calc_hash_sum_for_object_internal(counts, orbits, (orbit.1, orbits.get(orbit.1).unwrap()))
        }
    } + 1;
    counts.insert(orbit.0.clone().to_string(), count);
    count
}

fn calc_hash_sum_total(counts: Vec<(String, i32)>) -> i32 {
    counts.iter().map(|c| c.1).sum()
}

fn parent_path(orbits: &HashMap<String, String>, object:String) -> Vec<String> {
    if object == "COM" {
        vec!("COM".to_string())
    } else {
        let mut path = parent_path(&orbits, orbits.get(&object).unwrap().to_string());
        path.push(object);
        path
    }
}

fn common_path(path1: &Vec<String>, path2: &Vec<String>) -> Vec<String>{
    path1.iter().zip(path2.iter()).filter(|t| t.0 == t.1).map(|t| t.0.clone()).collect()
}

fn distance(orbits: HashMap<String, String>, object1: String, object2: String) -> i32 {
    let counts = calc_hash_sum_for_object_map(&orbits);
    let path1 = parent_path(&orbits, object1);
    let path2 = parent_path(&orbits, object2);
    let common = common_path(&path1, &path2);
    let o1 = counts.get(path1.last().unwrap()).unwrap();
    let o2 = counts.get(path2.last().unwrap()).unwrap();
    let oc = counts.get(common.last().unwrap()).unwrap();
    o1 + o2 - 2 * oc - 2

}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;

    use super::*;

    #[test]
    fn test_read_orbits() {
        let example = vec!("COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L").iter().map(|s| s.to_string()).collect();
        let orbits = read_orbits(&example);
        assert_eq!(orbits.len(), 11);
        assert_eq!(orbits["C"], "B");
        assert_eq!(orbits["G"], "B");
    }

    #[test]
    fn test_calc_hash_sum_for_object() {
        let example = vec!("COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L").iter().map(|s| s.to_string()).collect();
        let orbits = read_orbits(&example);
        let mut counts = calc_hash_sum_for_object(orbits);
        counts.sort_by(|a,b| a.0.partial_cmp(&b.0).unwrap());
        println!("{:#?}", counts);
        assert_eq!(counts.len(), 11);
        assert_eq!(counts[0].1, 1);
        assert_eq!(counts[10].1, 7);
    }

    #[test]
    fn test_part1() {
        let example = vec!("COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L").iter().map(|s| s.to_string()).collect();
        assert_eq!(part1(&example), 42);
    }

    #[test]
    fn test_part1_assignment() {
        let f = File::open("input6.txt").unwrap();
        let file = BufReader::new(&f);
        let orbits: Vec<_> = file.lines().map(|s| s.unwrap()).collect();
        assert_eq!(part1(&orbits),294191);
    }

    #[test]
    fn test_parent_path() {
        let example = vec!("COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU", "I)SAN").iter().map(|s| s.to_string()).collect();
        let orbits = read_orbits(&example);
        let you_path = parent_path(&orbits, "YOU".to_string());
        let san_path = parent_path(&orbits, "SAN".to_string());
        println!("{:#?}", you_path);
        println!("{:#?}", san_path);
        assert_eq!(you_path[7],"YOU".to_string());
        assert_eq!(you_path[3],"D".to_string());
        assert_eq!(san_path[5],"SAN".to_string());
        assert_eq!(san_path[3],"D".to_string());
    }

    #[test]
    fn test_common_path() {
        let example = vec!("COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU", "I)SAN").iter().map(|s| s.to_string()).collect();
        let orbits = read_orbits(&example);
        let you_path = parent_path(&orbits, "YOU".to_string());
        let san_path = parent_path(&orbits, "SAN".to_string());
        let common_path = common_path(&you_path, &san_path);
        println!("{:#?}", common_path);
        assert_eq!(common_path.len(),4);
        assert_eq!(common_path[3],"D".to_string());
    }

    #[test]
    fn test_distance() {
        let example = vec!("COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU", "I)SAN").iter().map(|s| s.to_string()).collect();
        let orbits = read_orbits(&example);
        assert_eq!(distance(orbits, "YOU".to_string(), "SAN".to_string()),4);
    }

    #[test]
    fn test_part2() {
        let example = vec!("COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU", "I)SAN").iter().map(|s| s.to_string()).collect();
        assert_eq!(part2(&example),4);
    }

    #[test]
    fn test_part2_assignment() {
        let f = File::open("input6.txt").unwrap();
        let file = BufReader::new(&f);
        let orbits: Vec<_> = file.lines().map(|s| s.unwrap()).collect();
        assert_eq!(part2(&orbits),424);
    }
}
