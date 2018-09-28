use table::*;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
struct Wind(u8);

impl Wind {
    fn from_index(i: usize) -> Self{
        Wind((i % 4) as u8)
    }
    fn make_iter(iter: impl Iterator<Item=usize>) -> impl Iterator<Item=Wind> {
        iter.map(Self::from_index)
    }
}

struct Game {
    score: [usize; 4],
    riichi_sticks: usize,
    deposit_sticks: usize,
}

impl Game {
    pub fn run_match(&mut self) {
        for prevailing_wind in Wind::make_iter(0..2) {
            self.run_round(prevailing_wind);
        }
    }
    pub fn run_half_match(&mut self) {
        for prevailing_wind in Wind::make_iter(0..2) {
            self.run_round(prevailing_wind);
        }
    }
    pub fn run_round(&mut self, prevailing_wind: Wind) {
        for dealer in 0..4 {
            self.run_hands(prevailing_wind, dealer);
        }
    }
    pub fn run_hands(&mut self, prevailing_wind: Wind, dealer: usize) {
        loop {
            let winner = self.run_hand(prevailing_wind, dealer);
            if winner != dealer {
                break;
            }
        }
    }
    pub fn run_hand(&mut self, prevailing_wind: Wind, dealer: usize) -> usize {
        let mut table = Table::new();
        table.shuffle_tiles();
        // table.stack_tiles();
        // table.draw_tiles();
        0
    }
}