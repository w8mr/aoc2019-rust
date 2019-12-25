use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::sync::{mpsc};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

use termion::{clear,cursor};
use std::time::Duration;

use intcode::*;

pub fn day2(opcodes: &Vec<i64>) -> i64 {
    let (_, input) = mpsc::channel();
    let (output, _) = mpsc::channel();
    let mut context = intcode::Context::new(opcodes.to_vec(), input, output);
    thread::spawn(move || {
        run(&mut context);
        context.read(0)
    }).join().unwrap_or_default()
}

pub fn day5(opcodes: &Vec<i64>, inputs: &Vec<i64>) -> Vec<i64> {
    let (input_send, input) = mpsc::channel();
    let (output, output_recieve) = mpsc::channel();

    inputs.iter().for_each(|&i| {input_send.send(i).unwrap();});

    let mut context = intcode::Context::new(opcodes.to_vec(), input, output);
    thread::spawn(move || {
        run(&mut context);
    });

    output_recieve.iter().collect()
}

pub fn day7(opcodes: &Vec<i64>, phases: Vec<usize>) -> (Vec<usize>, i64) {
    recurse(phases)
        .iter()
        .fold(None, |acc: Option<(Vec<usize>, i64)>, phases| {
            let result = day7_internal(opcodes, &phases);
            match acc {
                Some((p, high)) if high >= result => Some((p, high)),
                _ => Some((phases.clone(), result)),
            }
        }).unwrap()
}

fn day7_internal(opcodes: &Vec<i64>, phases:&Vec<usize>) -> i64 {
    let (sender0, reciever1) = mpsc::channel();
    let (sender1, reciever2) = mpsc::channel();
    let (sender2, reciever3) = mpsc::channel();
    let (sender3, reciever4) = mpsc::channel();
    let (sender4, reciever5) = mpsc::channel();
    let (sender5, reciever0) = mpsc::channel();

    init_amplifier(opcodes, sender1.clone(), reciever1);
    init_amplifier(opcodes, sender2.clone(), reciever2);
    init_amplifier(opcodes, sender3.clone(), reciever3);
    init_amplifier(opcodes, sender4.clone(), reciever4);
    init_amplifier(opcodes, sender5.clone(), reciever5);

    sender4.send(phases[4] as i64).unwrap();
    sender3.send(phases[3] as i64).unwrap();
    sender2.send(phases[2] as i64).unwrap();
    sender1.send(phases[1] as i64).unwrap();
    sender0.send(phases[0] as i64).unwrap();
    sender0.send(0).unwrap();

    std::mem::drop(sender1);
    std::mem::drop(sender2);
    std::mem::drop(sender3);
    std::mem::drop(sender4);
    std::mem::drop(sender5);

    let mut result = 0;
    reciever0.iter().for_each(|n| {
//        println!("iter {}", n);
        result = n;
        sender0.send(n).ok();
    });

    result
}

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

pub fn day11_part1(opcodes: &Vec<i64>, init_color:Color) -> usize {
    let state = run_robot(opcodes, init_color);

    state.panels.len()
}

pub fn day11_part2(opcodes: &Vec<i64>, init_color:Color) -> Vec<Vec<char>> {
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
        run(&mut context);
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

fn init_amplifier(opcodes: &Vec<i64>, sender: Sender<i64>, reciever :Receiver<i64>) -> JoinHandle<()> {
    let mut context = intcode::Context::new(opcodes.to_vec(), reciever, sender);
//    println!("init amplifier");
    thread::spawn(move || {
        run(&mut context);
    })
}

pub fn recurse(input: Vec<usize>) -> Vec<Vec<usize>> {
    if input.len() == 1 {
        vec!(input)
    } else {
        (0..input.len()).flat_map(|index| -> Vec<Vec<usize>> {
            let mut rec_input = input.clone();
            let item = rec_input.remove(index);
            let recurse = recurse(rec_input);
            recurse.iter().map(|vec| {
                let mut v = vec.clone();
                v.push(item);
                v
            }).collect()
        }).collect()
    }
}

pub fn day13_part1(opcodes: &mut Vec<i64>, display: bool, delay: u64) -> usize {
    day13(opcodes, display, delay).0
}

pub fn day13_part2(opcodes: &mut Vec<i64>, display: bool, delay: u64) -> i64 {
    (*opcodes)[0] = 2;
    day13(opcodes, display, delay).1
}

pub fn day13(opcodes: &mut Vec<i64>, display: bool, delay: u64) -> (usize, i64){
    let (input_send, input) = mpsc::channel();
    let (output, output_recieve) = mpsc::channel();

    day13_intcode_computer(opcodes, input, output);
    day13_game_loop(input_send, output_recieve, display, delay)
}

fn day13_game_loop(input_send: Sender<i64>, output_recieve: Receiver<i64>, display: bool, delay: u64) -> (usize, i64) {
    let mut score = 0;
    let mut block_count = 0;
    let mut paddle_position = 0;
    let mut ball_position = 0;
    let mut iteration = 0;

    if display { print!("{}", clear::All); }
    loop {
        let xpos = output_recieve.recv();
        if xpos.is_err() {
            break;
        }
        let xpos = xpos.unwrap();
        let ypos = output_recieve.recv().unwrap();
        let action = output_recieve.recv().unwrap();

        if display { print!("{}", cursor::Goto((xpos + 1) as u16, (ypos + 1) as u16)); }
        match action {
            0 => {
                if display { println!(" "); }
            },
            1 => {
                if display { println!("#"); }
            },
            2 => {
                block_count += 1;
                if display { println!("*"); }
            },
            3 => {
                paddle_position = xpos;
                if display { println!("="); }
            },
            4 => {
                ball_position = xpos;
                let new_direction = new_movement(paddle_position, ball_position);
                input_send.send(new_direction);
                if display { println!("o"); }
                iteration +=1;
                if display { println!("{}Update movement ball ({} {}) paddle {} new direction {} score {}      ", cursor::Goto(1,iteration % 10 + 25 ), ball_position, ypos, paddle_position, new_direction, score); }
                if display { thread::sleep(Duration::from_millis(delay)); }
            },
            new_score => {
                score = new_score;
            }
        }
    }
    if display { print!("{}", cursor::Goto(1, 35)); }

    (block_count, score)
}

fn new_movement(paddle_position: i64, ball_position: i64) -> i64 {
    if ball_position  > paddle_position { 1 }
    else if ball_position  < paddle_position { -1 }
    else { 0 }
}

fn day13_intcode_computer(opcodes: &mut Vec<i64>, input: Receiver<i64>, output: Sender<i64>) {
    let mut context = intcode::Context::new(opcodes.to_vec(), input, output);
    thread::spawn(move || {
        run(&mut context);
    });
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1_opcode99() {
        assert_eq!(day2(&vec!(99)), 99);
    }

    #[test]
    fn test_day2_part1_opcode1() {
        assert_eq!(day2(&vec!(1, 5, 6, 0, 99, 2, 3)), 5);
    }

    #[test]
    fn test_day2_part1_opcode2() {
        assert_eq!(day2(&vec!(2, 5, 6, 0, 99, 2, 3)), 6);
    }

    #[test]
    fn test_day2_part1_example() {
        assert_eq!(day2(&vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50)), 3500);
    }

    #[test]
    fn test_day2_part1_assignment() {
        let mut memory = read_program_from_file("input2.txt");
        memory[1]=12;
        memory[2]=02;
        day2(&memory);
        assert_eq!(day2(&memory), 4138687);
    }

    #[test]
    fn test_day2_part2_assignment() {
        let mut memory = read_program_from_file("input2.txt");
        memory[1]=66;
        memory[2]=35;
        day2(&memory);
        assert_eq!(day2(&memory), 19690720);
    }

    #[test]
    fn test_day5_part1_opcode030499() {
        let outputs = day5(&vec!(3, 0, 4, 0, 99), &vec!(42));
        assert_eq!(outputs.len(),1);
        assert_eq!(outputs[0],42);
    }

    #[test]
    fn test_day5_part1_opcode1002() {
        assert_eq!(day2(&vec!(1002,4,3,4,33)),1002);
    }

    #[test]
    fn test_day5_part1_opcode_negative() {
        assert_eq!(day2(&vec!(1101,100,-1,4,0)),1101);
    }

    #[test]
    fn test_day5_part1_assignment() {
        let memory = read_program_from_file("input5.txt");

        let outputs = day5(&memory, &vec!(1));
        assert_eq!(*outputs.last().unwrap(), 16225258);
    }

    #[test]
    fn test_day5_part2_assignment() {
        let memory = read_program_from_file("input5.txt");

        let outputs = day5(&memory, &vec!(5));
        assert_eq!(*outputs.last().unwrap(), 2808771);
    }

    #[test]
    fn test_day5_part2_example1_not_equal() {
        let outputs = day5(&vec!(3,9,8,9,10,9,4,9,99,-1,8), &vec!(5));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_day5_part2_example1_equal() {
        let outputs = day5(&vec!(3,9,8,9,10,9,4,9,99,-1,8), &vec!(8));
        assert_eq!(outputs, vec!(1));
    }

    #[test]
    fn test_day5_part2_example4_less_than() {
        let outputs = day5(&vec!(3,3,1107,-1,8,3,4,3,99), &vec!(5));
        assert_eq!(outputs, vec!(1));
    }

    #[test]
    fn test_day5_part2_example4_not_less_than() {
        let outputs = day5(&vec!(3,3,1107,-1,8,3,4,3,99), &vec!(8));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_day5_part2_jmp_example1_jmp() {
        let outputs = day5(&vec!(3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9), &vec!(0));
        assert_eq!(outputs, vec!(0));
    }

    #[test]
    fn test_day5_part2_large_example4_less_than() {
        let outputs = day5(&vec!(
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99), &vec!(7));
        assert_eq!(outputs, vec!(999));
    }

    #[test]
    fn test_day5_part2_large_example4_equals() {
        let outputs = day5(&vec!(
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99), &vec!(8));
        assert_eq!(outputs, vec!(1000));
    }

    #[test]
    fn test_day5_part2_large_example4_not_less_than() {
        let outputs = day5(&vec!(
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99), &vec!(9));
        assert_eq!(outputs, vec!(1001));
    }

    #[test]
    fn test_recursive1() {
        let rec = recurse(vec!(1));
        assert_eq!(rec.len(), 1);
        assert!((rec.contains(&vec!(1))));
    }

    #[test]
    fn test_recursive2() {
        let rec = recurse(vec!(1,2));
        assert_eq!(rec.len(), 2);
        assert!((rec.contains(&vec!(1,2))));
        assert!((rec.contains(&vec!(2,1))));
    }

    #[test]
    fn test_recursive3() {
        let rec = recurse(vec!(1,2,3));
        assert_eq!(rec.len(), 6);
        assert!((rec.contains(&vec!(1,2,3))));
        assert!((rec.contains(&vec!(1,3,2))));
        assert!((rec.contains(&vec!(2,1,3))));
        assert!((rec.contains(&vec!(2,3,1))));
        assert!((rec.contains(&vec!(3,1,2))));
        assert!((rec.contains(&vec!(3,2,1))));
    }

    #[test]
    fn test_day7_part1_example1_internal() {
        assert_eq!(day7_internal(&vec!(3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0), &vec!(4,3,2,1,0)), 43210);
    }

    #[test]
    fn test_day7_part1_example2_full() {
        assert_eq!(day7(&vec!(
            3,23,3,24,1002,24,10,24,1002,23,-1,23,
            101,5,23,23,1,24,23,23,4,23,99,0,0), (0..5).collect()), (vec!(0, 1, 2, 3, 4), 54321));
    }

    #[test]
    fn test_day7_part1_example3() {
        assert_eq!(day7_internal(&vec!(
            3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
            1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0), &vec!(1,0,4,3,2)), 65210);
    }

    #[test]
    fn test_day7_part1_example3_full() {
        assert_eq!(day7(&vec!(
            3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
            1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0), (0..5).collect()), (vec!(1, 0, 4, 3, 2), 65210));
    }

    #[test]
    fn test_day7_part2_example1_full() {
        assert_eq!(day7(&vec!(
            3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
            27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5), (5..10).collect()), (vec!(9, 8, 7, 6, 5), 139629729));
    }

    #[test]
    fn test_day7_part2_example2_full() {
        assert_eq!(day7(&vec!(
            3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
            -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
            53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10), (5..10).collect()), (vec!(9, 7, 8, 5, 6), 18216));
    }

    #[test]
    fn test_day7_part1_assignment() {
        let memory = read_program_from_file("input7.txt");
        assert_eq!(day7(&memory, (0..5).collect()), (vec!(0, 1, 2, 4, 3), 225056));
    }

    #[test]
    fn test_day7_part2_assignment() {
        let memory = read_program_from_file("input7.txt");
        assert_eq!(day7(&memory, (5..10).collect()), (vec!(8, 5, 9, 6, 7), 14260332));
    }

    #[test]
    fn test_day9_part1_example1() {
        assert_eq!(day5(&vec!(109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99), &vec!()), vec!(109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99));
    }

    #[test]
    fn test_day9_part1_example2() {
        assert_eq!(day5(&vec!(1102,34915192,34915192,7,4,7,99,0), &vec!()), vec!(1219070632396864));
    }

    #[test]
    fn test_day9_part1_example3() {
        assert_eq!(day5(&vec!(104,1125899906842624,99), &vec!()), vec!(1125899906842624));
    }

    #[test]
    fn test_day9_part1_assignment() {
        let memory = read_program_from_file("input9.txt");
        assert_eq!(day5(&memory, &vec!(1)), vec!(3235019597));
    }

    #[test]
    fn test_day9_part2_assignment() {
        let memory = read_program_from_file("input9.txt");
        assert_eq!(day5(&memory, &vec!(2)), vec!(80274));
    }

    #[test]
    fn test_day13_part1_assignment() {
        let mut memory = read_program_from_file("input13.txt");
        assert_eq!(day13_part1(&mut memory, false, 0), 355);
    }

    #[test]
    fn test_day13_part2_assignment() {
        let mut memory = read_program_from_file("input13.txt");
        assert_eq!(day13_part2(&mut memory, false, 0), 18371);
    }

}
