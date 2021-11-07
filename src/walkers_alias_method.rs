use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WalkerTable {
    pub aliases: Vec<u32>,
    pub tholds: Vec<u32>,
    pub max_weight: u32,
}

impl WalkerTable {
    pub fn new(aliases: Vec<u32>, tholds: Vec<u32>, max_weight: u32) -> WalkerTable {
        WalkerTable {
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
    use crate::builder::WalkerTableBuilder;

    #[test]
    fn unweighted_random_sampling() {
        let index_weights = vec![1, 1, 1, 1];
        let mut builder = WalkerTableBuilder::new(index_weights);
        let wa_table = builder.build();

        const N: usize = 100_000;
        const P: f32 = 0.25;
        const EXPT: f32 = N as f32 * P;

        let idxs = (0..N)
            .map(|_| wa_table.next())
            .collect::<Vec<usize>>()
            .to_vec();

        let i_0 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 0 { acc + 1 } else { acc }) as f32;
        let i_1 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 1 { acc + 1 } else { acc }) as f32;
        let i_2 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 2 { acc + 1 } else { acc }) as f32;
        let i_3 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 3 { acc + 1 } else { acc }) as f32;

        assert!(
            (EXPT * 0.95 < i_0 && i_0 < EXPT * 1.05)
                && (EXPT * 0.95 < i_1 && i_1 < EXPT * 1.05)
                && (EXPT * 0.95 < i_2 && i_2 < EXPT * 1.05)
                && (EXPT * 0.95 < i_3 && i_3 < EXPT * 1.05)
        )
    }

    #[test]
    fn weighted_random_sampling() {
        let index_weights = vec![1, 2, 3, 4];
        let mut builder = WalkerTableBuilder::new(index_weights);
        let wa_table = builder.build();

        const N: usize = 100_000;
        const EXPT: [f32; 4] = [
            N as f32 * 0.1,
            N as f32 * 0.2,
            N as f32 * 0.3,
            N as f32 * 0.4,
        ];

        let idxs = (0..N)
            .map(|_| wa_table.next())
            .collect::<Vec<usize>>()
            .to_vec();

        let i_0 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 0 { acc + 1 } else { acc }) as f32;
        let i_1 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 1 { acc + 1 } else { acc }) as f32;
        let i_2 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 2 { acc + 1 } else { acc }) as f32;
        let i_3 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 3 { acc + 1 } else { acc }) as f32;

        assert!(
            (EXPT[0] * 0.95 < i_0 && i_0 < EXPT[0] * 1.05)
                && (EXPT[1] * 0.95 < i_1 && i_1 < EXPT[1] * 1.05)
                && (EXPT[2] * 0.95 < i_2 && i_2 < EXPT[2] * 1.05)
                && (EXPT[3] * 0.95 < i_3 && i_3 < EXPT[3] * 1.05)
        )
    }
}
