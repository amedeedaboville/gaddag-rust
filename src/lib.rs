#![allow(dead_code)]

#[macro_use]
extern crate itertools;
pub mod bag;
pub mod board;
pub mod dictionary;
pub mod game;
//pub mod play;
pub mod player;
pub mod puzzle;
pub mod simulate;
#[cfg(feature = "with_std")]
pub mod text;
#[macro_use]
pub mod utils;
