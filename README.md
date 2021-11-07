# Lyrian

[![lyrian](https://github.com/LyrianBot/lyrian/actions/workflows/lyrian.yml/badge.svg)](https://github.com/LyrianBot/lyrian/actions/workflows/lyrian.yml)

A Rust crate to generate Japanese lyrics with Markov chain.

**Issue は日本語でも構いません。**

## Usage

### From text data

```rust
use std::fs::File;
use std::io::prelude::*;

use lyrian::model::LyrianModel;

fn main() {
    // Read learning data.
    let mut f = File::open("./examples/sample_text.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    // Build model from text data.
    let mut model = LyrianModel::from_str(&*contents).unwrap();

    // Generate lyric.
    let lyric_1 = model.generate_lyric(7, true).unwrap();
    let lyric_2 = model.generate_lyric(7, false).unwrap();
    println!("syllable: {}", lyric_1.join());
    println!("mora    : {}", lyric_2.join());

    // Write json file.
    // let mut f_json = File::create("./examples/sample_model2.json").unwrap();
    // f_json.write_all(model.to_json_str().unwrap().as_bytes()).unwrap();
}
```

### From json file

```rust
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
    let lyric_2 = model.generate_lyric(7, false).unwrap();
    println!("syllable: {}", lyric_1.join());
    println!("mora    : {}", lyric_2.join());
}
```

### Output

```
syllable: あるともう時間に
mora    : 読んだ僕は早くも
```

## Planned

- Support for voiceless sounds.
- Support for smoothly connected vowel sounds.
- Support for the feature to restrict parts of speech of the first word of a lyric.

## Resources

In the example code, we use Kenji Miyazawa's ["The Night of the Milky Way Train"](https://www.aozora.gr.jp/cards/000081/files/43737_19215.html) as a learning data.

## License

This crate is published under the [Mozilla Public License Version 2.0](LICENSE).
