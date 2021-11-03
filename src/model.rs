use crate::markov::{Markov, MarkovState};
use crate::morphological_analysis::tokenize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LyrianModel {
    pub markov_model: Vec<MarkovState>,
}

impl LyrianModel {
    fn new(markov_model: Vec<MarkovState>) -> LyrianModel {
        LyrianModel {
            markov_model: markov_model,
        }
    }

    pub fn to_json_str(self) -> Result<String, String> {
        match serde_json::to_string(&self.markov_model) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub fn make_model(contents: &str) -> Result<LyrianModel, String> {
    let tokens = tokenize(contents)?;
    let model_maker = Markov::new(tokens);
    let markov_model = model_maker.make_model();
    Ok(LyrianModel::new(markov_model))
}
