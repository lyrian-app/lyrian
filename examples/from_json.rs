use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("./examples/sample_model.json").unwrap();
    let mut json = String::new();
    f.read_to_string(&mut json).unwrap();

    let model = lyrian::model::LyrianModel::from_json(&*json).unwrap();

    println!("{}", model.generate_lyrics(5, 3, true).unwrap());
}
