use tile::*;
use meld::*;

struct Wall {
    tiles : [Tile; Tile::N],
    idx : usize,
    dead : usize
}

struct Land {
    tiles : Tiles,
    melds : Vec<Meld>
}

struct River {
    tiles : Vec<Tile>,
    riich : usize,
}