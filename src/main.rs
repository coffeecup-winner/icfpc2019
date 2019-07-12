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
use std::path::Path;

use crate::core::State;
use crate::problem::Problem;
use crate::terminal::{Colorizable, TerminalColor};

fn solve(path: &Path) -> std::io::Result<()> {
    let data = std::fs::read_to_string(path)?;
    let state = Problem::parse(&data);
    println!("{}", format!("Solving {} ({})", path.display(), state.info()).colorize(TerminalColor::Magenta));
    Ok(())
}

fn main() -> std::io::Result<()> {
    solve(Path::new(&env::args().into_iter().nth(1).unwrap()))
}
