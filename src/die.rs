use std::borrow::Cow;

use itertools::Itertools;
use rand::Rng;

#[derive(Debug)]
pub enum Operation {
    /// Roll 4d[-1, 1], sum the result, show signs.
    Fate,
    /// Roll nd6, keep highest, if there's more than one 6, count it.
    Forged {
        number: u8,
    },
    BloodBowlBlocking {
        number: u8,
    },
}
impl Operation {
    pub fn roll<R: Rng>(&self, rng: &mut R) -> String {
        match *self {
            Operation::Fate => {
                let mut dice = [0; 4];
                for die in dice.iter_mut() {
                    *die = rng.gen_range(-1, 2);
                }
                let sum: i8 = dice.iter().cloned().sum();
                format!("{:+?} = {:+}", dice.iter().format(", "), sum)
            }
            Operation::Forged { number } => {
                let mut dice = Vec::with_capacity(number as usize);
                let mut number_of_sixes = 0u8;
                let mut max = 0;
                for _ in 0..number {
                    let die = rng.gen_range(1u8, 7);
                    if die > max {
                        max = die;
                    }
                    if die == 6 {
                        number_of_sixes += 1;
                    }
                    dice.push(die);
                }
                format!(
                    "[{dice:?}] = {max}{critical}",
                    dice = dice.iter().format(", "),
                    max = max,
                    critical = if number_of_sixes >= 2 {
                        Cow::from(format!("x{}", number_of_sixes))
                    } else {
                        Cow::from("")
                    }
                )
            }
            Operation::BloodBowlBlocking { number } => {
                let mut dice = Vec::with_capacity(number as usize);
                for _ in 0..number {
                    let die = rng.gen_range(0, 6);
                    dice.push(die);
                }
                format!(
                    "[{dice}]",
                    dice = dice
                        .iter()
                        .map(|d| {
                            match *d {
                                0 => "🕱",
                                1 => "🕱💥",
                                2 | 3 => "➚",
                                4 => "🚷",
                                5 => "💥",
                                _ => unreachable!(),
                            }
                        })
                        .format(", ")
                )
            }
        }
    }
}

/*

#[derive(Debug)]
pub enum Operation {
    Roll(SameDice),
    Max(Vec<Operation>),
    Sum(Vec<Operation>),
}

pub enum Operator {
    KeepAll,
    Sum,
    Max { number: u32 },
    Min { number: u32 },
}
*/

/*
#[derive(Debug)]
pub struct SameDice {
    pub multiplicity: u32,
    pub kind: SingleDie,
}
impl SameDice {
    pub fn roll<R: Rng>(&self, rng: &mut R) -> Vec<i32> {
        let ref kind = self.kind;
        (0..self.multiplicity).into_iter()
            .map(|_| kind.roll(rng))
            .collect()
    }
}

pub enum Display {
    Fate,
    Number,
}

#[derive(Debug)]
pub enum SingleDie {
    Range { low: i32, high: i32, display: Display }
}

impl SingleDie {
    pub fn roll<R: Rng>(&self, rng: &mut R) -> i32 {
        match *self {
            SingleDie::Range {ref low, ref high }=> rng.gen_range(*low, *high),
        }
    }
}
*/
