use lindera::tokenizer::Tokenizer;
use std::mem;

pub struct LyrianToken {
    pub surface: String,
    pub part_of_speech: [String; 4],
    pub infl_type: String,
    pub infl_form: String,
    pub base_form: String,
    pub reading: String,
    pub phonetic: String,
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
            token.detail
        } else {
            vec!["unknown".to_string(); 9]
        };

        lyr_tokens.push(LyrianToken {
            surface: token.text.to_string(),
            part_of_speech: [
                mem::replace(&mut detail[0], String::from("")),
                mem::replace(&mut detail[1], String::from("")),
                mem::replace(&mut detail[2], String::from("")),
                mem::replace(&mut detail[3], String::from("")),
            ],
            infl_type: mem::replace(&mut detail[4], String::from("")),
            infl_form: mem::replace(&mut detail[5], String::from("")),
            base_form: mem::replace(&mut detail[6], String::from("")),
            reading: mem::replace(&mut detail[7], String::from("")),
            phonetic: mem::replace(&mut detail[8], String::from("")),
        });
    }

    Ok(lyr_tokens)
}
