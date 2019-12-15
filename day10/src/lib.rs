use std::collections::{HashSet, HashMap};

#[derive(std::fmt::Debug)]
struct Grid {
    asteroids: Vec<Asteroid>,
    height: usize,
    width: usize,
}

#[derive(std::fmt::Debug,std::cmp::PartialEq)]
struct Asteroid {
    index: usize,
    x: usize,
    y: usize,
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
    let asteroids:Vec<&Asteroid> = grid.asteroids.iter().collect();

    let mut asteroids_los: Vec<((usize,usize),usize)> = asteroids.iter().map(|asteroid| ((asteroid.x, asteroid.y), count_line_of_sight(&asteroids, asteroid))).collect();
    asteroids_los.sort_by_key(|e| e.1);
    let winner = asteroids_los.last().unwrap();
//    println!("Astroid ({},{}) can see {} Astroids", (winner.0).0, (winner.0).1, winner.1);

    *winner
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
}