use std::sync::{mpsc};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

pub fn run_simulation(opcodes: &Vec<i64>, phases: Vec<usize>) -> (Vec<usize>, i64) {
    recurse(phases)
        .iter()
        .fold(None, |acc: Option<(Vec<usize>, i64)>, phases| {
            let result = internal(opcodes, &phases);
            match acc {
                Some((p, high)) if high >= result => Some((p, high)),
                _ => Some((phases.clone(), result)),
            }
        }).unwrap()
}

fn internal(opcodes: &Vec<i64>, phases:&Vec<usize>) -> i64 {
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

fn init_amplifier(opcodes: &Vec<i64>, sender: Sender<i64>, reciever :Receiver<i64>) -> JoinHandle<()> {
    let mut context = intcode::Context::new(opcodes.to_vec(), reciever, sender);
//    println!("init amplifier");
    thread::spawn(move || {
        intcode::run(&mut context);
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
#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(internal(&vec!(3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0), &vec!(4, 3, 2, 1, 0)), 43210);
    }

    #[test]
    fn test_day7_part1_example2_full() {
        assert_eq!(run_simulation(&vec!(
            3,23,3,24,1002,24,10,24,1002,23,-1,23,
            101,5,23,23,1,24,23,23,4,23,99,0,0), (0..5).collect()), (vec!(0, 1, 2, 3, 4), 54321));
    }

    #[test]
    fn test_day7_part1_example3() {
        assert_eq!(internal(&vec!(
            3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
            1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0), &vec!(1,0,4,3,2)), 65210);
    }

    #[test]
    fn test_day7_part1_example3_full() {
        assert_eq!(run_simulation(&vec!(
            3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
            1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0), (0..5).collect()), (vec!(1, 0, 4, 3, 2), 65210));
    }

    #[test]
    fn test_day7_part2_example1_full() {
        assert_eq!(run_simulation(&vec!(
            3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
            27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5), (5..10).collect()), (vec!(9, 8, 7, 6, 5), 139629729));
    }

    #[test]
    fn test_day7_part2_example2_full() {
        assert_eq!(run_simulation(&vec!(
            3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
            -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
            53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10), (5..10).collect()), (vec!(9, 7, 8, 5, 6), 18216));
    }

    #[test]
    fn test_day7_part1_assignment() {
        let memory = intcode::read_program_from_file("input7.txt");
        assert_eq!(run_simulation(&memory, (0..5).collect()), (vec!(0, 1, 2, 4, 3), 225056));
    }

    #[test]
    fn test_day7_part2_assignment() {
        let memory = intcode::read_program_from_file("input7.txt");
        assert_eq!(run_simulation(&memory, (5..10).collect()), (vec!(8, 5, 9, 6, 7), 14260332));
    }
}
