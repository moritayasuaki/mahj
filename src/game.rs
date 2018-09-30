use rand;
use tile::*;
use table::*;
use meld::*;
use failure;

const OK: Result<(), failure::Error> = Ok(());

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
    dice: (usize, usize),
    dealer: usize
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Phase {
    Draw(Wind, bool),
    Meld(Wind, Claim),
    CollectClaims(),
    RespondToDraw(Wind, Tile),
    RespondToDiscard(RiverRef),
    RespondToClaim(Wind)
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Finish {
    WinBySelfDraw(Wind, Tile),
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

pub fn init_table(table: &mut Table, dice: (usize, usize)) {
    table.shuffle_tiles();
    table.break_tiles(dice.0 + dice.1);
    for _ in 0..3 {
        for seat in Wind::make_iter() {
            for _ in 0..4 {
                let tile = table.wall.draw().expect("initial draw");
                table.lands.get_mut(seat).tiles.add(tile);
            }
        }
    }
    for seat in Wind::make_iter() {
        let tile = table.wall.draw().expect("initial draw");
        table.lands.get_mut(seat).tiles.add(tile);
    }
}

pub fn run_hand(sticks: &mut Sticks, round: Wind, dealer: usize, table: &mut Table) -> Result<bool, failure::Error> {
    let dice = (shuffle_dice(), shuffle_dice());
    init_table(table, dice);
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
            WinBySelfDraw(EAST, ..) | WinByDiscard(EAST, ..) => true,
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

impl From<Phase> for Step {
    fn from(phase: Phase) -> Self {
        Step::Phase(phase)
    }
}

impl From<Finish> for Step {
    fn from(finish: Finish) -> Self {
        Step::Finish(finish)
    }
}

impl<'a> State<'a> {
    pub fn draw(&mut self, seat: Wind, ridge: bool) -> Result<Step, failure::Error> {
        let o = if ridge {
            self.table.wall.draw_ridge()
        } else {
            self.table.wall.draw()
        };
        if let Some(tile) = o {
            Ok(Phase::RespondToDraw(seat, tile).into())
        } else {
            Ok(Finish::ExaustiveDraw.into())
        }
    }
    pub fn meld(&mut self, seat: Wind, claim: Claim) -> Result<Step, failure::Error> {
        let _ = (seat, claim);
        unimplemented!()
    }
    pub fn respond_to_draw(&mut self, seat: Wind, tile: Tile) -> Result<Step, failure::Error> {
        use self::Phase::*;
        use self::ResponseToDraw::*;
        let _ = tile;
        match ResponseToDraw::choose()? {
            Discard(tile) => {
                self.table.rivers.discard(seat, tile);
                let r = self.table.rivers.current_ref();
                Ok(RespondToDiscard(r).into())
            },
            ConcealedKong(tile) => {
                let _ = tile;
                Ok(RespondToClaim(seat).into())
            },
            _ => unimplemented!()
        }
    }
    pub fn collect_claims(&mut self) -> Result<Step, failure::Error> {
        use self::Phase::*;
        let discarder = self.table.rivers.current_mut().expect("no one discarded").discarded_by();
        let mut claims = Claims::collect(discarder)?;
        if let Some(ClaimBy{nth, claim}) = claims.next() {
            let claimer = discarder.nth(nth as usize);
            Ok(Meld(claimer, claim).into())
        } else {
            let drawer = discarder.nth(1);
            Ok(Draw(drawer, false).into())
        }
    }
    pub fn respond_to_claim(&mut self, seat: Wind) -> Result<Step, failure::Error> {
        use self::Phase::*;
        use self::ResponseToClaim::*;
        match ResponseToClaim::choose()? {
            Discard(tile) => {
                self.table.rivers.discard(seat, tile);
                let r = self.table.rivers.current_ref();
                Ok(RespondToDiscard(r).into())
            }
        }
    }
}

impl Phase {
    pub fn new() -> Phase {
        Phase::Draw(EAST, false)
    }

    pub fn step<'a>(self, state: &mut State<'a>) -> Result<Step, failure::Error> {
        use self::Phase::*;
        match self {
            Draw(seat, ridge) => state.draw(seat, ridge),
            Meld(seat, claim) => state.meld(seat, claim),
            RespondToDraw(seat, tile) => state.respond_to_draw(seat, tile),
            CollectClaims() => state.collect_claims(),
            RespondToClaim(seat) => state.respond_to_claim(seat),
            _ => unimplemented!()
        }
    }
}

pub fn shuffle_dice() -> usize {
    let r: usize = rand::random();
    (r % 6) + 1
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum ResponseToDraw {
    Discard(Tile),
    ConcealedKong(Tile),
    AddingKong(Tile),
    DaclareReadyHand,
    Mahjong
}

impl ResponseToDraw {
    pub fn choose() -> Result<Self, failure::Error> {
        unimplemented!()
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum ResponseToClaim {
    Discard(Tile)
}

impl ResponseToClaim {
    pub fn choose() -> Result<Self, failure::Error> {
        unimplemented!()
    }
}



#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct ClaimBy{
    claim: Claim,
    nth: usize
}

pub struct Claims(u16);

impl Claims {
    pub fn new() -> Self {
        Claims(0)
    }
    pub fn collect(discarder: Wind) -> Result<Self, failure::Error> {
        let _ = discarder;
        unimplemented!()
    }
    pub fn add(&mut self, claimby: ClaimBy) {
        let ClaimBy{claim, nth} = claimby;
        self.0 |= 0o1 << (3 * claim.id() + nth - 1);
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
            let nth = i % 3 + 1;
            Some(ClaimBy{claim, nth})
        } else {
            None
        }
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum ResponseToDiscard {
    Mahjong,
    Kong,
    Pung,
    Chow,
    Through,
}

impl ResponseToDiscard {
    pub fn choose() -> Result<Self, failure::Error> {
        unimplemented!()
    }
}