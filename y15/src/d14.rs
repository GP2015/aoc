use alloc::boxed::Box;
use common::{FromUnsigned, PartAnswer};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Reindeer {
    speed: u8,
    fly_time: u8,
    rest_time: u8,
    dist: u16,
    points: u16,
}

impl Reindeer {
    pub const fn new(speed: u8, fly_time: u8, rest_time: u8) -> Self {
        Self {
            speed,
            fly_time,
            rest_time,
            dist: 0,
            points: 0,
        }
    }

    pub fn step(&mut self, t: u16) {
        let cycle_time = self.fly_time + self.rest_time;

        if t % u16::from(cycle_time) < u16::from(self.fly_time) {
            self.dist += u16::from(self.speed);
        }
    }

    pub const fn check_dist(&mut self, dist: u16) {
        if self.dist == dist {
            self.points += 1;
        }
    }
}

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let mut reindeers: Box<_> = input
        .split(|&c| c == b'\n')
        .map(|line| {
            let mut words = line.split(|&c| c == b' ');
            let mut get_word = |skip: usize| u8::from_unsigned(words.by_ref().nth(skip).unwrap());
            let speed = get_word(3);
            let fly_time = get_word(2);
            let rest_time = get_word(6);
            Reindeer::new(speed, fly_time, rest_time)
        })
        .collect();

    let mut furthest = 0;

    for t in 0..2503 {
        for reindeer in &mut reindeers {
            reindeer.step(t);
        }

        furthest = reindeers.iter().map(|r| r.dist).max().unwrap();

        for reindeer in &mut reindeers {
            reindeer.check_dist(furthest);
        }
    }

    [
        furthest.into(),
        reindeers.iter().map(|r| r.points).max().unwrap().into(),
    ]
}
