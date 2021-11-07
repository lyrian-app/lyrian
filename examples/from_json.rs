use std::fs::File;
use std::io::prelude::*;

use lyrian::model::LyrianModel;

fn main() {
    // Read json file.
    let mut f = File::open("./examples/sample_model.json").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    // Build model from json.
    let mut model = LyrianModel::from_json(&*contents).unwrap();

    // Generate lyric.
    let lyric_1 = model.generate_lyric(7, true).unwrap();
    let lyric_2 = model.generate_lyric(10, false).unwrap();
    println!("syllable: {}", lyric_1.join());
    println!("mora    : {}", lyric_2.join());
}
