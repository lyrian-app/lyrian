use crate::markov::{Markov, MarkovState};
use crate::morphological_analysis::tokenize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LyrianModel {
    pub markov_model: Vec<MarkovState>,
}

impl<'a> LyrianModel {
    fn new(markov_model: Vec<MarkovState>) -> LyrianModel {
        LyrianModel {
            markov_model: markov_model,
        }
    }

    pub fn from_str(contents: &str) -> Result<LyrianModel, String> {
        let tokens = tokenize(contents)?;
        let model_maker = Markov::new(tokens);
        let markov_model = model_maker.make_model();
        let lyr_model = LyrianModel::new(markov_model);
        Ok(lyr_model)
    }

    pub fn from_json(json: &'a str) -> Result<LyrianModel, String> {
        match serde_json::from_str::<'a, Vec<MarkovState>>(json) {
            Ok(markov_model) => {
                let lyr_model = LyrianModel::new(markov_model);
                Ok(lyr_model)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn generate_lyrics(
        &self,
        phrase_len: i32,
        first_word_len: i32,
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
        match serde_json::to_string(&self.markov_model) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }
    }
}
