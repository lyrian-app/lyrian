//! A module related to the information of the generated lyric.

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

    /// Adds a new [`LyrianToken`] to [`Lyric`].
    pub fn add_token(&mut self, token: LyrianToken) {
        self.tokens.push(token);
    }

    /// Join the words of [`Vec<LyrianToken>`].
    pub fn join(&self) -> String {
        self.tokens
            .iter()
            .fold(String::from(""), |acc, cur| format!("{}{}", acc, cur.word))
    }
}
