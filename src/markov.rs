use crate::morphological_analysis::LyrianToken;
use serde::{Deserialize, Serialize};
use std::mem;

pub struct Markov {
    pub tokens: Vec<LyrianToken>,
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

impl Markov {
    pub fn new(tokens: Vec<LyrianToken>) -> Markov {
        Markov { tokens: tokens }
    }

    pub fn make_model(self) -> Vec<MarkovState> {
        let chains = MarkovChain::from_tokens(self.tokens);
        let model = MarkovState::from_chains(chains);
        model
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
            let token = mem::replace(
                &mut cloned[i],
                LyrianToken {
                    word: String::from(""),
                    mora: String::from(""),
                    syllable: String::from(""),
                },
            );

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
                chains[i].chain_tokens.push(LyrianToken {
                    word: String::from("。"),
                    mora: String::from(""),
                    syllable: String::from(""),
                });
            }
        }

        chains
    }
}
