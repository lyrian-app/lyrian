//! # walkers_alias_method
//!
//! A module to create the alias table by Walker's Alias Method.
//! In Lyrian, it is used for weighted random sampling.
//!
//! ## Example
//!
//! ```rust
//! let fruit = ["Apple", "Banana", "Orange", "Peach"]
//!
//! // The weights of the output indexes.
//! // The higher the weight, the more likely the corresponding index will be
//! // output.
//! // In the following cases, the output probabilities for each index are 0.2,
//! // 0.1, 0.7 and 0. If a weight value is 0, the index will not be output.
//! // In other words, the index 3 will not be output in this cases.
//! let index_weights = vec![2, 1, 7, 0];
//!
//! let mut builder = WalkerTableBuilder::new(index_weights);
//! let wa_table = builder.build();
//!
//! let i = wa_table.next();
//! println!("{}", fruit[i]);
//! ```
//!

pub mod builder;
pub mod table;
