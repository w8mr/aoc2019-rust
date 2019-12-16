use std::collections::{HashMap};
use std::cmp::Ordering::Equal;
use std::f32::consts::{PI};

#[derive(std::fmt::Debug)]
struct Grid {
    asteroids: Vec<Asteroid>,
    height: usize,
    width: usize,
}

#[derive(std::fmt::Debug,std::cmp::PartialEq)]
pub struct Asteroid {
    index: usize,
    pub x: usize,
    pub y: usize,
}

fn gcd(mut a: isize, mut b: isize) -> usize {
    let mut t;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    return a.abs() as usize
}

fn read_grid(input: Vec<&str>) -> Grid {
    let height = input.len();
    let width = input[0].len();
    let asteroids = input.iter().enumerate()
        .flat_map(|(y, &line)|
            line.chars()
                .enumerate()
                .map(move |(x, ch)| (y * width + x, x, y, ch))
        ).filter(|&(_, _, _, ch)| ch == '#')
        .map(|(index,x , y, _)| Asteroid { index, x, y })
        .collect();
    Grid { asteroids, height, width }
}

fn distance_angle(a: &Asteroid, b: &Asteroid) -> ((isize, isize), usize){
    let dif_y = b.y as isize - a.y as isize;
    let dif_x = b.x as isize - a.x as isize;
    let gcd = gcd(dif_x, dif_y);
    let dif_x = dif_x/gcd as isize;
    let dif_y = dif_y/gcd as isize;
    ((dif_y, dif_x), gcd)
}

pub fn part1(input: Vec<&str>) -> ((usize, usize), usize) {
    let grid = read_grid(input);

    let winner = find_most_line_of_sight(&grid.asteroids.iter().collect());
//    println!("Astroid ({},{}) can see {} Astroids", (winner.0).0, (winner.0).1, winner.1);

    ((winner.0.x, winner.0.y), winner.1)
}

fn find_most_line_of_sight<'a>(asteroids: &Vec<&'a Asteroid>) -> (&'a Asteroid, usize) {
    let mut asteroids_los: Vec<(&Asteroid, usize)> = asteroids.iter().map(|asteroid| (*asteroid, count_line_of_sight(&asteroids, asteroid))).collect();
    asteroids_los.sort_by_key(|e| e.1);
    let winner = asteroids_los.last().unwrap();
    *winner
}

pub fn part2(input: Vec<&str>, index: usize) -> (usize, usize) {
    let grid = read_grid(input);

    let destroy_asteroids:Vec<&Asteroid> = sweep_destroy_asteroids(&grid);
    (destroy_asteroids[index].x, destroy_asteroids[index].y)
}

fn sweep_destroy_asteroids(grid: &Grid) -> Vec<&Asteroid>{
    let asteroids = &grid.asteroids.iter().collect();
    let winner = find_most_line_of_sight(asteroids);

    let angles_map = partition_by_angle(asteroids, &winner.0);
    let angles = find_angles(&angles_map);
    let mut index = 0;
    let mut destroyed: Vec<&Asteroid> = Vec::new();
    while destroyed.len() < asteroids.len() - 1 {
        println!("Sweep {}", index + 1);
        for a in angles.clone() {
            let found_asteroid = angles_map.get(&a).unwrap().get(index);
            match found_asteroid {
                Some(&(_, asteroid)) => {
                    println!("Angle {} {} {:#?}", a.0, a.1, asteroid);
                    destroyed.push(&asteroid);
                },
                _ => {}
            }
        }
        index += 1;
    }
    destroyed
}

fn find_angles(angles_map: &HashMap<(isize, isize), Vec<(usize, & Asteroid)>>) -> Vec<(isize, isize)>{
    let mut angles: Vec<(isize, isize)> = angles_map.keys().map(|&k| k).collect();
    angles.sort_by(|(ady, adx), (bdy, bdx)| angle(ady, adx).partial_cmp(&angle(bdy, bdx)).unwrap_or(Equal));
    angles
}

fn angle(dy: &isize , dx: &isize) -> f32 {
    ((*dy as f32).atan2(*dx as f32) + 2.5 * PI) % (2f32 * PI)
}

fn count_line_of_sight(asteroids: &Vec<&Asteroid>, asteroid_from: &&Asteroid) -> usize {
    partition_by_angle(asteroids, asteroid_from).len()
}

fn partition_by_angle<'a>(asteroids: &Vec<&'a Asteroid>, asteroid_from: &&Asteroid) -> HashMap<(isize, isize), Vec<(usize, &'a Asteroid)>> {
    let mut angles: HashMap<(isize, isize), Vec<(usize, &Asteroid)>> = HashMap::new();
    for astroid_to in asteroids {
        if *asteroid_from != *astroid_to {
            let (angle, range) = distance_angle(asteroid_from, astroid_to);
            if angles.contains_key(&angle) {
                let mut astroids_for_angle:Vec<(usize, &Asteroid)> = angles.remove(&angle).unwrap();
                astroids_for_angle.push((range, *astroid_to));
                astroids_for_angle.sort_by_key(|e| e.0);
                angles.insert(angle, astroids_for_angle);
            } else {
                angles.insert(angle, vec!((range, *astroid_to)));
            }
        }
    }
    angles
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Error};
    use std::io::BufRead;
    use std::fs::File;

    use super::*;

    #[test]
    fn test_read_grid() {
        let grid = read_grid(vec!("..#", ".#.", "#.."));
        assert_eq!(grid.asteroids.len(), 3);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!((grid.asteroids[0].index), 2);
        assert_eq!((grid.asteroids[1].index), 4);
        assert_eq!((grid.asteroids[2].index), 6);
    }

    #[test]
    fn test_read_grid2() {
        let grid = read_grid(vec!(".#..#", ".....", "#####", "....#", "...##"));
        assert_eq!(grid.asteroids.len(), 10);
        assert_eq!(grid.width, 5);
        assert_eq!(grid.height, 5);
        assert_eq!((grid.asteroids[0].index), 1);
    }

    #[test]
    fn test_gcd_1() {
        assert_eq!(gcd(60, 24), 12);
    }

    #[test]
    fn test_gcd_2() {
        assert_eq!(gcd(35, 6), 1);
    }

    #[test]
    fn test_gcd_4() {
        assert_eq!(gcd(462, 1071), 21);
    }

    #[test]
    fn test_gcd_neg_1() {
        assert_eq!(gcd(60, -24), 12);
    }

    #[test]
    fn test_gcd_neg_2() {
        assert_eq!(gcd(-60, -24), 12);
    }

    #[test]
    fn test_distance_angle() {
        let astroids = read_grid(vec!("..#", ".#.", "#..")).asteroids;
        let ((dy, dx), range) = distance_angle(&astroids[0], &astroids[2]);
        assert_eq!(dy, 1);
        assert_eq!(dx, -1);
        assert_eq!(range, 2);

        let ((dy, dx), range) = distance_angle(&astroids[1], &astroids[0]);
        assert_eq!(dy, -1);
        assert_eq!(dx, 1);
        assert_eq!(range, 1);
    }

    #[test]
    fn test_day10_part1() {
        let ((x, y), c) = part1(vec!("..#", ".#.", "#.."));
        assert_eq!(x, 1);
        assert_eq!(y, 1);
        assert_eq!(c, 2);
    }

    #[test]
    fn test_day10_part1_example_1() {
        let ((x, y), c) = part1(vec!(".#..#", ".....", "#####", "....#", "...##"));
        assert_eq!(x, 3);
        assert_eq!(y, 4);
        assert_eq!(c, 8);
    }

    #[test]
    fn test_day10_part1_example_2() {
        let ((x, y), c) = part1(vec!("......#.#.", "#..#.#....", "..#######.", ".#.#.###..", ".#..#.....", "..#....#.#", "#..#....#.", ".##.#..###", "##...#..#.", ".#....####"));
        assert_eq!(x, 5);
        assert_eq!(y, 8);
        assert_eq!(c, 33);
    }

    #[test]
    fn test_day10_part1_example_3() {
        let ((x, y), c) = part1(vec!("#.#...#.#.", ".###....#.", ".#....#...", "##.#.#.#.#", "....#.#.#.", ".##..###.#", "..#...##..", "..##....##", "......#...", ".####.###."));
        assert_eq!(x, 1);
        assert_eq!(y, 2);
        assert_eq!(c, 35);
    }

    #[test]
    fn test_day10_part1_example_4() {
        let ((x, y), c) = part1(vec!(".#..#..###", "####.###.#", "....###.#.", "..###.##.#", "##.##.#.#.", "....###..#", "..#.#..#.#", "#..#.#.###", ".##...##.#", ".....#.#.."));
        assert_eq!(x, 6);
        assert_eq!(y, 3);
        assert_eq!(c, 41);
    }

    #[test]
    fn test_day10_part1_example_5() {
        let ((x, y), c) = part1(vec!(".#..##.###...#######", "##.############..##.", ".#.######.########.#", ".###.#######.####.#.", "#####.##.#.##.###.##", "..#####..#.#########", "####################", "#.####....###.#.#.##", "##.#################", "#####.##.###..####..", "..######..##.#######", "####.##.####...##..#", ".#####..#.######.###", "##...#.##########...", "#.##########.#######", ".####.#.###.###.#.##", "....##.##.###..#####", ".#.#.###########.###", "#.#.#.#####.####.###", "###.##.####.##.#..##"));
        assert_eq!(x, 11);
        assert_eq!(y, 13);
        assert_eq!(c, 210);
    }

    #[test]
    fn test_day10_part1_assignment() {
        let f = File::open("input10.txt").unwrap();
        let file = BufReader::new(&f);
        let raw_lines: Vec<Result<String, Error>> = file.lines().collect();
        let grid_lines: Vec<&str> = raw_lines.iter().map(|l| l.as_ref().unwrap()).map(|s|s.as_str()).collect();
        let ((x, y), c) = part1(grid_lines);
        assert_eq!(x, 26);
        assert_eq!(y, 29);
        assert_eq!(c, 299);
    }

    #[test]
    fn test_sweep_destroy_example5() {
        let grid = read_grid(vec!(".#..##.###...#######", "##.############..##.", ".#.######.########.#", ".###.#######.####.#.", "#####.##.#.##.###.##", "..#####..#.#########", "####################", "#.####....###.#.#.##", "##.#################", "#####.##.###..####..", "..######..##.#######", "####.##.####...##..#", ".#####..#.######.###", "##...#.##########...", "#.##########.#######", ".####.#.###.###.#.##", "....##.##.###..#####", ".#.#.###########.###", "#.#.#.#####.####.###", "###.##.####.##.#..##"));
        let destroyed_asteroids = sweep_destroy_asteroids(&grid);
        assert_eq!(destroyed_asteroids[0].x, 11);
        assert_eq!(destroyed_asteroids[0].y, 12);
        assert_eq!(destroyed_asteroids[199].x, 8);
        assert_eq!(destroyed_asteroids[199].y, 2);
        assert_eq!(destroyed_asteroids[298].x, 11);
        assert_eq!(destroyed_asteroids[298].y, 1);
    }

    #[test]
    fn test_day10_part2_example_1() {
        let asteroid = part2(vec!(".#..#", ".....", "#####", "....#", "...##"), 5);
        assert_eq!(asteroid.0, 0);
        assert_eq!(asteroid.1, 2);
    }

    #[test]
    fn test_day10_part2_assignment() {
        let f = File::open("input10.txt").unwrap();
        let file = BufReader::new(&f);
        let raw_lines: Vec<Result<String, Error>> = file.lines().collect();
        let grid_lines: Vec<&str> = raw_lines.iter().map(|l| l.as_ref().unwrap()).map(|s|s.as_str()).collect();
        let asteroid = part2(grid_lines, 199);
        assert_eq!(asteroid.0, 14);
        assert_eq!(asteroid.1, 19);
    }


}