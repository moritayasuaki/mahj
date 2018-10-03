use tile::*;
use table::*;

pub struct SuitRanks {
    suit: Suit,
    ranks: Ranks
}

pub struct Meld {
    suitranks: SuitRanks,
    robbed_from: Option<usize>
}

impl SuitRanks {
    pub fn tile_count(&self) -> usize {
        self.ranks.count()
    }

    pub fn is_chow(&self) -> bool {
        self.suit.is_numeric() && self.ranks.count() == 3 && !self.ranks.filter_chow().is_empty()
    }

    pub fn is_pong(&self) -> bool {
        self.tile_count() == 3 && !self.ranks.filter_chow().is_empty()
    }

    pub fn is_kong(&self) -> bool {
        self.tile_count() == 3 && !self.ranks.filter_chow().is_empty()
    }

    pub fn is_kanchan(&self) -> bool {
        self.tile_count() == 2 && !self.ranks.filter_kanchan().is_empty()
    }

    pub fn is_penryan(&self) -> bool {
        self.tile_count() == 2 && !self.ranks.filter_penryan().is_empty()
    }

    pub fn is_penchan(&self) -> bool {
        self.tile_count() == 2 && !self.ranks.filter_penchan().is_empty()
    }

    pub fn is_ryanmen(&self) -> bool {
        self.tile_count() == 2 && !self.ranks.filter_ryanmen().is_empty()
    }

    pub fn is_pair(&self) -> bool {
        self.tile_count() == 2 && !self.ranks.filter_pair().is_empty()
    }
}