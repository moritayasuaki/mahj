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
    Draw{wind: Wind},
    Choose{wind: Wind, drawn: Tile},
    Replace{wind: Wind, expose: bool},
    Ask{kong: bool},
    Meld{wind: Wind, claim: Claim},
    Discard{wind: Wind},
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

pub struct Players(pub [Player; 4]);

impl Players {
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
    pub melds: &'a mut Melds,
    pub player: &'a mut Player
}

impl<'a> Seat<'a> {
    pub fn show_draw_phase(&mut self, drawn: Tile) {
        let mut tiles = self.land.clone();
        let player = &mut self.player;
        writeln!(player, "{}家 ツモ番", self.wind.show());
        while let Some(tile) = tiles.next() {
            write!(player, "{}", tile.figure().show());
        }
        writeln!(player, " {}", drawn.figure().show());
    }

    pub fn show_claim_phase(&mut self) {
        let mut s = String::new();
        let player = &mut self.player;
        let s = self.wind;
        let river = &mut self.river;
        writeln!(player, "{}家 鳴き番", s.show());
        for o in s.others() {
            write!(player, "{}家河", o.show());
            river.iter().filter(|d| d.discarded_by() == o).for_each(|d| {
                write!(player, "{}", d.tile().figure().show());
            });
            writeln!(player, "");
        }
        write!(player, "{}家河", s.show());
        river.iter().filter(|d| d.discarded_by() == s).for_each(|d| {
            write!(player, "{}", d.tile().figure().show());
        });
        writeln!(player, "");
        let mut tiles = self.land.clone();
        while let Some(tile) = tiles.next() {
            write!(player, "{}", tile.figure().show());
        }
        writeln!(player, "");
    }

    pub fn take_tile_into_hand(&mut self, tile: Tile) {
        self.land.add(tile)
    }
    pub fn remove_tile_from_hand(&mut self, tile: Tile) {
        self.land.del(tile)
    }
    pub fn has_meld(&mut self) {

    }
    pub fn extract_tile_from_hand(&mut self, figure: Figure) -> Option<Tile> {
        self.land.extract(figure)
    }
    pub fn throw_tile_into_river(&mut self, tile: Tile) {
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
    pub fn has_kong_concealed(&self, figure: Figure) -> bool {
        self.land.clone().figures().has_kong(figure)
    }
    pub fn has_pung_exposed(&self, figure: Figure) -> bool {
        self.melds.iter().find(|meld|
            (meld.wind(self.river) == Some(self.wind)) && (meld.set().shape() == Shape::PUNG) && (meld.set().figure() == figure)).is_some()
    }
    pub fn has_figure_concealed(&self, figure: Figure) -> bool {
        self.land.clone().figures().has_one(figure)
    }
    pub fn look_river(&self) -> Tile {
        self.river.last().expect("no river").tile()
    }
    pub fn rob_tile(&mut self) -> Option<(Tile, usize)> {
        let i = self.river.index;
        let d = self.river.last_mut()?;
        d.add_robbed_mark(self.wind);
        Some((d.tile(), i))
    }
    pub fn do_choice(&mut self, choice: Choice, tile: Tile) -> Result<Step, failure::Error> {
        match choice {
            Choice::Discard{figure, riichi} => {
                let discard = self.extract_tile_from_hand(figure)
                    .ok_or(failure::err_msg(format!("Can not discard {}", figure.show())))?;
                self.take_tile_into_hand(tile);
                self.throw_tile_into_river(discard);
                Phase::Ask{kong: false}.into()
            },
            Choice::DrawAndDiscard{riichi} => {
                self.throw_tile_into_river(tile);
                Phase::Ask{kong: false}.into()
            },
            Choice::Kong{figure} => {
                self.land.add(tile);
                if let Some(ts) = self.land.extract_set(Set::from_shape_figure(Shape::KONG, figure)) {
                    self.river.add(self.wind, ts[0]);
                    let (t, i) = self.river.rob(self.wind).expect("hoge");
                    let meld = Meld::from_set_robinfo(Set::from_shape_figure(Shape::KONG, t.figure()), i);
                    self.melds.add(meld);
                    return Phase::Replace{wind: self.wind, expose: false}.into();
                }
                if let Some(t) = self.land.extract(figure) {
                    for m in self.melds.iter_mut() {
                        if m.set() == Set::from_shape_figure(Shape::KONG, figure) && m.wind(self.river).unwrap() == self.wind {
                            self.river.add(self.wind, t);
                            return Phase::Ask{kong: false}.into()
                        }
                    }
                }
                self.land.del(tile);
                return Err(failure::err_msg("can not make kong"))
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
            melds: &mut self.table.lands.melds,
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
    pub fn replace(&mut self, seat: Wind, expose: bool) -> Result<Step, failure::Error> {
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
        seat.show_draw_phase(tile);
        let choice = seat.get_choice();
        seat.do_choice(choice, tile)
    }
    pub fn ask(&mut self, kong: bool) -> Result<Step, failure::Error> {
        let claimee = self.table.rivers.last().expect("Tiles not found on river").discarded_by();
        let mut claims = Claims::new(claimee);

        for claimer in claimee.others() {
            let seat = &mut self.seat(claimer);
            seat.show_claim_phase();
            let claim = seat.get_claim();
            claims.add(claim, claimer);
        }

        let default = if !kong {
            Phase::Draw{wind: claimee.rightside()}
        } else {
            Phase::Replace{wind: claimee, expose: true}
        };

        if let Some((claim, claimer)) = claims.next() {
            match claim {
                Claim::MAHJONG => Finish::WinByDiscard(claimer).into(),
                Claim::CHOW | Claim::PUNG | Claim::KONG if !kong => Phase::Meld{wind: claimer, claim}.into(),
                _ => default.into()
            }
        } else {
            default.into()
        }
    }
    pub fn discard(&mut self, seat: Wind) -> Result<Step, failure::Error> {
        Phase::Ask{kong: false}.into()
    }
}

impl Phase {
    pub fn new() -> Phase {
        Phase::Draw{wind: Wind::EAST}
    }

    pub fn step<'a>(self, state: &mut State<'a>) -> Result<Step, failure::Error> {
        use self::Phase::*;
        match self {
            Draw{wind} => state.draw(wind),
            Choose{wind, drawn} => state.choose(wind, drawn),
            Replace{wind, expose} => state.replace(wind, expose),
            Meld{wind, claim} => state.meld(wind, claim),
            Ask{kong} => state.ask(kong),
            Discard{wind} => state.discard(wind),
        }
    }
}


