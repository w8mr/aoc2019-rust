use regex::{Regex};

#[derive(std::fmt::Debug)]
struct Moon {
    position: Vec<i32>,
    velocity: Vec<i32>,
}

impl Moon {
    fn new(position: Vec<i32>) -> Moon {
        Moon {
            position,
            velocity: vec!(0, 0, 0),
        }
    }
}

fn compare_position(pos1: i32, pos2: i32) -> i32 {
    if pos1>pos2 { 1 }
    else if pos2>pos1 { -1 }
    else { 0 }
}

pub fn day12_part1(positions: Vec<Vec<i32>>, steps: u32) -> i32 {
    let mut moons:Vec<Moon> = positions.iter().map(|positions| Moon::new((*positions).clone())).collect();
    for dimension in 0..3 {
        println!("Dimension {}", dimension);
        for step in 0..steps {
            println!("Step {}", step + 1);
            update_velocities(&mut moons, dimension);
            update_positions(&mut moons, dimension)
        }

    }
    let total_energy = sum_energy(&mut moons);
    println!("Total energy {}", total_energy);
    total_energy
}

fn sum_energy(moons: &mut Vec<Moon>) -> i32 {
    moons.iter().map(|moon| energy(&moon)).sum()
}

fn energy(moon: &&Moon) -> i32 {
    potentional_energy(&moon) * kinetic_energy(&moon)
}

fn potentional_energy(moon: &&Moon) -> i32 {
    (0..3).map(|dimension| moon.position[dimension].abs()).sum()
}

fn kinetic_energy(moon: &&Moon) -> i32 {
    (0..3).map(|dimension| moon.velocity[dimension].abs()).sum()
}


fn update_positions(moons: &mut Vec<Moon>, dimension:usize) -> () {
    for moon in moons {
        update_position(moon, dimension);
        println!("Moon ({},{},{}) ({},{},{})", moon.position[0], moon.position[1], moon.position[2], moon.velocity[0], moon.velocity[1], moon.velocity[2]);
    }
}

fn update_velocities(mut moons: &mut Vec<Moon>, dimension:usize) {
    for (index1, index2) in index_pairs(4) {
        update_velocity(&mut moons, index1, index2, dimension);
//        println!("Moon pair ({},{},{}) ({},{},{})", moons[index1].position[0], moons[index1].position[1], moons[index1].position[2], moons[index2].position[0], moons[index2].position[1], moons[index2].position[2]);
    }
}

fn update_velocity(moons: &mut Vec<Moon>, index1: usize, index2: usize, dimension:usize) {
    let velocity_change = compare_position(moons[index1].position[dimension], moons[index2].position[dimension]);
    moons[index1].velocity[dimension] -= velocity_change;
    moons[index2].velocity[dimension] += velocity_change;
}

fn update_position(moon: &mut Moon, dimension:usize) {
    moon.position[dimension] += moon.velocity[dimension];
}

fn index_pairs(index: usize) -> Vec<(usize, usize)> {
    (0..index).flat_map(|i1| ((i1 + 1)..index).map(move |i2| (i1, i2))).collect()
}

pub fn parse_input_line(line: &str) -> Vec<i32> {
    let re = Regex::new(r"<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>").unwrap();
    let caps = re.captures(line).unwrap();
    caps.iter().skip(1).map(|s| s.unwrap().as_str().parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use crate::{day12_part1, parse_input_line};

    #[test]
    fn day12_part1_example1() {
        assert_eq!(day12_part1(vec!(vec!(-1,0,2), vec!(2,-10,-7), vec!(4,-8,8), vec!(3,5,-1)), 10), 179);
    }

    #[test]
    fn day12_part1_example2() {
        assert_eq!(day12_part1(vec!(vec!(-8,-10,0),vec!(5,5,10),vec!(2,-7,3),vec!(9,-8,-3)), 100), 1940);
    }

    #[test]
    fn day12_parse_input_line() {
        assert_eq!(parse_input_line("<x=10, y=7, z=-9>"), vec!(10,7,-9));
    }

}
