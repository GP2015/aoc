use core::{array, iter};

use alloc::boxed::Box;
use common::{FromSigned, PartAnswer};
use derive_more::{Deref, DerefMut};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
struct Ingredient([i8; 5]);

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let ingredients: Box<_> = input
        .split(|&c| c == b'\n')
        .map(|line| {
            let mut words = line.split(|&c| c == b' ').skip(1);

            let [capacity, durability, flavour, texture] = (0..4)
                .map(|_| words.nth(1).unwrap())
                .map(|w| i8::from_signed(&w[..w.len() - 1]))
                .next_array()
                .unwrap();

            let calories = i8::from_signed(words.nth(1).unwrap());

            Ingredient([capacity, durability, flavour, texture, calories])
        })
        .collect();

    (1..100)
        .combinations(ingredients.len() - 1)
        .map(|comb| {
            let arr = iter::once(comb[0])
                .chain(comb.array_windows().map(|w: &[_; 2]| w[1] - w[0]))
                .chain(iter::once(100 - comb.last().unwrap()))
                .zip(ingredients.iter())
                .map(|(quantity, ingredient)| ingredient.map(|i| i32::from(i) * quantity))
                .fold([0; 5], |acc, arr| array::from_fn(|i| acc[i] + arr[i]));

            let prod = arr[0..4]
                .iter()
                .map(|&i| if i < 0 { 0 } else { i })
                .product();

            (prod, arr[4] == 500)
        })
        .fold([0, 0], |[a, b], (prod, cond)| {
            [a.max(prod), if cond { b.max(prod) } else { b }]
        })
        .map(Into::into)
}
