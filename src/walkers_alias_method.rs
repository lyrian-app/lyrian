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
