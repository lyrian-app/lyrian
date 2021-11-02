use lindera::tokenizer::Tokenizer;
use std::mem;

#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq)]
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

    pub fn mora_len(self) -> usize {
        if self.mora == String::from("unknown") {
            return 0;
        }
        self.mora.chars().count()
    }

    pub fn syllable_len(self) -> usize {
        if self.syllable == String::from("unknown") {
            return 0;
        }

        let mut cloned = self.syllable.clone();
        let targets = [
            'ー', '～', 'ン', 'ッ', 'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ャ', 'ュ', 'ョ', '「', '」',
            '。', '、', '!', '！', '?', '？', '"', '#', '$', '%', '&', '\'', '(', ')', '（', '）',
            '-', '=', '＝', '^', '＾', '|', '\\', '｜', '￥', '@', '`', '[', ']', '{', '}', '｛',
            '｝', ';', '；', ':', '：', '+', '＋', '*', '＊', '<', '＜', '>', '＞', '_',
        ];
        cloned.retain(|c| !targets.iter().any(|target| c == *target));

        // TODO: Processing of voiceless sound
        // TODO: Processing to join vowel

        cloned.chars().count()
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
