use lindera::tokenizer::Tokenizer;
use std::mem;

#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq)]
pub struct LyrianToken {
    pub word: String,
    pub mora: String,
    pub syllable: String,
}

pub fn tokenize(contents: &str) -> Result<Vec<LyrianToken>, String> {
    let mut tokenizer;
    let lin_tokens;

    match Tokenizer::new() {
        Ok(v) => tokenizer = v,
        Err(e) => return Err(e.to_string()),
    }

    match tokenizer.tokenize(&*contents) {
        Ok(v) => lin_tokens = v,
        Err(e) => return Err(e.to_string()),
    }

    let mut lyr_tokens = Vec::new();
    for token in lin_tokens {
        let mut detail = if token.detail.len() != 1 {
            token.detail.split_at(7).1.to_vec()
        } else {
            vec![String::from("unknown"); 2]
        };

        lyr_tokens.push(LyrianToken {
            word: token.text.to_string(),
            mora: mem::replace(&mut detail[0], String::from("")),
            syllable: mem::replace(&mut detail[1], String::from("")),
        });
    }

    Ok(lyr_tokens)
}
