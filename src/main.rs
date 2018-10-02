extern crate rand;
extern crate failure;

mod tile;
mod meld;
mod table;
mod game;
mod action;


fn main() -> Result<(), failure::Error> {
    let score = game::run_halfmatch()?;
    println!("{:?}", score);
    Ok(())
}