use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkovModel<T> {
    pub elements: Vec<T>,
    pub state_space: Vec<Vec<f32>>,
    pre_index: usize,
}

impl<T> MarkovModel<T>
where
    T: Clone,
    T: Eq,
    T: Ord,
    T: PartialOrd,
    T: PartialEq,
{
    fn new(elements: Vec<T>, state_space: Vec<Vec<f32>>, pre_index: usize) -> MarkovModel<T> {
        MarkovModel {
            elements: elements,
            state_space: state_space,
            pre_index: pre_index,
        }
    }

    pub fn from(elements: Vec<T>) -> MarkovModel<T> {
        let mut non_dup_elements = elements.clone();
        non_dup_elements.sort();
        non_dup_elements.dedup();

        let elements_len = non_dup_elements.len();

        let mut state_freq = vec![vec![0; elements_len]; elements_len];
        let mut pre_index: Option<usize> = None;
        for token in elements {
            let cur_index = non_dup_elements
                .iter()
                .position(|t| token == *t)
                .expect("There is no token that should exist.");
            if let Some(i) = pre_index {
                state_freq[i][cur_index] += 1;
            }
            pre_index = Some(cur_index);
        }

        let mut state_space = vec![vec![1.0; elements_len]; elements_len];
        for (i, vector) in state_freq.iter().enumerate() {
            let row_sum = vector.iter().fold(0, |acc, cur| acc + cur);
            let mut cumulative_p = 0.0;
            for (j, count) in vector.iter().enumerate() {
                if row_sum != 0 {
                    cumulative_p = cumulative_p + (*count as f32 / row_sum as f32);
                    state_space[i][j] = cumulative_p;
                }
            }
        }

        MarkovModel::new(non_dup_elements, state_space, elements_len)
    }

    pub fn next(&mut self) -> &T {
        let mut rng = rand::thread_rng();
        let f = rng.gen::<f32>();
        let row_index = if self.pre_index != self.elements.len() {
            self.pre_index
        } else {
            rng.gen::<usize>() % self.elements.len()
        };
        let cur_index: usize = {
            let mut res = self.state_space[row_index].len() - 1;
            for (i, p) in self.state_space[row_index].iter().enumerate() {
                if f <= *p {
                    res = i;
                    break;
                }
            }
            res
        };

        self.pre_index = cur_index;
        self.elements
            .get(cur_index)
            .expect("There is no token that should exist.")
    }

    pub fn initialize(&mut self) {
        self.pre_index = self.elements.len();
    }
}
