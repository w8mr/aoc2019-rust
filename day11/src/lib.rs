use std::collections::HashMap;
use std::sync::{mpsc};
use std::thread;

#[derive(std::fmt::Debug)]
pub enum Color {
    Black,
    White,
}


#[derive(std::fmt::Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(std::fmt::Debug)]
struct State{
    x: i32,
    y: i32,
    direction: Direction,
    panels: HashMap<(i32, i32), Color>,
}

pub fn run_simulation_part1(opcodes: &Vec<i64>, init_color:Color) -> usize {
    let state = run_robot(opcodes, init_color);

    state.panels.len()
}

pub fn run_simulation_part2(opcodes: &Vec<i64>, init_color:Color) -> Vec<Vec<char>> {
    let state = run_robot(opcodes, init_color);

    let mut lines:Vec<Vec<char>> = Vec::new();
    for ((x,y), color) in state.panels.iter() {
        if lines.len() < *y as usize + 1 {
            lines.resize_with(*y as usize + 1, || Vec::new())
        }
        let line: &mut Vec<char> = lines[*y as usize].as_mut();
        if line.len() < *x as usize + 1 {
            line.resize_with(*x as usize + 1, || '.')
        }
        line[*x as usize] = match color { Color::Black => '.', Color::White => '#' };
    }

    lines
}

fn run_robot(opcodes: &Vec<i64>, init_color: Color) -> State {
    fn step(input: i64, state: &mut State) {
        state.direction = match (input, &state.direction) {
            (0, Direction::Up) => Direction::Left,
            (0, Direction::Left) => Direction::Down,
            (0, Direction::Down) => Direction::Right,
            (0, Direction::Right) => Direction::Up,
            (1, Direction::Up) => Direction::Right,
            (1, Direction::Right) => Direction::Down,
            (1, Direction::Down) => Direction::Left,
            (1, Direction::Left) => Direction::Up,
            _ => panic!("Illegal argument input {}",input)
        };
        match state.direction {
            Direction::Left => state.x -= 1,
            Direction::Down => state.y += 1,
            Direction::Right => state.x += 1,
            Direction::Up => state.y -= 1,
        };
    }

    fn paint(input: i64, state: &mut State) {
        match input {
            0 => state.panels.insert((state.x, state.y), Color::Black),
            1 => state.panels.insert((state.x, state.y), Color::White),
            _ => panic!("Illegal argument input {}",input)
        };
    }

    fn read_color(state: &State) -> &Color {
        state.panels.get(&(state.x, state.y)).unwrap_or(&Color::Black)
    }


    let (input_send, input) = mpsc::channel();
    let (output, output_recieve) = mpsc::channel();
    let mut state = State {
        x: 0,
        y: 0,
        direction: Direction::Up,
        panels: HashMap::new(),
    };
    state.panels.insert((0, 0), init_color);
    let mut context = intcode::Context::new(opcodes.to_vec(), input, output);
    thread::spawn(move || {
        intcode::run(&mut context);
    });
    loop {
        input_send.send(match read_color(&state) {
            Color::Black => 0,
            Color::White => 1
        }).unwrap();
        let paint_command = output_recieve.recv();
        if paint_command.is_err() { break }
        paint(paint_command.unwrap(), &mut state);
        step(output_recieve.recv().unwrap(), &mut state);
        //println!("x:{}, y:{}, new{:#?}", &state.x, &state.y, paint_command.unwrap())
    }
    state
}

#[cfg(test)]
mod tests {

}
