use tile::*;
use table::*;
pub type SuitRanks = (Suit, Ranks);
pub enum Meld {
    Concealed{suitranks: SuitRanks},
    Robbed{suitranks: SuitRanks, refr: RefRever},
}

pub struct M {

}