use crate::lyric::Lyric;
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

    pub fn generate_lyric(&mut self, lyric_len: usize, rhythmical: bool) -> Result<Lyric, String> {
        for _ in 0..64 {
            let mut lyric = Lyric::new(Vec::new());
            for _ in 0..64 {
                lyric.add_token(self.markov.next().clone());
                if lyric_len < lyric.length(rhythmical) {
                    break;
                } else if lyric_len == lyric.length(rhythmical) {
                    return Ok(lyric);
                }
            }
            self.markov.initialize();
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
