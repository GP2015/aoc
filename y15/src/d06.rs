use alloc::{boxed::Box, vec};
use common::PartAnswer;
use core::fmt::Debug;
use derive_more::{Deref, DerefMut};

#[derive(Clone, Copy, Debug)]
enum InstrStem {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    stem: InstrStem,
    from_node: [u16; 2],
    to_node: [u16; 2],
}

impl Instruction {
    fn next(input_idx: &mut usize, input: &[u8]) -> Self {
        #[allow(clippy::indexing_slicing)]
        let (stem, inc) = if input[*input_idx + 1] == b'o' {
            (InstrStem::Toggle, 7)
        } else if input[*input_idx + 6] == b'n' {
            (InstrStem::TurnOn, 8)
        } else {
            (InstrStem::TurnOff, 9)
        };

        *input_idx += inc;

        let read_number = |input_idx: &mut usize| {
            #[allow(clippy::indexing_slicing)]
            let chunk: [u8; 3] = input[*input_idx..*input_idx + 3].try_into().unwrap();

            let mut num = u16::from(chunk[0] - b'0');

            for &c in &chunk[1..] {
                if !c.is_ascii_digit() {
                    break;
                }

                num *= 10;
                num += u16::from(c - b'0');

                *input_idx += 1;
            }

            *input_idx += 2;

            num
        };

        let x_from = read_number(input_idx);
        let y_from = read_number(input_idx);

        *input_idx += 8;

        let x_to = read_number(input_idx);
        let y_to = read_number(input_idx);

        Self {
            stem,
            from_node: [x_from, y_from],
            to_node: [x_to, y_to],
        }
    }
}

#[derive(Clone, Debug, Deref, DerefMut)]
struct Grid<T>(Box<[T; 1_000_000]>);

impl<T> Grid<T>
where
    T: Debug + Default + Clone + Copy,
{
    pub fn new() -> Self {
        Self(
            vec![T::default(); 1_000_000]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
        )
    }

    pub fn set(&mut self, from_node: [u16; 2], to_node: [u16; 2], effect: impl Fn(T) -> T) {
        for row in from_node[0] as usize..=to_node[0] as usize {
            for col in from_node[1] as usize..=to_node[1] as usize {
                let idx = row * 1000 + col;
                #[allow(clippy::indexing_slicing)]
                let light = &mut self[idx];
                *light = effect(*light);
            }
        }
    }
}

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let mut digital_grid = Grid::<bool>::new();
    let mut analogue_grid = Grid::<u32>::new();

    let mut input_idx = 0;

    while input_idx < input.len() {
        let instr = Instruction::next(&mut input_idx, input);

        macro_rules! set {
            ($grid:ident, $effect:expr) => {
                $grid.set(instr.from_node, instr.to_node, $effect)
            };
        }

        match instr.stem {
            InstrStem::TurnOn => set!(digital_grid, |_| true),
            InstrStem::TurnOff => set!(digital_grid, |_| false),
            InstrStem::Toggle => set!(digital_grid, |s| !s),
        }

        match instr.stem {
            InstrStem::TurnOn => set!(analogue_grid, |s| s + 1),
            InstrStem::TurnOff => set!(analogue_grid, |s| s.saturating_sub(1)),
            InstrStem::Toggle => set!(analogue_grid, |s| s + 2),
        }
    }

    let digital_total = digital_grid.0.iter().filter(|&&light| light).count();
    let analogue_total: u32 = analogue_grid.0.iter().sum();

    [digital_total.into(), analogue_total.into()]
}
