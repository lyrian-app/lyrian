use rand::prelude::*;
use serde::{Deserialize, Serialize};

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
