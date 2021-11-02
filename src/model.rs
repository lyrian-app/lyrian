use crate::markov::{Markov, MarkovState};
use crate::morphological_analysis::{tokenize, LyrianToken};

pub struct LyrianModel {
    pub token: LyrianToken,
    pub state_space: Vec<MarkovState>,
}

pub fn make_model(contents: &str) -> Result<Vec<LyrianModel>, String> {
    let tokens = tokenize(contents)?;
    let model_maker = Markov::new(tokens);
    let lyr_model = model_maker.tokens_to_model();
    Ok(lyr_model)
}
