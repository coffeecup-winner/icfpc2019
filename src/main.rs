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

use std::env;
use std::fs::File;
use std::io::prelude::*;

use crate::problem::Problem;

fn main() {
    let mut file = File::open(env::args().into_iter().nth(1).unwrap()).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let state = Problem::parse(&data);
    println!("Max points: {}", state.max_points());
}
