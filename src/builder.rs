use crate::walkers_alias_method::WalkerBox;

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

#[cfg(test)]
mod builder_test {
    use crate::builder::WalkerBoxBuilder;
    use crate::walkers_alias_method::WalkerBox;

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
