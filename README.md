# Lyrian

A Rust crate to generate Japanese lyrics with Markov chain.

## Usage

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

    // Generate lyric by syllable.
    let lyric = model.generate_lyric(7, true).unwrap();
    println!("syllable: {}", lyric.join());

    // Convert model to json.
    let json = model.to_json_str().unwrap();

    // Build model from json.
    let mut model = LyrianModel::from_json(&*json).unwrap();

    // Generate lyric by mora.
    let lyric = model.generate_lyric(10, false).unwrap();
    println!("mora    : {}", lyric.join());
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
