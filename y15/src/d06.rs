use alloc::{boxed::Box, vec};
use common::{FromUnsigned, PartAnswer};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum InstrStem {
    TurnOn,
    TurnOff,
    Toggle,
}

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let mut grid: Box<[u32]> = vec![0; 1_000_000].into_boxed_slice();

    for line in input.split(|&c| c == b'\n') {
        let mut words = line.split(|&c| c == b' ');

        let stem = if words.next().unwrap()[1] == b'o' {
            InstrStem::Toggle
        } else if words.next().unwrap()[1] == b'n' {
            InstrStem::TurnOn
        } else {
            InstrStem::TurnOff
        };

        let [from_raw, _, to_raw] = words.next_array().unwrap();

        let [from_node, to_node] = [from_raw, to_raw].map(|raw| {
            raw.split(|&c| c == b',')
                .map(usize::from_unsigned)
                .next_array::<2>()
                .unwrap()
        });

        for row in from_node[0]..=to_node[0] {
            let col_start = row * 1000;
            let idx_from = col_start + from_node[1];
            let idx_to = col_start + to_node[1];

            for light in &mut grid[idx_from..=idx_to] {
                *light = match stem {
                    InstrStem::TurnOn => (*light + 2) | 1,
                    InstrStem::TurnOff => light.saturating_sub(2) & 0xFFFF_FFFE,
                    InstrStem::Toggle => (*light + 4) ^ 1,
                }
            }
        }
    }

    grid.iter()
        .fold([0, 0], |[a, b], &light| [a + (light & 1), b + (light >> 1)])
        .map(Into::into)
}
