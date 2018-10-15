extern crate rand;
extern crate failure;

mod tile;
mod meld;
mod table;
mod game;
mod action;
mod player;
mod dice;

fn main() -> Result<(), failure::Error> {
    let mut players = game::Players([
        player::Player::from_stdio()?,
        player::Player::from_stdio()?,
        player::Player::from_stdio()?,
        player::Player::from_stdio()?]);
    let score = players.run_halfmatch()?;
    println!("{:?}", score);
    Ok(())
}