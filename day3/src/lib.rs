#[derive(Debug)]
struct Path {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
    horizontal: bool
}

struct Context {
    smallest: Option<i32>,
    distance_path1: i32,
    distance_path2: i32
}

fn distance(x:i32, y: i32) -> i32 {
    return x.abs() + y.abs();
}
pub fn part1(wires: Vec<String>) -> i32 {
    let wire_paths:Vec<Vec<Path>> = wires.iter().map(|w| read_paths(w)).collect();
    let r = find_smallest_cross(wire_paths.get(0).unwrap(), wire_paths.get(1).unwrap(), distance);

    return r.smallest.unwrap();
}

fn find_smallest_cross(paths1: &Vec<Path>, paths2: &Vec<Path>, distance: fn(i32, i32) -> i32) -> Context {
    paths1.iter().fold(Context{smallest: None, distance_path1: 0, distance_path2: 0},
        |context, p| find_smallest_cross1(context, p, paths2, distance)
    )
}

fn find_smallest_cross1(context: Context, path1: &Path, paths: &Vec<Path>, distance: fn(i32, i32) -> i32) -> Context {
    return paths.iter().fold(context, |context, path2| find_smallest_cross2(context, path1, path2, distance))
}

fn find_smallest_cross2(context: Context, path1: &Path, path2: &Path, distance: fn(i32, i32) -> i32) -> Context {
    let new_smallest = match find_cross(path1, path2) {
        Some((0, 0)) => context.smallest,
        Some((x, y)) => {
            let d = distance(x, y);
            match context.smallest {
                Some(s) if s < d => Some(s),
                _ => Some(d)
            }
        },
        _ => context.smallest
    };
    Context{smallest: new_smallest, distance_path1: context.distance_path1, distance_path2: context.distance_path2}
}



fn is_crossed(path1: &Path, path2: &Path) -> Option<(i32, i32)> {
    return if path2.x_start.min(path2.x_end) >= path1.x_start.min(path1.x_end) && path2.x_start.min(path2.x_end) <= path1.x_start.max(path1.x_end) &&
        path1.y_start.min(path1.y_end) >= path2.y_start.min(path2.y_end) && path1.y_start.min(path1.y_end) <= path2.y_start.max(path2.y_end)
    { Some((path2.x_start.min(path2.x_end), path1.y_start.min(path1.y_end))) } else { None };
}

fn find_cross(path1: &Path, path2: &Path) -> Option<(i32, i32)> {
    let result = match (path1.horizontal, path2.horizontal) {
        (true, false) =>
            is_crossed(path1, path2),
        (true, true) | (false, false) => None,
        (false, true) => is_crossed(path2, path1),
    };
    return result
}

fn read_paths(wire: &String) -> Vec<Path> {
    let mut paths = Vec::new();
    let mut x = 0;
    let mut y = 0;

    let moves = wire.split(",");
    for mv in moves {
        let (dir, steps) = mv.split_at(1);
        let steps: i32 = steps.parse().unwrap();
        match dir {
            "R" => {
                let x_new = x + steps;
                paths.push(Path { x_start: x, x_end: x_new, y_start: y, y_end: y, horizontal: true });
                x = x_new;
            },
            "L" => {
                let x_new = x - steps;
                paths.push(Path { x_start: x, x_end: x_new, y_start: y, y_end: y, horizontal: true });
                x = x_new;
            },
            "U" => {
                let y_new = y - steps;
                paths.push(Path { x_start: x, x_end: x, y_start: y, y_end: y_new, horizontal: false });
                y = y_new;
            },
            "D" => {
                let y_new = y + steps;
                paths.push(Path { x_start: x, x_end: x, y_start: y, y_end: y_new, horizontal: false });
                y = y_new;
            },
            _ => panic!()
        }
    }

    return paths;
}


#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;

    use super::*;

    #[test]
    fn part1_example1() {
        let mut wires = Vec::new();
        wires.push("R8,U5,L5,D3".to_string());
        wires.push("U7,R6,D4,L4".to_string());
        assert_eq!(part1(wires), 6);
    }

    #[test]
    fn part1_example2() {
        let mut wires = Vec::new();
        wires.push("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string());
        wires.push("U62,R66,U55,R34,D71,R55,D58,R83".to_string());
        assert_eq!(part1(wires), 159);
    }

    #[test]
    fn part1_example3() {
        let mut wires = Vec::new();
        wires.push("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string());
        wires.push("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string());
        assert_eq!(part1(wires), 135);
    }

    #[test]
    fn part1_assignment() {
        let f = File::open("input.txt").unwrap();
        let file = BufReader::new(&f);
        let wires: Vec<_> = file.lines().map(|l| l.unwrap()).collect();
        assert_eq!(part1(wires), 721);

    }

}
