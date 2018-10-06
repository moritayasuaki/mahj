use tile::*;
use table::*;
use meld::*;
use action::*;
use player::*;
use dice::*;
use failure;
use std;
use std::io::{BufRead, Write};


pub struct Sticks {
    pub score: [isize; 4],
    pub burst_flags: u8,
    pub deposit: isize,
    pub stack: isize
}

impl Sticks {
    pub fn new() -> Self {
        Sticks {
            score: [25000; 4],
            burst_flags: 0,
            deposit: 0,
            stack: 0
        }
    }
    pub fn payment(&mut self, payer: usize, receiver: usize, points: isize) {
        self.score[payer] -=  points;
        self.score[receiver] += points;
    }
    pub fn is_bursted(&self) -> bool {
        self.burst_flags != 0
    }
}

pub struct State<'a> {
    table: &'a mut Table,
    sticks: &'a mut Sticks,
    round: Wind,
    dice: Dice,
    dealer: usize,
    players: &'a mut [Player; 4]
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Phase {
    Draw(Wind),
    Choose(Wind, Tile),
    Replace(Wind),
    Ask(),
    Meld(Wind, Claim),
    Discard(Wind),
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Finish {
    WinByDraw(Wind, Tile),
    WinByDiscard(Wind),
    ExaustiveDraw,
    FourRiichiAbort,
    NineTerminalAbort,
    FourWindAbort,
    ThreeWinAbort,
    FourKongAbort,   
}

pub struct Players([Player; 4]);

impl Players {
    pub fn new() -> Self {
        Players([Player::new(), Player::new(), Player::new(), Player::new()])
    }
    pub fn run_match(&mut self) -> Result<[isize; 4], failure::Error> {
        let sticks = &mut Sticks::new();
        for round in Wind::make_iter() {
            self.run_round(sticks, round)?;
            if sticks.is_bursted() {
                break;
            }
        }
        Ok(sticks.score)
    }

    pub fn run_halfmatch(&mut self) -> Result<[isize; 4], failure::Error> {
        let sticks = &mut Sticks::new();
        for round in Wind::make_iter().take(Wind::N/2) {
            self.run_round(sticks, round)?;
            if sticks.is_bursted() {
                break;
            }
        }
        Ok(sticks.score)
    }

    pub fn run_eastmatch(&mut self) -> Result<[isize; 4], failure::Error> {
        let sticks = &mut Sticks::new();
        let round = Wind::from_id(0);
        self.run_round(sticks, round)?;
        Ok(sticks.score)
    }

    pub fn run_round(&mut self, sticks: &mut Sticks, round: Wind) -> Result<(), failure::Error> {
        for dealer in 0..4 {
            self.run_hands(sticks, round, dealer)?;
            if sticks.is_bursted() {
                break;
            }
        }
        Ok(())
    }

    pub fn run_hands(&mut self, sticks: &mut Sticks, round: Wind, dealer: usize) -> Result<(), failure::Error> {
        let table = &mut Table::new();
        loop {
            let bonus_hand = self.run_hand(sticks, round, dealer, table)?;
            if sticks.is_bursted() {
                break;
            }
            if !bonus_hand {
                break;
            }
        }
        Ok(())
    }
    pub fn run_hand(&mut self, sticks: &mut Sticks, round: Wind, dealer: usize, table: &mut Table) -> Result<bool, failure::Error> {
        let dice = init_table(table);
        let mut phase = Phase::new();
        let players = &mut self.0;
        let state = &mut State {
            dice,
            sticks,
            round,
            dealer,
            table,
            players,
        };
        let finish = loop {
            match phase.step(state)? {
                Step::Phase(next_phase) => phase = next_phase,
                Step::Finish(finish) => break finish,
            }
        };
        finish.payment(state);
        Ok(finish.has_bonus_hand())
    }
}

pub fn init_table(table: &mut Table) -> Dice {
    table.shuffle_tiles();
    let dice = Dice::new();
    table.break_tiles(dice.sum());

    let mut draw = |id: usize| {
        let tile = table.draw_tile().expect("drawing for building table");
        table.lands.tiles[id].add(tile);
    };
 
    for _ in 0..3 {
        for wind in Wind::make_iter() {
            for _ in 0..4 {
                draw(wind.id())
            }
        }
    }
    for wind in Wind::make_iter() {
        draw(wind.id())
    }
    dice
}


impl Finish {
    pub fn is_goulash_hand(self) -> bool {
        match self {
            Finish::ExaustiveDraw | Finish::FourRiichiAbort |
            Finish::NineTerminalAbort | Finish::FourWindAbort |
            Finish::ThreeWinAbort | Finish::FourKongAbort => true,
            _ => false
        }
    }
    pub fn is_dealer_win(self) -> bool {
        use self::Finish::*;
        match self {
            WinByDraw(Wind::EAST, ..) | WinByDiscard(Wind::EAST, ..) => true,
            _ => false
        }
    }
    pub fn has_bonus_hand(self) -> bool {
        self.is_goulash_hand() || self.is_dealer_win()
    }
    pub fn payment(self, state: &mut State) {
        let _ = state;
    }
}

pub enum Step {
    Phase(Phase),
    Finish(Finish)
}

impl Step {
    pub fn phase(phase: Phase) -> Self {
        Step::Phase(phase)
    }
    pub fn finish(finish: Finish) -> Self {
        Step::Finish(finish)
    }
}


impl From<Phase> for Result<Step, failure::Error> {
    fn from(phase: Phase) -> Self {
        Ok(Step::Phase(phase))
    }
}

impl From<Finish> for Result<Step, failure::Error> {
    fn from(finish: Finish) -> Self {
        Ok(Step::Finish(finish))
    }
}
pub struct Seat<'a> {
    pub wind: Wind,
    pub land: &'a mut Tiles,
    pub river: &'a mut Rivers,
    pub wall: &'a mut Wall,
    pub meld: &'a mut Melds,
    pub player: &'a mut Player
}

impl<'a> Seat<'a> {
    pub fn show_draw_phase(&mut self, drawn: Tile) -> String {
        let mut s = String::new();
        let mut tiles = self.land.clone();
        while let Some(tile) = tiles.next() {
            s.push_str(&format!("{}", tile.figure().show()));
        }
        s.push_str(&format!(" {}", drawn.figure().show()));
        s
    }
    pub fn take_tile(&mut self, tile: Tile) {
        self.land.add(tile)
    }
    pub fn discard_figure(&mut self, figure: Figure) -> bool {
        if let Some(tile) = self.land.extract(figure) {
            self.put_river(tile);
            true
        } else {
            false
        }
    }

    pub fn discard_tile(&mut self, tile: Tile) -> bool {
        if self.land.has(tile) {
            self.land.del(tile);
            self.put_river(tile);
            true
        } else {
            false
        }
    }
    pub fn put_river(&mut self, tile: Tile) {
        self.river.add(self.wind, tile)
    }
    pub fn get_claim(&mut self) -> Claim {
        self.player.get_line()
            .and_then(|line| Claim::parse(&line))
            .unwrap_or_default()
    }
    pub fn get_choice(&mut self) -> Choice {
        self.player.get_line()
            .and_then(|line| Choice::parse(&line))
            .unwrap_or_default()
    }

    pub fn do_choice(&mut self, choice: Choice, tile: Tile) -> Result<Step, failure::Error> {
        match choice {
            Choice::Discard{figure, riichi} => {
                if self.discard_figure(figure) {
                    self.take_tile(tile);
                    Phase::Ask().into()
                } else {
                    Err(failure::err_msg(format!("Can not discard {}", figure.show())))
                }
            },
            Choice::DrawAndDiscard{riichi} => {
                self.discard_tile(tile);
                Phase::Ask().into()
            },
            Choice::Kong(fig) => {
                unimplemented!()
            },
            Choice::Mahjong => {
                unimplemented!()
            },
            _ => unimplemented!()
        }
    }

}

impl<'a> State<'a> {
    pub fn seat(&mut self, wind: Wind) -> Seat {
        let pid = self.pid(wind);
        Seat {
            wind,
            land: &mut self.table.lands.tiles[wind.id()],
            river: &mut self.table.rivers,
            wall: &mut self.table.wall,
            meld: &mut self.table.lands.melds,
            player: &mut self.players[pid],
        }
    }
    pub fn pid(&self, wind: Wind) -> usize {
        (self.dealer + wind.id()) % 4
    }
    pub fn draw(&mut self, seat: Wind) -> Result<Step, failure::Error> {
        if let Some(tile) = self.table.draw_tile() {
            self.choose(seat, tile).into()
        } else {
            Finish::ExaustiveDraw.into()
        }
    }
    pub fn replace(&mut self, seat: Wind) -> Result<Step, failure::Error> {
        if let Some(tile) = self.table.draw_replacement() {
            self.choose(seat, tile).into()
        } else {
            Finish::ExaustiveDraw.into()
        }
    }
    pub fn meld(&mut self, seat: Wind, claim: Claim) -> Result<Step, failure::Error> {
        let _ = (seat, claim);
        unimplemented!()
    }
    pub fn choose(&mut self, turn: Wind, tile: Tile) -> Result<Step, failure::Error> {
        let seat = &mut self.seat(turn);
        println!("{}", seat.show_draw_phase(tile));
        let choice = seat.get_choice();
        seat.do_choice(choice, tile)
    }
    pub fn ask(&mut self) -> Result<Step, failure::Error> {
        let claimee = self.table.rivers.current_mut().expect("Tiles not found on river").discarded_by();
        let mut claims = Claims::new(claimee);
        for claimer in claimee.others() {
            let seat = &mut self.seat(claimer);
            let claim = seat.get_claim();
            claims.add(claim, claimer);
        }
        if let Some((claim, claimer)) = claims.next() {
            match claim {
                Claim::THROUGH => Phase::Draw(claimee.rightside()).into(),
                Claim::CHOW | Claim::PUNG | Claim::KONG => Phase::Meld(claimer, claim).into(),
                Claim::MAHJONG => Finish::WinByDiscard(claimer).into(),
                _ => unreachable!()
            }
        } else {
            Phase::Draw(claimee.rightside()).into()
        }
    }
    pub fn discard(&mut self, seat: Wind) -> Result<Step, failure::Error> {
        Phase::Ask().into()
    }
}

impl Phase {
    pub fn new() -> Phase {
        Phase::Draw(Wind::EAST)
    }

    pub fn step<'a>(self, state: &mut State<'a>) -> Result<Step, failure::Error> {
        use self::Phase::*;
        match self {
            Draw(seat) => state.draw(seat),
            Choose(seat, tile) => state.choose(seat, tile),
            Replace(seat) => state.replace(seat),
            Meld(seat, claim) => state.meld(seat, claim),
            Ask() => state.ask(),
            Discard(seat) => state.discard(seat),
        }
    }
}


