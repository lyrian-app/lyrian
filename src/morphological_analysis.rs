use lindera::tokenizer::Tokenizer;
use serde::{Deserialize, Serialize};
use std::mem;

use crate::chars::*;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialOrd, PartialEq)]
pub struct LyrianToken {
    pub word: String,
    pub mora: String,
    pub syllable: String,
}

impl LyrianToken {
    pub fn new(word: String, mora: String, syllable: String) -> LyrianToken {
        LyrianToken {
            word: word,
            mora: mora,
            syllable: syllable,
        }
    }

    pub fn empty_token() -> LyrianToken {
        LyrianToken {
            word: String::from(""),
            mora: String::from(""),
            syllable: String::from(""),
        }
    }

    pub fn length(&self, rhythmical: bool) -> usize {
        if rhythmical {
            return self.syllable_len();
        }
        self.mora_len()
    }

    fn mora_len(&self) -> usize {
        if self.mora == String::from("unknown") {
            return 0;
        }
        self.mora.chars().count()
    }

    fn syllable_len(&self) -> usize {
        if self.syllable == String::from("unknown") {
            return 0;
        }

        let mut length = self.syllable.chars().count();

        for ch1 in SKIPPED_CHARS {
            for ch2 in self.syllable.chars() {
                if ch1 == ch2 {
                    length -= 1;
                }
            }
        }

        // TODO: Processing of voiceless sound
        // TODO: Processing to join vowel

        length
    }
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

        lyr_tokens.push(LyrianToken::new(
            token.text.to_string(),
            mem::replace(&mut detail[0], String::from("")),
            mem::replace(&mut detail[1], String::from("")),
        ));
    }

    Ok(lyr_tokens)
}

#[cfg(test)]
mod morphological_analysis_test {
    use crate::morphological_analysis::{tokenize, LyrianToken};

    #[test]
    fn get_lyrian_tokens_from_text() {
        let text = "すもももももももものうち";
        match tokenize(text) {
            Ok(tokens) => {
                let expected = vec![
                    LyrianToken::new(
                        "すもも".to_string(),
                        "スモモ".to_string(),
                        "スモモ".to_string(),
                    ),
                    LyrianToken::new("も".to_string(), "モ".to_string(), "モ".to_string()),
                    LyrianToken::new("もも".to_string(), "モモ".to_string(), "モモ".to_string()),
                    LyrianToken::new("も".to_string(), "モ".to_string(), "モ".to_string()),
                    LyrianToken::new("もも".to_string(), "モモ".to_string(), "モモ".to_string()),
                    LyrianToken::new("の".to_string(), "ノ".to_string(), "ノ".to_string()),
                    LyrianToken::new("うち".to_string(), "ウチ".to_string(), "ウチ".to_string()),
                ];
                assert_eq!(tokens, expected)
            }
            Err(msg) => panic!("{}", msg),
        }
    }

    #[test]
    fn get_word_length_on_mora() {
        let token = LyrianToken::new(
            "大空".to_string(),
            "オオゾラ".to_string(),
            "オーゾラ".to_string(),
        );
        assert_eq!(token.length(false, false, false), 4)
    }

    #[test]
    fn get_word_length_on_syllable() {
        let token = LyrianToken::new(
            "大空".to_string(),
            "オオゾラ".to_string(),
            "オーゾラ".to_string(),
        );
        assert_eq!(token.length(true, false, false), 3)
    }

    #[test]
    #[ignore]
    fn get_word_length_that_has_voiceless_sound() {
        let token = LyrianToken::new("桜".to_string(), "サクラ".to_string(), "サクラ".to_string());
        assert_eq!(token.length(false, true, false), 2)
    }

    #[test]
    #[ignore]
    fn get_word_length_that_has_smooth_vowel_sound() {
        let token = LyrianToken::new(
            "赤井".to_string(),
            "アカイ".to_string(),
            "アカイ".to_string(),
        );
        assert_eq!(token.length(false, false, true), 2)
    }
}
