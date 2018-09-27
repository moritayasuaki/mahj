use tile::*;
use meld::*;
use rand;
use mem;

pub struct Table {
    wall: Wall,
    land: [Land; 4],
    river: [River; 4]
}

pub struct Wall {
    tiles: [Tile; Tile::N],
    index: usize,
    dead_index: usize
}

pub struct Land {
    tiles: Tiles,
    melds: Vec<Meld>
}

pub struct River {
    tiles: Vec<Tile>,
    riich: usize,
}

impl Wall {
    pub const N_DEAD_WALL: usize = Tile::N;

    pub fn new() -> Self {
        let mut w = Wall {
            tiles: [Tile::from_id(0); Tile::N],
            index: 0,
            dead_index: 0,
        };
        for i in 0..Tile::N {
            w.tiles[i] = Tile::from_id(i);
        }
        w
    }

    fn add(a: usize, b: usize) -> usize {
        (a + b) % Tile::N
    }

    fn sub(a: usize, b: usize) -> usize {
        if a >= b {
            a - b
        } else {
            Tile::N + a - b
        }
    }

    pub fn rest(&self) -> usize {
        Self::sub(
            self.index, Self::add(
                self.dead_index, Self::N_DEAD_WALL))
    }

    pub fn next_back(&mut self) -> Option<Tile> {
        if self.rest() > 0 {
            self.index = Self::sub(self.index, 1);
            Some(self.tiles[self.index])
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<Tile> {
        if self.rest() > 0 {
            let i = self.index;
            self.index = Self::add(self.index, 1);
            Some(self.tiles[i])
        } else {
            None
        }
    }

    pub fn shuffle(&mut self) {
        for i in 0..Tile::N{
            let j = rand::random::<usize>() % (1 + i);
            if i != j {
                self.tiles.swap(i, j)
            }
        }
    }
}

impl Land {
    pub fn new() -> Self {
        Land {
            tiles: Tiles::new(),
            melds: Vec::new()
        }
    }
    pub fn add(&mut self, tile: Tile) {
        self.tiles.add(tile)
    }
    pub fn del(&mut self, tile: Tile) {
        self.tiles.del(tile)
    }
    pub fn extract(&mut self, figure: Figure) -> Option<Tile> {
        self.tiles.extract(figure)
    }
}

impl River {
    pub fn add(&mut self, tile: Tile) {
        self.tiles.push(tile)
    }
    pub fn reset(&mut self) {
        self.tiles = Vec::new();
    }
}

pub struct RiverRef(u16);

impl RiverRef {
    pub fn make(wind: usize, index: usize) -> RiverRef {
        RiverRef((wind + index * 4) as u16)
    }
    pub fn index(&self) -> usize {
        (self.0 / 4) as usize
    }
    pub fn wind(&self) -> usize {
        (self.0 % 4) as usize
    }
}