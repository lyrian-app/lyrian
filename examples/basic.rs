use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("./examples/sample_text.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let model = lyrian::make_model(&*contents).unwrap();
    println!("{}", model.generate_lyrics(5, true).unwrap());
    println!("{}", model.to_json().unwrap());
}
