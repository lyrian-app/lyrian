use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WalkerBox {
    pub aliases: Vec<usize>,
    pub tholds: Vec<usize>,
    pub max_weight: usize,
}

impl WalkerBox {
    pub fn new(aliases: Vec<usize>, tholds: Vec<usize>, max_weight: usize) -> WalkerBox {
        WalkerBox {
            aliases: aliases,
            tholds: tholds,
            max_weight: max_weight,
        }
    }

    pub fn from_index_weights(index_weights: Vec<usize>) -> WalkerBox {
        let table_len = index_weights.len();

        let sum = index_weights.iter().fold(0, |acc, cur| acc + cur);
        let index_weights = index_weights
            .iter()
            .map(|w| w * sum * table_len)
            .collect::<Vec<usize>>()
            .to_vec();

        let sum = index_weights.iter().fold(0, |acc, cur| acc + cur);
        let mean = sum / table_len;

        let mut below_vec = Vec::new();
        let mut above_vec = Vec::new();
        for (i, w) in index_weights.iter().enumerate() {
            if *w <= mean {
                below_vec.push((i, *w));
            } else {
                above_vec.push((i, *w));
            }
        }

        let mut aliases = vec![0; table_len];
        let mut tholds = vec![0; table_len];

        loop {
            match below_vec.pop() {
                Some(below) => {
                    if let Some(above) = above_vec.pop() {
                        let diff = mean - below.1;
                        aliases[below.0] = above.0;
                        tholds[below.0] = diff;
                        if above.1 - diff <= mean {
                            below_vec.push((above.0, above.1 - diff));
                        } else {
                            above_vec.push((above.0, above.1 - diff));
                        }
                    } else {
                        aliases[below.0] = below.0;
                        tholds[below.0] = below.1;
                    }
                }
                None => break,
            }
        }

        WalkerBox::new(aliases, tholds, mean)
    }
}

#[cfg(test)]
mod walkers_alias_method_test {
    use crate::walkers_alias_method::WalkerBox;

    #[test]
    fn make_box() {
        let index_weights = vec![2, 7, 9, 2, 4, 8, 1, 3, 6, 5];
        let w_box = WalkerBox::from_index_weights(index_weights);

        let expected = WalkerBox::new(
            vec![2, 1, 1, 2, 2, 2, 5, 9, 5, 8],
            vec![1269, 2209, 1081, 1269, 329, 235, 1739, 799, 47, 658],
            2209,
        );

        assert_eq!(w_box, expected)
    }
}
