//! # Lyrian
//!
//! A Rust crate to generate Japanese lyrics with Markov chain.
//!
//! ## Usage
//!
//! ```rust
//! use std::fs::File;
//! use std::io::prelude::*;
//!
//! use lyrian::model::LyrianModel;
//!
//! fn main() {
//!     // Read learning data.
//!     let mut f = File::open("./examples/sample_text.txt").unwrap();
//!     let mut contents = String::new();
//!     f.read_to_string(&mut contents).unwrap();
//!
//!     // Build model from text data.
//!     let mut model = LyrianModel::from_str(&*contents).unwrap();
//!
//!     // Generate lyric by syllable.
//!     let lyric = model.generate_lyric(7, true).unwrap();
//!     println!("syllable: {}", lyric.join());
//!
//!     // Convert model to json.
//!     let json = model.to_json_str().unwrap();
//!
//!     // Build model from json.
//!     let mut model = LyrianModel::from_json(&*json).unwrap();
//!
//!     // Generate lyric by mora.
//!     let lyric = model.generate_lyric(10, false).unwrap();
//!     println!("mora    : {}", lyric.join());
//! }
//! ```
//!
//! ### Output
//!
//! ```md
//! syllable: あるともう時間に
//! mora    : 読んだ僕は早くも
//! ```
//!

pub mod lyric;
pub mod model;

mod chars;
mod markov;
mod morphological_analysis;
mod walkers_alias_method;
