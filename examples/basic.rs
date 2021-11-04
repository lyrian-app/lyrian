use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("./examples/sample_text.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let mut model = lyrian::model::LyrianModel::from_str(&*contents).unwrap();

    println!("{:?}", model.generate_lyric(7, true).unwrap());
}
