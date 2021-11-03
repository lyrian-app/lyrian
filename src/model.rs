use crate::markov::MarkovModel;
use crate::morphological_analysis::tokenize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LyrianModel {
    pub markov: MarkovModel,
}

impl<'a> LyrianModel {
    fn new(markov_model: MarkovModel) -> LyrianModel {
        LyrianModel {
            markov: markov_model,
        }
    }

    pub fn from_str(contents: &str) -> Result<LyrianModel, String> {
        let tokens = tokenize(contents)?;
        let markov_model = MarkovModel::from_tokens(tokens);
        let lyr_model = LyrianModel::new(markov_model);
        Ok(lyr_model)
    }

    pub fn from_json(json: &'a str) -> Result<LyrianModel, String> {
        match serde_json::from_str::<'a, MarkovModel>(json) {
            Ok(markov_model) => {
                let lyr_model = LyrianModel::new(markov_model);
                Ok(lyr_model)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn generate_lyric(
        &self,
        lyric_len: usize,
        first_word_len: usize,
        rhythmical: bool,
    ) -> Result<String, String> {
        if lyric_len < first_word_len {
            return Err(String::from(
                "The phrase length must be longer than the first word length.",
            ));
        }

        for _ in 0..64 {
            let first_state_opt = &self.markov.get_random_state(first_word_len, rhythmical);
            match first_state_opt {
                Some(state) => {
                    let mut chained_tokens = Vec::new();
                    let mut current_state = state.clone();

                    for _ in 0..64 {
                        let next_token = current_state.get_random_token();
                        current_state = self.markov.search_state(next_token.clone().word).unwrap();
                        chained_tokens.push(next_token);

                        let generated_len = {
                            let chained_tokens_len = chained_tokens
                                .iter()
                                .fold(0, |acc, cur| acc + cur.length(rhythmical));
                            first_word_len + chained_tokens_len
                        };

                        if lyric_len < generated_len {
                            chained_tokens.retain(|_| false);
                        }

                        if lyric_len == generated_len {
                            let chained_words = chained_tokens
                                .iter()
                                .fold(String::from(""), |acc, cur| format!("{}{}", acc, cur.word));
                            return Ok(format!("{}{}", state.token.word, chained_words));
                        }
                    }
                }
                None => {
                    return Err(String::from(
                        "Could not find a word with the same length as first_word_len.",
                    ))
                }
            }
        }

        Err(String::from(
            "Could not generate a lyric in given arguments.",
        ))
    }

    pub fn to_json_str(&self) -> Result<String, String> {
        match serde_json::to_string(&self.markov) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }
    }
}
