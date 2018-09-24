

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
struct Tile(u8);
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
struct Figure(u8);
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
struct Suit(u8);
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
struct Rank(u8);

impl Tile {
    const N: usize = 136;
    fn id(&self) -> usize {
        self.0 as usize
    }
    fn from_id(id: usize) -> Self {
        Tile((id % Self::N) as u8)
    }
    fn mk_iter() -> impl Iterator<Item=Self> {
        (0..Self::N).map(Self::from_id)
    }
    fn figure(&self) -> Figure {
        Figure((self.id() / 4) as u8)
    }
}

impl Figure {
    const N: usize = 34;
    fn id(&self) -> usize {
        self.0 as usize
    }
    fn from_id(id: usize) -> Self {
        Figure((id % Self::N) as u8)
    }
    fn from_suitrank(s: Suit, r: Rank) -> Self {
        Self::from_id(s.id() * 9 + r.id())
    }
    fn mk_iter() -> impl Iterator<Item=Self> {
        (0..Self::N).map(Self::from_id)
    }
    fn suit(&self) -> Suit {
        Suit((self.id() / 9) as u8)
    }
    fn rank(&self) -> Suit {
        Suit((self.id() % 9) as u8)
    }
}

impl Suit {
    const N: usize = 4;
    fn id(&self) -> usize {
        self.0 as usize
    }
    fn from_id(id: usize) -> Self {
        Suit((id % Self::N) as u8)
    }
    fn mk_iter() -> impl Iterator<Item=Self> {
        (0..Self::N).map(Self::from_id)
    }
}

impl Rank {
    const N: usize = 9;
    fn id(&self) -> usize {
        self.0 as usize
    }
    fn from_id(id: usize) -> Self {
        Rank((id % Self::N) as u8)
    }
    fn mk_iter() -> impl Iterator<Item=Self> {
        (0..Self::N).map(Self::from_id)
    }
}

#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd)]
struct Tiles([u64; (Tile::N - 1) / 64 + 1]);
#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd)]
struct Figures([Ranks; 4]);
#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd)]
struct Suits([u8; 4]);
#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd)]
struct Ranks(u32);

impl Tiles {
    fn new() -> Self {
        Tiles([0; (Tile::N - 1) / 64 + 1])
    }
    fn is_empty(&self) -> bool {
        self.count() == 0
    }
    fn count(&self) -> usize {
        self.0.iter().map(|&b| b.count_ones() as usize).sum()
    }
    fn next(&mut self) -> Option<Tile> {
        self.0.iter_mut()
            .enumerate()
            .find_map(|(i, b)| if *b != 0 {
                let t = *b;
                *b &= t - 1;
                Some(Tile::from_id(i * 64 + t.trailing_zeros() as usize))
            } else {
                None
            })
    }
    fn add(&mut self, t: Tile) {
        let id = t.id();
        (self.0)[id / 64] |= 1 << (id % 64);
    }
    fn del(&mut self, t: Tile) {
        let id = t.id();
        (self.0)[id / 64] &= !(1 << (id % 64));
    }
    fn extract(&mut self, f: Figure) -> Option<Tile> {
        let id = f.id() * 4;
        let s = ((self.0)[id/64] >> (id % 64)) & 0xf;
        if s != 0 {
            let t = Tile::from_id(id + s.trailing_zeros() as usize);
            self.del(t);
            Some(t)
        } else {
            None
        }
    }
 }

impl Figures {
    fn new() -> Self {
        Figures([Ranks::new(); 4])
    }
    fn is_empty(&self) -> bool {
        self.count() == 0
    }
    fn count(&self) -> usize {
        self.0.iter().map(|r| r.count()).sum()
    }
    fn next(&mut self) -> Option<Figure> {
        self.0.iter_mut()
            .enumerate()
            .find_map(|(i, r)| {
                r.next().map(|r| Figure::from_suitrank(Suit::from_id(i), r))
            })
    }
}

impl Ranks {
    fn new() -> Self {
        Ranks(0)
    }
    fn is_empty(&self) -> bool {
        self.count() == 0
    }
    fn count(&self) -> usize {
        let r = self.0;
        let r = (r & 0o007007007) + ((r >> 3) & 0o007007007) + ((r >> 6) & 0o007007007);
        let r = (r & 0o000000777) + ((r >> 9) & 0o000000777) + ((r >> 18) & 0o000000777);
        r as usize
    }
    fn next(&mut self) -> Option<Rank> {
        let r = self.0;
        if r != 0 {
            let i = r.trailing_zeros() / 3;
            self.0 -= 1 << (3 * i);
            Some(Rank::from_id(i as usize))
        } else {
            None
        }
    }
    fn filter_pung(self) -> Self {
        let r = self.0;
        let r = (r + 0o111111111) >> 2;
        let r = r & 0o111111111;
        Ranks(r)
    }
    fn filter_kong(self) -> Self {
        let r = self.0;
        let r = r >> 2;
        let r = r & 0o111111111;
        Ranks(r)
    }
    fn filter_eye(self) -> Self {
        let r = self.0;
        let r = (r + 0o222222222) >> 2;
        let r = r & 0o111111111;
        Ranks(r)
    }
    fn filter_one(self) -> Self {
        let r = self.0;
        let r = (r + 0o333333333) >> 2;
        let r = r & 0o111111111;
        Ranks(r)
    }
    fn filter_chow(self) -> Self {
        let r = self.filter_one().0;
        let r = r & (r >> 3) & (r >> 6);
        Ranks(r)
    }
    fn filter_penchan(self) -> Self {
        let r = self.filter_one().0;
        let r = r & (r >> 3);
        Ranks(r)
    }
    fn filter_kanchan(self) -> Self {
        let r = self.filter_one().0;
        let r = r & (r >> 6);
        Ranks(r)
    }
}

impl Suits {
    fn new() -> Self {
        Suits([0; 4])
    }
    fn is_empty(&self) -> bool {
        self.count() == 0
    }
    fn count(&self) -> usize {
        self.0.iter().map(|&c| c as usize).sum()
    }
    fn next(&mut self) -> Option<Suit> {
        self.0.iter_mut()
            .enumerate()
            .find_map(|(i, c)| if *c != 0 {
                *c -= 1;
                Some(Suit::from_id(i))
            } else {
                None
            })
    }
}