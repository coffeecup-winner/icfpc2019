#![feature(associated_type_defaults)]
#![feature(vec_remove_item)]
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod core;
mod geometry;
mod grid;
mod problem;
mod robot;
mod state;
mod terminal;

use std::env;

use crate::problem::Problem;
use crate::terminal::{Colorizable, TerminalColor};

fn main() {
    let data = std::fs::read_to_string(env::args().into_iter().nth(1).unwrap()).unwrap();
    let state = Problem::parse(&data);
    println!("{}", format!("Max points: {}", state.max_points()).colorize(TerminalColor::Yellow));
}
