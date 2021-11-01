use crate::markov::MarkovState;

pub struct LyrianModel {
    word: String,
    mora: String,
    syllable: String,
    state_space: Vec<MarkovState>,
}
