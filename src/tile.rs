

#[must_use]
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Tile(u8);
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Figure(u8);
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Suit(u8);
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Rank(u8);
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct RankSpec(u8);

impl Tile {
    pub const N: usize = 136;
    pub fn id(&self) -> usize {
        self.0 as usize
    }
    pub fn from_id(id: usize) -> Self {
        Tile((id % Self::N) as u8)
    }
    pub fn mk_iter() -> impl Iterator<Item=Self> {
        (0..Self::N).map(Self::from_id)
    }
    pub fn suit(&self) -> Suit {
        self.figure().suit()
    }
    pub fn rank(&self) -> Rank {
        self.figure().rank()
    }
    pub fn figure(&self) -> Figure {
        Figure((self.id() / 4) as u8)
    }
    pub fn spec(&self) -> usize {
        (self.0 & 0x3) as usize
    }
    pub fn rank_spec(&self) -> RankSpec {
        RankSpec::from_rank_spec(self.rank(), self.spec())
    }
}

impl RankSpec {
    pub fn id(&self) -> usize {
        self.0 as usize
    }
    pub fn from_id(id: usize) -> Self {
        RankSpec(id as u8)
    }
    pub fn from_rank_spec(rank: Rank, spec: usize) -> Self {
        RankSpec::from_id((rank.id() << 2) | spec)
    }

}

impl Figure {
    pub const N: usize = 34;
    const STRS: [&'static str; Self::N] = [
        "🀇","🀈","🀉","🀊","🀋","🀌","🀍","🀎","🀏",
        "🀐","🀑","🀒","🀓","🀔","🀕","🀖","🀗","🀘",
        "🀙","🀚","🀛","🀜","🀝","🀞","🀟","🀠","🀡",
        "🀀","🀁","🀂","🀃","🀄","🀅", "🀆"];
    pub fn id(&self) -> usize {
        self.0 as usize
    }
    pub fn from_id(id: usize) -> Self {
        Figure((id % Self::N) as u8)
    }
    pub fn from_suitrank(s: Suit, r: Rank) -> Self {
        Self::from_id(s.id() * 9 + r.id())
    }
    pub fn mk_iter() -> impl Iterator<Item=Self> {
        (0..Self::N).map(Self::from_id)
    }
    pub fn suit(&self) -> Suit {
        Suit((self.id() / 9) as u8)
    }
    pub fn rank(&self) -> Rank {
        Rank((self.id() % 9) as u8)
    }
    pub fn show(&self) -> &'static str {
        Self::STRS[self.id()]
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::STRS.iter()
            .position(|t| t == &s)
            .map(Figure::from_id)
    }
}

#[test]
fn parse_test() {
    assert_eq!(Figure::parse("🀞"), Some(Figure::from_suitrank(Suit::CIRCLE, Rank::from_id(5))));
}

impl Suit {
    pub const N: usize = 4;
    pub const CHARA: Self = Suit(0);
    pub const BAMBOO: Self = Suit(1);
    pub const CIRCLE: Self = Suit(2);
    pub const HOUNOR: Self = Suit(3);
    pub fn id(&self) -> usize {
        self.0 as usize
    }
    pub fn from_id(id: usize) -> Self {
        Suit((id % Self::N) as u8)
    }
    pub fn mk_iter() -> impl Iterator<Item=Self> {
        (0..Self::N).map(Self::from_id)
    }
    pub fn is_horner(&self) -> bool {
        self.0 >= 3
    }
    pub fn is_numeric(&self) -> bool {
        self.0 < 3 
    }
}

impl Rank {
    pub const N: usize = 9;
    pub fn id(&self) -> usize {
        self.0 as usize
    }
    pub fn from_id(id: usize) -> Self {
        Rank((id % Self::N) as u8)
    }
    pub fn mk_iter() -> impl Iterator<Item=Self> {
        (0..Self::N).map(Self::from_id)
    }
}

#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd)]
pub struct Tiles([u64; (Tile::N - 1) / 64 + 1]);
#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd)]
pub struct Figures([Ranks; 4]);
#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd)]
pub struct Suits([u8; 4]);
#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd)]
pub struct Ranks(u32);
#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd)]
pub struct SuitRanks(u32);

impl Tiles {
    pub fn new() -> Self {
        Tiles([0; (Tile::N - 1) / 64 + 1])
    }
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
    pub fn count(&self) -> usize {
        self.0.iter().map(|&b| b.count_ones() as usize).sum()
    }
    pub fn clear(&mut self) {
        for t in self.0.iter_mut() {
            *t = 0;
        }
    }
    pub fn next(&mut self) -> Option<Tile> {
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
    pub fn add(&mut self, t: Tile) {
        let id = t.id();
        (self.0)[id / 64] |= 1 << (id % 64);
    }
    pub fn del(&mut self, t: Tile) {
        let id = t.id();
        (self.0)[id / 64] &= !(1 << (id % 64));
    }
    pub fn has(&self, t: Tile) -> bool {
        let id = t.id();
        ((self.0)[id / 64] & (1 << (id % 64))) != 0
    }
    pub fn extract_figure(&mut self, figure: Figure) -> Option<Tile> {
        let id = figure.id() * 4;
        let s = ((self.0)[id/64] >> (id % 64)) & 0xf;
        if s != 0 {
            let t = Tile::from_id(id + s.trailing_zeros() as usize);
            self.del(t);
            Some(t)
        } else {
            None
        }
    }
    pub fn extract_suitranks(&mut self, suitranks: SuitRanks) -> Vec<Tile> {
        let mut v = Vec::new();
        let mut fs = suitranks.to_figures();
        for f in fs {
            if let Some(t) = self.extract_figure(f) {
                v.push(t);
            } else {
                return Vec::new();
            }
        }
        v
    }
    pub fn figures(&mut self) -> Figures {
        let mut figures = Figures::new();
        while let Some(tile) = self.next() {
            figures.add(tile.figure());
        }
        figures
    }
 }

impl Figures {
    pub fn new() -> Self {
        Figures([Ranks::new(); 4])
    }
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
    pub fn count(&self) -> usize {
        self.0.iter().map(|r| r.count()).sum()
    }
    pub fn next(&mut self) -> Option<Figure> {
        self.0.iter_mut()
            .enumerate()
            .find_map(|(i, r)| {
                r.next().map(|r| Figure::from_suitrank(Suit::from_id(i), r))
            })
    }
    pub fn suitranks(&self, suit: Suit) -> SuitRanks {
        SuitRanks::from_suitranks(suit, (self.0)[suit.id()])
    }

    pub fn add(&mut self, figure: Figure) {
        (self.0)[figure.suit().id()].add(figure.rank())
    }

    pub fn has_chow(&self, rep: Figure) -> bool {
        (self.0)[rep.suit().id()].filter_chow().has(rep.rank())
    }
    pub fn has_pung(&self, rep: Figure) -> bool {
        (self.0)[rep.suit().id()].filter_pung().has(rep.rank())
    }
    pub fn has_kong(&self, rep: Figure) -> bool {
        (self.0)[rep.suit().id()].filter_kong().has(rep.rank())
    }
    pub fn has_pair(&self, rep: Figure) -> bool {
        (self.0)[rep.suit().id()].filter_pair().has(rep.rank())
    }
    pub fn has_one(&self, rep: Figure) -> bool {
        (self.0)[rep.suit().id()].filter_one().has(rep.rank())
    }
}

impl Ranks {
    pub fn new() -> Self {
        Ranks(0)
    }
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
    pub fn raw(self) -> usize {
        self.0 as usize
    }
    pub fn from_raw(raw: usize) -> Self {
        Ranks((raw & 0o777777777) as u32)
    }
    pub fn count(&self) -> usize {
        let r = self.0;
        let r = (r & 0o007007007) + ((r >> 3) & 0o007007007) + ((r >> 6) & 0o007007007);
        let r = (r & 0o000000777) + ((r >> 9) & 0o000000777) + ((r >> 18) & 0o000000777);
        r as usize
    }
    pub fn next(&mut self) -> Option<Rank> {
        let r = self.0;
        if r != 0 {
            let i = r.trailing_zeros() / 3;
            self.0 -= 1 << (3 * i);
            Some(Rank::from_id(i as usize))
        } else {
            None
        }
    }
    pub fn add(&mut self, rank: Rank) {
        self.0 += 1 << (3 * rank.id());
    }
    pub fn has(&self, rank: Rank) -> bool {
        self.0 & (0o7 << (3 * rank.id())) != 0
    }
    pub fn filter_pung(self) -> Self {
        let r = self.0;
        let r = (r + 0o111111111) >> 2;
        let r = r & 0o111111111;
        Ranks(r)
    }
    pub fn filter_kong(self) -> Self {
        let r = self.0;
        let r = r >> 2;
        let r = r & 0o111111111;
        Ranks(r)
    }
    pub fn filter_pair(self) -> Self {
        let r = self.0;
        let r = (r + 0o222222222) >> 2;
        let r = r & 0o111111111;
        Ranks(r)
    }
    pub fn filter_one(self) -> Self {
        let r = self.0;
        let r = (r + 0o333333333) >> 2;
        let r = r & 0o111111111;
        Ranks(r)
    }
    pub fn filter_chow(self) -> Self {
        let r = self.filter_one().0;
        let r = r & (r >> 3) & (r >> 6);
        Ranks(r)
    }
    pub fn filter_penryan(self) -> Self {
        let r = self.filter_one().0;
        let r = r & (r >> 3);
        Ranks(r)
    }
    pub fn filter_kanchan(self) -> Self {
        let r = self.filter_one().0;
        let r = r & (r >> 6);
        Ranks(r)
    }
    pub fn filter_penchan(self) -> Self {
        let r = self.filter_penryan().0;
        let r = r & 0o010000001;
        Ranks(r)
    }
    pub fn filter_ryanmen(self) -> Self {
        let r = self.filter_penryan().0;
        let r = r & 0o001111110;
        Ranks(r)
    }
}

impl Suits {
    pub fn new() -> Self {
        Suits([0; 4])
    }
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
    pub fn count(&self) -> usize {
        self.0.iter().map(|&c| c as usize).sum()
    }
    pub fn next(&mut self) -> Option<Suit> {
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

pub struct SuitTiles(u64);

impl SuitTiles {
    pub fn raw(self) -> usize {
        self.0 as usize
    }
    pub fn from_raw(raw: usize) -> Self {
        SuitTiles((raw & 0x3fffffffff) as u64)
    }
    pub fn new() -> Self {
        SuitTiles(0)
    }
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    pub fn suit(&self) -> Suit {
        Suit::from_id(((self.0 >> (4 * 9)) & 0x3) as usize)
    }
    pub fn singleton(tile: Tile) -> Self {
        let mut a = tile.suit().id() << (4 * 9);
        a |= 1 << (4 * tile.rank().id());
        SuitTiles::from_raw(a)
    }
}

impl SuitRanks {
    pub fn raw(self) -> usize {
        self.0 as usize
    }
    pub fn from_raw(raw: usize) -> Self {
        SuitRanks((raw & 0o3777777777) as u32)
    }
    pub fn from_suitranks(suit: Suit, ranks: Ranks) -> Self {
        SuitRanks::from_raw(suit.id() << (3 * 9) | ranks.raw())
    }
    pub fn into_suitranks(self) -> (Suit, Ranks) {
        (self.suit(), self.ranks())
    }
    pub fn ranks(self) -> Ranks {
        Ranks::from_raw(self.raw())
    }
    pub fn suit(self) -> Suit {
        Suit::from_id(self.raw() >> (3 * 9))
    }
    pub fn tile_count(&self) -> usize {
        self.ranks().count()
    }

    pub fn is_chow(&self) -> bool {
        self.suit().is_numeric() && self.ranks().count() == 3 && !self.ranks().filter_chow().is_empty()
    }

    pub fn is_pong(&self) -> bool {
        self.tile_count() == 3 && !self.ranks().filter_chow().is_empty()
    }

    pub fn is_kong(&self) -> bool {
        self.tile_count() == 3 && !self.ranks().filter_chow().is_empty()
    }

    pub fn is_kanchan(&self) -> bool {
        self.tile_count() == 2 && !self.ranks().filter_kanchan().is_empty()
    }

    pub fn is_penryan(&self) -> bool {
        self.tile_count() == 2 && !self.ranks().filter_penryan().is_empty()
    }

    pub fn is_penchan(&self) -> bool {
        self.tile_count() == 2 && !self.ranks().filter_penchan().is_empty()
    }

    pub fn is_ryanmen(&self) -> bool {
        self.tile_count() == 2 && !self.ranks().filter_ryanmen().is_empty()
    }

    pub fn is_pair(&self) -> bool {
        self.tile_count() == 2 && !self.ranks().filter_pair().is_empty()
    }

    pub fn to_figures(&self) -> Vec<Figure> {
        let mut v = Vec::new();
        let mut ranks = self.ranks();
        while Some(rank) = ranks.next() {
            v.push(Figure::from_suitrank(self.suit(), rank))
        }
        v
    }
}