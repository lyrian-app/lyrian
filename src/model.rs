use crate::markov::MarkovState;
use crate::morphological_analysis::tokenize;

pub struct LyrianModel {
    pub word: String,
    pub mora: String,
    pub syllable: String,
    pub state_space: Vec<MarkovState>,
}

pub fn make_model(contents: &str) -> Result<Vec<LyrianModel>, String> {}
