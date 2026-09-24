mod d01;

include!(concat!(env!("OUT_DIR"), "/gen.rs"));

use std::{
    io::{self, StdinLock},
    time::SystemTime,
};

trait IsDay {
    fn part_a(stdin_handle: StdinLock);
    fn part_b(stdin_handle: StdinLock);

    fn part(part: char, stdin_handle: StdinLock) {
        let start_time = SystemTime::now();

        match part {
            'a' => Self::part_a(stdin_handle),
            'b' => Self::part_b(stdin_handle),
            _ => unreachable!(),
        }

        let elapsed = start_time.elapsed().unwrap();
        println!("Time: {elapsed:?}",);
    }
}

fn main() {
    let mut args = std::env::args();
    _ = args.next();
    let day = args.next().unwrap().parse().unwrap();
    let part = args.next().unwrap().parse().unwrap();

    let stdin_handle = io::stdin().lock();

    run_day(day, part);
}
