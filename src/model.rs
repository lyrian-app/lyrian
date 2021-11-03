use crate::markov::MarkovModel;
use crate::morphological_analysis::tokenize;
use serde::{Deserialize, Serialize};
use std::mem;

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
            let first_token = &self.markov.get_random_state(first_word_len, rhythmical);
            let chained_tokens = Vec::new();

            for _ in 0..64 {
                chained_tokens.push(first_token.get_random_token(rhythmical));
                let generated_len = {
                    let first_token_len = first_token.token.length(rhythmical);
                    let chained_tokens_len = chained_tokens
                        .iter()
                        .fold(0, |acc, cur| acc + cur.length(rhythmical));
                    first_token_len + chained_tokens_len
                };

                if lyric_len < generated_len {
                    mem::drop(chained_tokens);
                }

                if lyric_len == generated_len {
                    let chained_words = chained_tokens.iter().fold("", |acc, cur| acc + cur.word);
                    return Ok(first_token.token.word + chained_words);
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
