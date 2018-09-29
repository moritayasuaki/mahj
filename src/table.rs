use tile::*;
use meld::*;
use rand;
use mem;
use std;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct Wind(pub u8);

pub const EAST: Wind = Wind(0);

pub struct Table {
    pub wall: Wall,
    pub lands: Lands,
    pub rivers: Rivers
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

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct DiscardedTile(u16);
impl DiscardedTile {
    // [0..2]: discarded_by
    // [2..3]: riichi_flag
    // [3..5]: robbed_by
    // [5..6]: robbed_flag
    // [8..16]: tile
    fn raw(self) -> usize {
        self.0 as usize
    }
    pub fn from_raw(raw: usize) -> Self {
        DiscardedTile(raw as u16)
    }
    pub fn from_tile_seat(tile: Tile, seat: Wind) -> Self {
        Self::from_raw((tile.id() << 8) | seat.id())
    }
    pub fn discarded_by(self) -> Wind {
        Wind::from_id(self.raw() & 0o3)
    }
    pub fn add_riichi_flag(self) -> Self {
        Self::from_raw(self.raw() | 0o4)
    }
    pub fn is_riichi_declaration(self) -> bool {
        (self.raw() & 0o4) != 0
    }
    pub fn add_robbed_mark(self, robbed_by: Wind) -> Self {
        Self::from_raw(self.raw() | (robbed_by.id() << 3) | 0o40)
    }
    pub fn is_robbed(self) -> bool {
        (self.raw() & 0o40) != 0
    }
    pub fn robbed_by(self) -> Option<Wind> {
        if self.is_robbed() {
            Some(Wind::from_id((self.raw() >> 3) & 0o3))
        } else {
            None
        }
    }
    pub fn tile(self) -> Tile {
        Tile::from_id(self.raw() >> 8)
    }
}

pub struct Rivers2 {
    pub tiles: [DiscardedTile; 90],
    pub index: usize
}

impl Rivers2 {
    pub fn new() -> Self {
        Rivers2 {
            tiles: [DiscardedTile::from_raw(0usize); 90],
            index: 0
        }
    }
    pub fn get_vec(&self, seat: Wind) -> Vec<DiscardedTile> {
        self.tiles.iter().filter(|dtile| dtile.discarded_by() == seat).cloned().collect()
    }
    pub fn clear(&mut self) {
        self.index = 0
    }
}

pub struct River {
    pub tiles: Vec<Tile>,
    pub riichi: usize,
}

pub struct Lands([Land; Wind::N]);
pub struct Rivers([River; Wind::N]);


#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct RiverRef(u16);

impl Wind {
    pub const N: usize = 4;
    pub fn id(self) -> usize {
        self.0 as usize
    }
    pub fn from_id(id: usize) -> Self {
        Wind((id % 4) as u8)
    }
    pub fn make_iter() -> impl Iterator<Item=Self> {
        (0..Self::N).map(Self::from_id)
    }
    pub fn next(self) -> Self {
        Self::from_id(self.id() + 1)
    }
    pub fn prev(self) -> Self {
        Self::from_id(self.id() + 3)
    }
}

impl Table {
    pub fn new() -> Self {
        Table {
            wall : Wall::new(),
            lands : Lands::new(),
            rivers : Rivers::new()
        }
    }
    pub fn shuffle_tiles(&mut self) {
        self.lands.clear();
        self.rivers.clear();
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

impl Lands {
    pub fn new() -> Self {
        Lands([Land::new(), Land::new(), Land::new(), Land::new()])
    }
    pub fn get_mut(&mut self, seat: Wind) -> &mut Land {
        &mut (self.0)[seat.id()]
    }
    pub fn get(&self, seat: Wind) -> &Land {
        &(self.0)[seat.id()]
    }
    pub fn clear(&mut self) {
        self.0.iter_mut().for_each(Land::clear)
    }
}

impl Rivers {
    pub fn new() -> Self {
        Rivers([River::new(), River::new(), River::new(), River::new()])
    }
    pub fn get_mut(&mut self, seat: Wind) -> &mut River {
        &mut (self.0)[seat.id()]
    }
    pub fn get(&self, seat: Wind) -> &River {
        &(self.0)[seat.id()]
    }
    pub fn clear(&mut self) {
        self.0.iter_mut().for_each(River::clear)
    }
}

impl RiverRef {
    pub fn raw(self) -> usize {
        self.0 as usize
    }
    pub fn make(wind: usize, index: usize) -> RiverRef {
        RiverRef((wind + index * Wind::N) as u16)
    }
    pub fn index(self) -> usize {
        (self.raw() / Wind::N)
    }
    pub fn wind(self) -> Wind {
        Wind::from_id(self.raw())
    }
}

