use tile::*;
use meld::*;
use rand;
use mem;
use std;

pub type Wind = u8;

pub struct Table {
    pub wall: Wall,
    pub lands: [Land; 4],
    pub rivers: [River; 4]
}

pub struct Wall {
    pub tiles: [Tile; Tile::N],
    pub index: usize,
    pub ridge: usize
}

pub struct Land {
    pub tiles: Tiles,
    pub melds: Vec<Meld>
}

pub struct River {
    pub tiles: Vec<Tile>,
    pub riichi: usize,
}

impl Table {
    pub fn new() -> Self {
        Table {
            wall : Wall::new(),
            lands : [Land::new(), Land::new(), Land::new(), Land::new()],
            rivers : [River::new(), River::new(), River::new(), River::new()]
        }
    }
    pub fn shuffle_tiles(&mut self) {
        for i in 0..4 {
            self.lands[i].clear();
            self.rivers[i].clear();
        }
        self.wall.shuffle();
    }
    pub fn break_tiles(&mut self, dice: usize) {
        self.wall.ridge = dice * 32;
    }
}

impl Wall {
    pub const N_DEAD_WALL: usize = Tile::N;

    pub fn new() -> Self {
        let mut w = Wall {
            tiles: [Tile::from_id(0); Tile::N],
            index: 0,
            ridge: 0,
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
                self.ridge, Self::N_DEAD_WALL))
    }

    pub fn draw(&mut self) -> Option<Tile> {
        if self.rest() > 0 {
            self.index = Self::sub(self.index, 1);
            Some(self.tiles[self.index])
        } else {
            None
        }
    }

    pub fn draw_ridge(&mut self) -> Option<Tile> {
        if self.rest() > 0 {
            let i = self.ridge;
            self.ridge = Self::add(self.ridge, 1);
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

    pub fn breaking(&mut self, pos: usize) {
        self.index = pos;
        self.ridge = pos;
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
    pub fn clear(&mut self) {
        self.tiles.clear()
    }
}

impl River {
    pub fn new() -> Self {
        River {
            tiles: Vec::new(),
            riichi: std::usize::MAX,
        }
    }
    pub fn add(&mut self, tile: Tile) {
        self.tiles.push(tile)
    }
    pub fn clear(&mut self) {
        self.tiles.clear();
        self.riichi = std::usize::MAX;
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