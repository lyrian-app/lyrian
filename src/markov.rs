use crate::morphological_analysis::LyrianToken;
use std::mem;

pub struct MarkovState {
    pub token: LyrianToken,
    pub state_space: Vec<MarkovProbability>,
}

pub struct MarkovProbability {
    pub token: LyrianToken,
    pub probability: f32,
}

struct MarkovCounter {
    token: LyrianToken,
    count: i32,
}

struct MarkovChains {
    token: LyrianToken,
    chain_tokens: Vec<LyrianToken>,
}

pub struct Markov {
    pub tokens: Vec<LyrianToken>,
}

impl Markov {
    pub fn new(tokens: Vec<LyrianToken>) -> Markov {
        Markov { tokens: tokens }
    }

    pub fn tokens_to_model(self) -> Vec<MarkovState> {
        let chains = self.make_chains();
        let model = chains_to_model(chains);
        model
    }

    fn make_chains(self) -> Vec<MarkovChains> {
        let mut chains: Vec<MarkovChains> = Vec::new();
        let mut cloned = self.tokens.clone();

        let mut chain_tokens_index = 0;
        for i in 0..self.tokens.len() {
            let token = mem::replace(
                &mut cloned[i],
                LyrianToken {
                    word: String::from(""),
                    mora: String::from(""),
                    syllable: String::from(""),
                },
            );

            let i = match chains.iter().position(|c| c.token.word == token.word) {
                Some(v) => v,
                None => {
                    chains.push(MarkovChains {
                        token: token.clone(),
                        chain_tokens: Vec::new(),
                    });
                    chains.len() - 1
                }
            };

            if i != 0 {
                chains[chain_tokens_index].chain_tokens.push(token);
            }

            chain_tokens_index = i;
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

fn chains_to_model(chains: Vec<MarkovChains>) -> Vec<MarkovState> {
    let mut model = Vec::new();

    for chain in chains {
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

        let mut state = Vec::new();
        let sum = counters.iter().fold(0, |acc, cur| acc + cur.count);
        for counter in counters {
            state.push(MarkovProbability {
                token: counter.token,
                probability: counter.count as f32 / sum as f32,
            })
        }

        model.push(MarkovState {
            token: chain.token,
            state_space: state,
        })
    }

    model
}
