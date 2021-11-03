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

    pub fn generate_lyrics(
        &self,
        phrase_len: usize,
        first_word_len: usize,
        rhythmical: bool,
    ) -> Result<String, String> {
        if phrase_len < first_word_len {
            return Err(String::from(
                "The phrase length must be longer than the first word length.",
            ));
        }

        Ok(String::from(""))
    }

    pub fn to_json_str(&self) -> Result<String, String> {
        match serde_json::to_string(&self.markov) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }
    }
}
