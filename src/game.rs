use rand;
use tile::*;
use table::*;
use meld::*;
use action::*;
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
    dice: [usize; 2],
    dealer: usize
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Phase {
    Draw(Wind),
    Choose(Wind, Tile),
    Replace(Wind),
    Claims(),
    Meld(Wind, Claim),
    Discard(Wind),
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Finish {
    WinByDraw(Wind, Tile),
    WinByDiscard(Wind, RiverRef),
    ExaustiveDraw,
    FourRiichiAbort,
    NineTerminalAbort,
    FourWindAbort,
    ThreeWinAbort,
    FourKongAbort,   
}

pub fn run_match() -> Result<[isize; 4], failure::Error> {
    let sticks = &mut Sticks::new();
    for round in Wind::make_iter() {
        run_round(sticks, round)?;
        if sticks.is_bursted() {
            break;
        }
    }
    Ok(sticks.score)
}

pub fn run_halfmatch() -> Result<[isize; 4], failure::Error> {
    let sticks = &mut Sticks::new();
    for round in Wind::make_iter().take(Wind::N/2) {
        run_round(sticks, round)?;
        if sticks.is_bursted() {
            break;
        }
    }
    Ok(sticks.score)
}

pub fn run_eastmatch() -> Result<[isize; 4], failure::Error> {
    let sticks = &mut Sticks::new();
    let round = Wind::from_id(0);
    run_round(sticks, round)?;
    Ok(sticks.score)
}

pub fn run_round(sticks: &mut Sticks, round: Wind) -> Result<(), failure::Error> {
    for dealer in 0..4 {
        run_hands(sticks, round, dealer)?;
        if sticks.is_bursted() {
            break;
        }
    }
    Ok(())
}

pub fn run_hands(sticks: &mut Sticks, round: Wind, dealer: usize) -> Result<(), failure::Error> {
    let table = &mut Table::new();
    loop {
        let bonus_hand = run_hand(sticks, round, dealer, table)?;
        if sticks.is_bursted() {
            break;
        }
        if !bonus_hand {
            break;
        }
    }
    Ok(())
}

pub fn init_table(table: &mut Table) -> [usize; 2] {
    table.shuffle_tiles();
    let dice = [shuffle_dice(), shuffle_dice()];
    table.break_tiles(dice.iter().sum());

    let mut draw = |seat| {
        let tile = table.draw_tile().expect("drawing for building table");
        table.seat(seat).take_tile(tile);
    };
    
    for _ in 0..3 {
        for seat in Wind::make_iter() {
            for _ in 0..4 {
                draw(seat)
            }
        }
    }
    for seat in Wind::make_iter() {
        draw(seat)
    }
    dice
}

pub fn run_hand(sticks: &mut Sticks, round: Wind, dealer: usize, table: &mut Table) -> Result<bool, failure::Error> {
    let dice = init_table(table);
    let mut phase = Phase::new();
    let state = &mut State {
        dice,
        sticks,
        round,
        dealer,
        table,
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

impl Finish {
    pub fn is_goulash_hand(self) -> bool {
        use self::Finish::*;
        match self {
            ExaustiveDraw | FourRiichiAbort |
            NineTerminalAbort | FourWindAbort |
            ThreeWinAbort | FourKongAbort => true,
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

impl<'a> State<'a> {
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
    pub fn choose(&mut self, seat: Wind, tile: Tile) -> Result<Step, failure::Error> {
        println!("{}", self.table.seat(seat).draw_show(tile));
        let stdin = std::io::stdin();
        let line = stdin.lock().lines().next().unwrap()?;
        match Choice::parse(&line)? {
            Choice::Discard(fig) => {
                if self.table.seat(seat).discard_figure(fig) {
                    Phase::Claims().into()
                } else {
                    Err(failure::err_msg(format!("Can not discard {}", fig.show())))
                }
            },
            _ => unimplemented!()
        }
    }
    pub fn claims(&mut self) -> Result<Step, failure::Error> {
        let seat = self.table.rivers.current_mut().expect("Tiles not found on river").discarded_by();
        let stdin = std::io::stdin();
        let mut claims = Claims::collect(stdin.lock().lines().take(3).map(|x| x.unwrap()))?;
        if let Some(ClaimBy{nth, claim}) = claims.next() {
            let claimer = seat.nth(nth as usize);
            Phase::Meld(claimer, claim).into()
        } else {
            Phase::Draw(seat.rightside()).into()
        }
    }
    pub fn discard(&mut self, seat: Wind) -> Result<Step, failure::Error> {
        Phase::Claims().into()
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
            Claims() => state.claims(),
            Discard(seat) => state.discard(seat),
        }
    }
}

pub fn shuffle_dice() -> usize {
    let r: usize = rand::random();
    (r % 6) + 1
}

