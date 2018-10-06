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
    let mut players = game::Players::new();
    let score = players.run_halfmatch()?;
    println!("{:?}", score);
    Ok(())
}

#[test]
fn test_main() -> Result<(), failure::Error> {
    main()
}