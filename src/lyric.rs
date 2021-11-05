use crate::morphological_analysis::LyrianToken;

#[derive(Debug)]
pub struct Lyric {
    tokens: Vec<LyrianToken>,
}

impl Lyric {
    pub fn new(tokens: Vec<LyrianToken>) -> Lyric {
        Lyric { tokens: tokens }
    }

    pub fn length(&self, syllable: bool, voiceless: bool, smooth: bool) -> usize {
        self.tokens
            .iter()
            .fold(0, |acc, cur| acc + cur.length(syllable, voiceless, smooth))
    }

    pub fn add_token(&mut self, token: LyrianToken) {
        self.tokens.push(token);
    }

    pub fn join(&self) -> String {
        self.tokens
            .iter()
            .fold(String::from(""), |acc, cur| format!("{}{}", acc, cur.word))
    }
}
