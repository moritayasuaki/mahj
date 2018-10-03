use table::*;
use meld::*;
use tile::*;
use std::ops::Deref;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Choice {
    Discard(Figure),
    Riichi(Figure),
    Kong(Figure),
    Through,
    ThroughRiichi,
    NineTerminals,
    Mahjong
}

impl Choice {
    pub fn parse(s: &str) -> Result<Self, failure::Error> {
        let mut tokens = s.split_whitespace();
        fn figure_arg<'a>(f: fn(Figure) -> Choice, mut tokens: impl Iterator<Item=&'a str>) -> Result<Choice, failure::Error> {
            let expr = tokens.next().ok_or(failure::err_msg("No argument"))?;
            let fig = Figure::parse(expr).ok_or(failure::err_msg("Parse error"))?;
            Ok(f(fig))
        };
        if let Some(t) = tokens.next() {
            match t {
                "NineTerminals" => Ok(Choice::NineTerminals),
                "Mohjong" => Ok(Choice::Mahjong),
                "Discard" => figure_arg(Choice::Discard, tokens),
                "Riichi" => figure_arg(Choice::Riichi, tokens),
                "Kong" =>  figure_arg(Choice::Kong, tokens),
                "Through" => Ok(Choice::Through),
                "ThroughRiichi" => Ok(Choice::ThroughRiichi),
                command => Err(failure::err_msg(format!("No such command: {}", command)))
            }
        } else {
            Ok(Choice::Through)
        }
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct Claim(u8);

impl Claim {
    pub const N: usize = 4;
    pub const MAHJONG: Self = Claim(0);
    pub const KONG: Self = Claim(1);
    pub const PUNG: Self = Claim(2);
    pub const CHOW: Self = Claim(3);
    pub const THROUGH: Self = Claim(4);
    pub fn from_id(id: usize) -> Self {
        Claim((id % Self::N) as u8)
    }
    pub fn id(self) -> usize {
        self.0 as usize
    }
}

impl Claim {
    pub fn parse(s: &str) -> Result<Self, failure::Error> {
        let mut tokens = s.split_whitespace();
        if let Some(t) = tokens.next() {
            match t {
                "Mahjong" => Ok(Claim::MAHJONG),
                "Kong" => Ok(Claim::KONG),
                "Pung" => Ok(Claim::PUNG),
                "Chow" => Ok(Claim::CHOW),
                "Through" => Ok(Claim::THROUGH),
                claim =>  Err(failure::err_msg(format!("invalid claim {}", claim))),
            }
        } else {
            Ok(Claim::THROUGH)
        }
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct ClaimBy {
    pub claim: Claim,
    pub nth: usize
}

pub struct Claims(u16);

impl Claims {
    pub fn new() -> Self {
        Claims(0)
    }
    pub fn collect<'a>(ss: impl Iterator<Item = impl Deref<Target=str>>) -> Result<Self, failure::Error> {
        let mut claims = Self::new();
        for (nth, s) in ss.enumerate() {
            if let Ok(claim) = Claim::parse(&s) {
                claims.add(ClaimBy {nth, claim})
            }
        }
        Ok(claims)
    }
    pub fn add(&mut self, claimby: ClaimBy) {
        let ClaimBy{claim, nth} = claimby;
        self.0 |= 0o1 << (3 * claim.id() + nth);
    }
    pub fn empty(&self) -> bool {
        self.0 == 0
    }
    pub fn next(&mut self) -> Option<ClaimBy> {
        if !self.empty() {
            let t = self.0;
            let i = t.trailing_zeros() as usize;
            self.0 = t & (t-1);
            let claim = Claim::from_id(i / 3);
            let nth = i % 3;
            Some(ClaimBy{claim, nth})
        } else {
            None
        }
    }
}
