use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Read learning data.
    let mut f = File::open("./examples/sample_text.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    // Make model from text data.
    let mut model = lyrian::model::LyrianModel::from_str(&*contents).unwrap();

    // Generate rhythmical lyric.
    // If the argument "rhythmical" is true, the lyric will be generated on a syllable unit.
    // Also, the voiceless sound will be taken into account.
    let lyric = model.generate_lyric(7, true).unwrap();
    println!("{}", lyric.join());

    // Convert model to json.
    let json = model.to_json_str().unwrap();

    // Make model from json.
    let mut model = lyrian::model::LyrianModel::from_json(&*json).unwrap();

    // Generate unrhythmical lyric.
    // If the argument "rhythmical" is false, the lyric will be generated on a mora unit.
    let lyric = model.generate_lyric(10, false).unwrap();
    println!("{}", lyric.join());
}
