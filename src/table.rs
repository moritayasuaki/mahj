use std;
use tile::*;
use meld::*;
use rand;
use std::mem;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct Wind(u8);


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
    pub fn from_tile_wind(tile: Tile, wind: Wind) -> Self {
        Self::from_raw((tile.id() << 8) | wind.id())
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

pub struct Rivers {
    pub tiles: [DiscardedTile; Tile::N],
    pub index: usize
}

impl Rivers {
    pub fn new() -> Self {
        Rivers {
            tiles: [DiscardedTile::from_raw(0usize); Tile::N],
            index: 0
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &DiscardedTile> {
        self.tiles.iter().take(self.index)
    }
    pub fn len(&self) -> usize {
        self.index
    }
    pub fn add(&mut self, wind: Wind, tile: Tile) {
        let i = self.index;
        self.index = i + 1;
        self.tiles[i] = DiscardedTile::from_tile_wind(tile, wind)
    }
    pub fn clear(&mut self) {
        self.index = 0
    }
    pub fn top(&mut self) -> Option<&mut DiscardedTile> {
        let i = self.index;
        if i != 0 {
            Some(&mut self.tiles[i-1])
        } else {
            None
        }
    }
}

pub struct Melds {
    index: usize,
    meld: [Meld; 16],
    tiles: Tiles
}

impl Melds {
    pub fn new() -> Self {
        Melds {
            index: 0,
            meld: [Meld::from_raw(0); 16],
            tiles: Tiles::new()
        }
    }
    pub fn clear(&mut self) {
        self.index = 0;
        self.tiles.clear();
    }
    pub fn iter(&self) -> impl Iterator<Item = &Meld> {
        self.meld.iter().take(self.index)
    }
    pub fn add(&mut self, meld: Meld) {
        let i = self.index;
        self.index += 1;
        self.meld[i] = meld;
    }
    pub fn top(&mut self) -> &mut Meld {
        &mut self.meld[self.index - 1]
    }
}

pub struct Lands {
    pub tiles: [Tiles; Wind::N],
    pub melds: Melds
}


impl Wind {
    pub const N: usize = 4;
    pub const EAST: Wind = Wind(0);
    pub const SOUTH: Wind = Wind(1);
    pub const WEST: Wind = Wind(2);
    pub const NORTH: Wind = Wind(3);
    pub fn id(self) -> usize {
        self.0 as usize
    }
    pub fn from_id(id: usize) -> Self {
        Wind((id % 4) as u8)
    }
    pub fn make_iter() -> impl Iterator<Item=Self> {
        (0..Self::N).map(Self::from_id)
    }
    pub fn others(self) -> impl Iterator<Item=Self> {
        let id = self.id();
        (id+1..id+4).map(Self::from_id)
    }
    pub fn nth(self, offset: usize) -> Self {
        Self::from_id(self.id() + offset)
    }
    pub fn rightside(self) -> Self {
        self.nth(1)
    }
    pub fn frontside(self) -> Self {
        self.nth(2)
    }
    pub fn leftside(self) -> Self {
        self.nth(3)
    }
    pub fn show(self) -> &'static str {
        match self {
            Self::EAST => "東",
            Self::SOUTH => "南",
            Self::WEST => "西",
            Self::NORTH => "北",
            _ => unreachable!()
        }
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
        self.wall.make_break(dice * 32 % Tile::N)
    }

    pub fn draw_tile(&mut self) -> Option<Tile> {
        self.wall.next_back()
    }
    pub fn draw_replacement(&mut self) -> Option<Tile> {
        self.wall.next()
    }
}

impl Wall {
    pub const N_DEAD_WALL: usize = 14;

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
        if a + b >= Tile::N {
            a + b - Tile::N
        } else {
            a + b
        }
    }

    fn sub(a: usize, b: usize) -> usize {
        if a >= b {
            a - b
        } else {
            Tile::N + a - b
        }
    }

    pub fn len(&self) -> usize {
        if self.index == self.ridge {
            Tile::N
        } else {
            Self::sub(self.index, self.ridge)
        }
    }

    pub fn next_back(&mut self) -> Option<Tile> {
        if self.len() > Self::N_DEAD_WALL {
            self.index = Self::sub(self.index, 1);
            let i = self.index;
            Some(self.tiles[i])
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<Tile> {
        if self.len() > Self::N_DEAD_WALL {
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

    pub fn make_break(&mut self, pos: usize) {
        self.index = pos;
        self.ridge = pos;
    }
}

impl Lands {
    pub fn new() -> Self {
        Lands {
            tiles: [Tiles::new(), Tiles::new(), Tiles::new(), Tiles::new()],
            melds: Melds::new(),
        }
    }
    pub fn clear(&mut self) {
        for tiles in self.tiles.iter_mut() {
            tiles.clear()
        }
        self.melds.clear()
    }
}