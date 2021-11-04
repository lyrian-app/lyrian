use crate::morphological_analysis::LyrianToken;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkovModel {
    pub tokens: Vec<LyrianToken>,
    pub state_space: Vec<Vec<f32>>,
    pre_index: usize,
}

impl MarkovModel {
    fn new(tokens: Vec<LyrianToken>, state_space: Vec<Vec<f32>>, pre_index: usize) -> MarkovModel {
        MarkovModel {
            tokens: tokens,
            state_space: state_space,
            pre_index: pre_index,
        }
    }

    pub fn from_tokens(tokens: Vec<LyrianToken>) -> MarkovModel {
        let mut non_dup_tokens = tokens.clone();
        non_dup_tokens.sort();
        non_dup_tokens.dedup();

        let tokens_len = non_dup_tokens.len();

        let mut state_freq = vec![vec![0; tokens_len]; tokens_len];
        let mut pre_index: Option<usize> = None;
        for token in tokens {
            let cur_index = non_dup_tokens
                .iter()
                .position(|t| token.word == t.word)
                .expect("There is no token that should exist.");
            if let Some(i) = pre_index {
                state_freq[i][cur_index] += 1;
            }
            pre_index = Some(cur_index);
        }

        let mut state_space = vec![vec![0.0; tokens_len]; tokens_len];
        for (i, vector) in state_freq.iter().enumerate() {
            let row_sum = vector.iter().fold(0, |acc, cur| acc + cur);
            let mut cumulative_p = 0.0;
            for (j, count) in vector.iter().enumerate() {
                cumulative_p = cumulative_p + *count as f32 / row_sum as f32;
                state_space[i][j] = cumulative_p;
            }
        }

        MarkovModel::new(non_dup_tokens, state_space, tokens_len)
    }

    pub fn next(&mut self) -> &LyrianToken {
        let mut rng = rand::thread_rng();
        let f = rng.gen::<f32>();
        let row_index = if self.pre_index != self.tokens.len() {
            self.pre_index
        } else {
            rng.gen::<usize>() % self.tokens.len()
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
        self.tokens
            .get(cur_index)
            .expect("There is no token that should exist.")
    }

    pub fn initialize(&mut self) {
        self.pre_index = self.tokens.len();
    }
}
