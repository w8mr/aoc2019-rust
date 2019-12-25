use std::sync::{mpsc};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use termion::{clear,cursor};
use std::time::Duration;

pub fn run_simulation_part1(opcodes: &mut Vec<i64>, display: bool, delay: u64) -> usize {
    run_simulation(opcodes, display, delay).0
}

pub fn run_simulation_part2(opcodes: &mut Vec<i64>, display: bool, delay: u64) -> i64 {
    (*opcodes)[0] = 2;
    run_simulation(opcodes, display, delay).1
}

pub fn run_simulation(opcodes: &mut Vec<i64>, display: bool, delay: u64) -> (usize, i64){
    let (input_send, input) = mpsc::channel();
    let (output, output_recieve) = mpsc::channel();

    run_intcode_computer(opcodes, input, output);
    game_loop(input_send, output_recieve, display, delay)
}

fn game_loop(input_send: Sender<i64>, output_recieve: Receiver<i64>, display: bool, delay: u64) -> (usize, i64) {
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

fn run_intcode_computer(opcodes: &mut Vec<i64>, input: Receiver<i64>, output: Sender<i64>) {
    let mut context = intcode::Context::new(opcodes.to_vec(), input, output);
    thread::spawn(move || {
        intcode::run(&mut context);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13_part1_assignment() {
        let mut memory = intcode::read_program_from_file("input13.txt");
        assert_eq!(run_simulation_part1(&mut memory, false, 0), 355);
    }

    #[test]
    fn test_day13_part2_assignment() {
        let mut memory = intcode::read_program_from_file("input13.txt");
        assert_eq!(run_simulation_part2(&mut memory, false, 0), 18371);
    }

}
