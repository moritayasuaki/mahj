use table::*;
use meld::*;
use tile::*;
use std::ops::Deref;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Choice {
    DrawAndDiscard{riichi: bool},
    Discard{figure:Figure, riichi: bool},
    Kong(Figure),
    NineTerminals,
    Mahjong
}

impl Default for Choice {
    fn default() -> Self {
        Choice::DrawAndDiscard{riichi: false}
    }
}

impl Choice {
    pub fn parse(s: &str) -> Result<Self, failure::Error> {
        let mut tokens = s.split_whitespace();
        if let Some(t) = tokens.next() {
            match t {
                "NineTerminals" => Ok(Choice::NineTerminals),
                "Mohjong" => Ok(Choice::Mahjong),
                "Discard" | "Riichi" => {
                    let riichi = t == "Riichi";
                    if let Some(expr) = tokens.next() {
                        Figure::parse(expr).ok_or(failure::err_msg("Parse error"))
                            .map(|figure| Choice::Discard{figure, riichi})
                    } else {
                        Ok(Choice::DrawAndDiscard{riichi})
                    }
                },
                "Kong" => {
                    if let Some(expr) = tokens.next() {
                        Figure::parse(expr).ok_or(failure::err_msg("Parse error"))
                            .map(|figure| Choice::Kong(figure))
                    } else {
                        Err(failure::err_msg("No arguments"))
                    }
                }
                command => Err(failure::err_msg(format!("No such command: {}", command)))
            }
        } else {
            Err(failure::err_msg(format!("No command")))
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

impl Default for Claim {
    fn default() -> Self {
        Claim::THROUGH
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
                claim =>  Err(failure::err_msg(format!("Invalid claim {}", claim))),
            }
        } else {
            Err(failure::err_msg("No token"))
        }
    }
}

pub struct Claims {
    claims: u16,
    claimee: Wind
}

impl Claims {
    pub fn new(claimee: Wind) -> Self {
        Claims {
            claims: 0,
            claimee
        }
    }

    pub fn add(&mut self, claim: Claim, claimer: Wind) {
        assert_ne!(self.claimee, claimer);
        let nth = (claimer.id() + 3 - self.claimee.id()) % 4;
        if claim != Claim::THROUGH {
            self.claims |= 0o1 << (3 * claim.id() + nth);
        }
    }
    pub fn is_empty(&self) -> bool {
        self.claims == 0
    }
    pub fn next(&mut self) -> Option<(Claim, Wind)> {
        if !self.is_empty() {
            let t = self.claims;
            let i = t.trailing_zeros() as usize;
            self.claims = t & (t-1);
            let claim = Claim::from_id(i / 3);
            let nth = i % 3;
            let claimer = Wind::from_id(nth + 1);
            Some((claim, claimer))
        } else {
            None
        }
    }
}
