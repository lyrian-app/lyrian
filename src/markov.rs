use crate::morphological_analysis::LyrianToken;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkovModel {
    pub states: Vec<MarkovState>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkovState {
    pub token: LyrianToken,
    pub state_space: Vec<MarkovProbability>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkovProbability {
    pub token: LyrianToken,
    pub probability: f32,
}

struct MarkovCounter {
    token: LyrianToken,
    count: i32,
}

struct MarkovChain {
    token: LyrianToken,
    chain_tokens: Vec<LyrianToken>,
}

impl MarkovModel {
    fn new(states: Vec<MarkovState>) -> MarkovModel {
        MarkovModel { states: states }
    }

    pub fn from_tokens(tokens: Vec<LyrianToken>) -> MarkovModel {
        let chains = MarkovChain::from_tokens(tokens);
        let states = MarkovState::from_chains(chains);
        MarkovModel::new(states)
    }

    pub fn get_random_state(&self, word_len: usize, rhythmical: bool) -> Option<&MarkovState> {
        let mut filtered = Vec::new();
        for state in &self.states {
            if state.token.length(rhythmical) == word_len {
                filtered.push(state)
            }
        }

        let mut rng = rand::thread_rng();
        let i: usize = rng.gen::<usize>() % filtered.len();
        for (j, state) in filtered.iter().enumerate() {
            if i == j {
                return Some(state);
            }
        }

        None
    }

    pub fn search_state(&self, state_name: &str) -> Option<&MarkovState> {
        for state in &self.states {
            if state.token.word == *state_name {
                return Some(state);
            }
        }
        None
    }
}

impl MarkovState {
    fn from_chains(chains: Vec<MarkovChain>) -> Vec<MarkovState> {
        let mut states = Vec::new();
        for chain in chains {
            let counters = MarkovCounter::from_chain(&chain);
            let state_space = MarkovProbability::from_counters(counters);
            states.push(MarkovState {
                token: chain.token,
                state_space: state_space,
            });
        }

        states
    }

    pub fn get_random_token(&self) -> &LyrianToken {
        let mut probs: Vec<(&LyrianToken, f32)> = Vec::new();
        let mut sum = 0.0;
        for prob in &self.state_space {
            sum = prob.probability + sum;
            probs.push((&prob.token, sum));
        }

        let mut rng = rand::thread_rng();
        let f: f32 = rng.gen();
        for (token, p) in &probs {
            if f < *p {
                return token;
            }
        }

        let i = probs.len() - 1;
        probs[i].0
    }
}

impl MarkovProbability {
    fn from_counters(counters: Vec<MarkovCounter>) -> Vec<MarkovProbability> {
        let mut state_space = Vec::new();
        let sum = counters.iter().fold(0, |acc, cur| acc + cur.count);
        for counter in counters {
            state_space.push(MarkovProbability {
                token: counter.token,
                probability: counter.count as f32 / sum as f32,
            })
        }

        state_space
    }
}

impl MarkovCounter {
    fn from_chain(chain: &MarkovChain) -> Vec<MarkovCounter> {
        let mut counters = Vec::new();

        let mut non_dup_tokens = chain.chain_tokens.clone();
        non_dup_tokens.sort();
        non_dup_tokens.dedup();
        for token in non_dup_tokens {
            let count = chain.chain_tokens.iter().fold(0, |acc, cur| {
                if token.word == cur.word {
                    acc + 1
                } else {
                    acc
                }
            });
            counters.push(MarkovCounter {
                token: token,
                count: count,
            });
        }

        counters
    }
}

impl MarkovChain {
    fn from_tokens(tokens: Vec<LyrianToken>) -> Vec<MarkovChain> {
        let mut chains: Vec<MarkovChain> = Vec::new();

        let mut chain_tokens_index = 0;
        for (i, token) in tokens.iter().enumerate() {
            let j = match chains.iter().position(|c| c.token.word == token.word) {
                Some(v) => v,
                None => {
                    chains.push(MarkovChain {
                        token: token.clone(),
                        chain_tokens: Vec::new(),
                    });
                    chains.len() - 1
                }
            };

            if i != 0 {
                chains[chain_tokens_index].chain_tokens.push(token.clone());
            }

            chain_tokens_index = j;
        }

        for chain in &mut chains {
            if chain.chain_tokens.is_empty() {
                chain.chain_tokens.push(LyrianToken::empty_token());
            }
        }

        chains
    }
}
