use rand;
use table::*;
use failure;

type Res<T> = Result<T, failure::Error>;

struct Game {
    score: [usize; 4],
    riichi_sticks: usize,
    deposit_sticks: usize,
}

type Re = Res<()>;
const OK: Res<()> = Ok(());

impl Game {
    pub fn run_match(&mut self) -> Re {
        for prevailing_wind in 0..2 {
            self.run_round(prevailing_wind)?;
        }
        OK
    }
    pub fn run_half_match(&mut self) -> Re {
        for prevailing_wind in 0..2 {
            self.run_round(prevailing_wind)?;
        }
        OK
    }
    pub fn run_round(&mut self, prevailing_wind: Wind) -> Re {
        for dealer in 0..4 {
            self.run_hands(prevailing_wind, dealer)?;
        }
        OK
    }
    pub fn run_hands(&mut self, prevailing_wind: Wind, dealer: usize) -> Re{
        loop {
            let winner = self.run_hand(prevailing_wind, dealer)?;
            if winner != dealer {
                break;
            }
        }
        OK
    }
    pub fn run_hand(&mut self, prevailing_wind: Wind, dealer: usize) -> Res<usize> {
        let mut table = Table::new();
        table.shuffle_tiles();
        let dice: usize = rand::random();
        table.break_tiles(dice);
        for _ in 0..3 {
            for player in 0..4 {
                for _ in 0..4 {
                    let tile = table.wall.draw().expect("initial draw");
                    table.lands[player].tiles.add(tile);
                }
            }
        }
        for player in 0..4 {
            let tile = table.wall.draw().expect("initial draw");
            table.lands[player].tiles.add(tile);
        }
        Ok(0)
    }
}