use lindera::tokenizer::Tokenizer;
use lindera_core::LinderaResult;

pub struct LyrianToken {
    surface: String,
    part_of_speech: [String; 4],
    infl_type: String,
    infl_form: String,
    base_form: String,
    reading: String,
    phonetic: String,
}
