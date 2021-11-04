use crate::morphological_analysis::LyrianToken;

#[derive(Debug)]
pub struct Lyric {
    tokens: Vec<LyrianToken>,
}

impl Lyric {
    pub fn new(tokens: Vec<LyrianToken>) -> Lyric {
        Lyric { tokens: tokens }
    }

    pub fn length(&self, rhythmical: bool) -> usize {
        self.tokens
            .iter()
            .fold(0, |acc, cur| acc + cur.length(rhythmical))
    }

    pub fn add_token(&mut self, token: LyrianToken) {
        self.tokens.push(token);
    }
}
