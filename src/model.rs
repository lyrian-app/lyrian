use crate::markov::{Markov, MarkovState};
use crate::morphological_analysis::tokenize;

pub struct LyrianModel {
    pub markov_model: Vec<MarkovState>,
}

pub fn make_model(contents: &str) -> Result<LyrianModel, String> {
    let tokens = tokenize(contents)?;
    let model_maker = Markov::new(tokens);
    let markov_model = model_maker.make_model();
    Ok(LyrianModel::new(markov_model))
}
