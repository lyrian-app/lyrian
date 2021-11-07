//! A module related to the information of the generated lyric.

use crate::chars::{dup_num, SYMBOLS};
use crate::morphological_analysis::LyrianToken;

/// Lyric structure
pub struct Lyric {
    tokens: Vec<LyrianToken>,
}

impl Lyric {
    /// Creates a new instance of [`Lyric`].
    pub fn new(tokens: Vec<LyrianToken>) -> Lyric {
        Lyric { tokens: tokens }
    }

    /// Returns the length of the generated lyric.
    pub fn length(&self, syllable: bool) -> usize {
        self.tokens
            .iter()
            .fold(0, |acc, cur| acc + cur.length(syllable))
    }

    /// Adds a new token to [`Lyric`].
    pub fn add_token(&mut self, token: LyrianToken) {
        self.tokens.push(token);
    }

    /// Join the words of `tokens`.
    pub fn join(&self) -> String {
        self.tokens.iter().fold(String::from(""), |acc, cur| {
            let count = dup_num(&cur.mora.chars().collect(), &SYMBOLS.to_vec());
            if count == 0 {
                format!("{}{}", acc, cur.word)
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod lyric_test {
    use crate::lyric::Lyric;

    #[test]
    fn join_words_of_tokens() {
        use crate::morphological_analysis::LyrianToken;

        let tokens = vec![
            LyrianToken::new(
                "大きな".to_string(),
                "オオキナ".to_string(),
                "オーキナ".to_string(),
            ),
            LyrianToken::new("、".to_string(), "、".to_string(), "、".to_string()),
            LyrianToken::new("空".to_string(), "ソラ".to_string(), "ソラ".to_string()),
        ];
        let lyric = Lyric::new(tokens);
        assert_eq!(lyric.join(), "大きな空".to_string())
    }
}
