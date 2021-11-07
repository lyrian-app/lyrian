use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub struct WalkerBoxBuilder {
    index_weights: Vec<u32>,
}

impl WalkerBoxBuilder {
    pub fn new(index_weights: Vec<u32>) -> WalkerBoxBuilder {
        WalkerBoxBuilder {
            index_weights: index_weights,
        }
    }

    pub fn build(&mut self) -> WalkerBox {
        let table_len = self.index_weights.len();

        self.index_weights = self
            .index_weights
            .iter()
            .map(|w| w * self.sum() * table_len as u32)
            .collect::<Vec<u32>>()
            .to_vec();

        let (aliases, tholds) = self.calc_box();

        WalkerBox::new(aliases, tholds, self.mean())
    }

    fn sum(&self) -> u32 {
        self.index_weights.iter().fold(0, |acc, cur| acc + cur)
    }

    fn mean(&self) -> u32 {
        self.sum() / self.index_weights.len() as u32
    }

    fn calc_box(&self) -> (Vec<u32>, Vec<u32>) {
        let table_len = self.index_weights.len();
        let (mut below_vec, mut above_vec) = self.separate_weights();
        let mean = self.mean();

        let mut aliases = vec![0; table_len];
        let mut tholds = vec![0; table_len];
        loop {
            match below_vec.pop() {
                Some(below) => {
                    if let Some(above) = above_vec.pop() {
                        let diff = mean - below.1;
                        aliases[below.0] = above.0 as u32;
                        tholds[below.0] = diff;
                        if above.1 - diff <= mean {
                            below_vec.push((above.0, above.1 - diff));
                        } else {
                            above_vec.push((above.0, above.1 - diff));
                        }
                    } else {
                        aliases[below.0] = below.0 as u32;
                        tholds[below.0] = below.1;
                    }
                }
                None => break,
            }
        }

        (aliases, tholds)
    }

    fn separate_weights(&self) -> (Vec<(usize, u32)>, Vec<(usize, u32)>) {
        let mut below_vec = Vec::new();
        let mut above_vec = Vec::new();
        for (i, w) in self.index_weights.iter().enumerate() {
            if *w <= self.mean() {
                below_vec.push((i, *w));
            } else {
                above_vec.push((i, *w));
            }
        }
        (below_vec, above_vec)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WalkerBox {
    pub aliases: Vec<u32>,
    pub tholds: Vec<u32>,
    pub max_weight: u32,
}

impl WalkerBox {
    pub fn new(aliases: Vec<u32>, tholds: Vec<u32>, max_weight: u32) -> WalkerBox {
        WalkerBox {
            aliases: aliases,
            tholds: tholds,
            max_weight: max_weight,
        }
    }

    pub fn next(&self) -> usize {
        let mut rng = rand::thread_rng();
        let i = rng.gen::<usize>() % self.tholds.len();
        let r = rng.gen_range(0..self.max_weight);
        if r < self.tholds[i] {
            self.aliases[i] as usize
        } else {
            i
        }
    }
}

#[cfg(test)]
mod walkers_alias_method_test {
    use crate::walkers_alias_method::{WalkerBox, WalkerBoxBuilder};

    #[test]
    fn make_box() {
        let index_weights = vec![2, 7, 9, 2, 4, 8, 1, 3, 6, 5];
        let mut builder = WalkerBoxBuilder::new(index_weights);
        let w_box = builder.build();

        let expected = WalkerBox::new(
            vec![2, 1, 1, 2, 2, 2, 5, 9, 5, 8],
            vec![1269, 2209, 1081, 1269, 329, 235, 1739, 799, 47, 658],
            2209,
        );

        assert_eq!(w_box, expected)
    }
}
