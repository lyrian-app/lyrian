use crate::morphological_analysis::LyrianToken;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::mem;

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkovModel {
    pub states: Vec<MarkovState>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarkovState {
    pub token: LyrianToken,
    pub state_space: Vec<MarkovProbability>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarkovProbability {
    pub token: LyrianToken,
    pub probability: f32,
}

struct MarkovCounter {
    token: LyrianToken,
    count: i32,
}

#[derive(Clone)]
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

    pub fn get_random_state(&self, word_len: usize, rhythmical: bool) -> Option<MarkovState> {
        let mut filtered = self
            .states
            .iter()
            .filter(|state| state.token.length(rhythmical) == word_len)
            .collect();

        if filtered.len() == 0 {
            return None;
        }

        let mut rng = rand::thread_rng();
        let i = (rng.next_u32() % filtered.len()) as usize;

        let state = mem::replace(
            &mut filtered[i],
            MarkovState {
                token: LyrianToken::empty_token(),
                state_space: Vec::new(),
            },
        );

        Some(state)
    }
}

impl MarkovState {
    fn from_chains(chains: Vec<MarkovChain>) -> Vec<MarkovState> {
        let mut states = Vec::new();
        for chain in chains {
            let token = chain.clone().token;
            let counters = MarkovCounter::from_chain(chain);
            let state_space = MarkovProbability::from_counters(counters);
            states.push(MarkovState {
                token: token,
                state_space: state_space,
            });
        }

        states
    }

    pub fn get_random_token(&self) -> LyrianToken {
        let mut probs = Vec::new();
        let mut sum = 0.0;
        for prob in self.state_space {
            sum = prob.probability + sum;
            probs.push((prob.token, sum));
        }

        let mut rng = rand::thread_rng();
        let f: f32 = rng.gen();
        for (token, p) in probs {
            if f < p {
                return token;
            }
        }

        mem::replace(&mut probs[probs.len() - 1].0, LyrianToken::empty_token())
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
    fn from_chain(chain: MarkovChain) -> Vec<MarkovCounter> {
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
        let mut cloned = tokens.clone();

        let mut chain_tokens_index = 0;
        for i in 0..tokens.len() {
            let token = mem::replace(&mut cloned[i], LyrianToken::empty_token());

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
                chains[chain_tokens_index].chain_tokens.push(token);
            }

            chain_tokens_index = j;
        }

        for i in 0..chains.len() {
            if chains[i].chain_tokens.is_empty() {
                chains[i].chain_tokens.push(LyrianToken::empty_token());
            }
        }

        chains
    }
}
