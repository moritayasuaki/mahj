use rand;
use tile::*;
use table::*;
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
    Draw(Wind),
    DrawRidge(Wind),
    ChooseDrawAction(Wind, Tile),
    ChooseClaimAction(RiverRef),
    ChooseDiscardTile(Wind),
    ChooseRobbingKong(RiverRef),
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
            Ok(next_phase) => phase = next_phase,
            Err(finish) => break finish,
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
        const EAST: Wind = Wind(0);
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

impl Phase {
    pub fn new() -> Phase {
        Phase::Draw(EAST)
    }

    pub fn step<'a>(self, state: &mut State<'a>) -> Result<Result<Self, Finish>, failure::Error> {
        use self::Phase::*;
        match self {
            Draw(seat) => Self::draw(seat, state),
            DrawRidge(seat) => Self::draw_ridge(seat, state),
            _ => unimplemented!()
        }
    }
    pub fn draw(seat: Wind, state: &mut State) -> Result<Result<Self, Finish>, failure::Error> {
        use self::Phase::*;
        if let Some(tile) = state.table.wall.draw() {
            Ok(Ok(ChooseDrawAction(seat, tile)))
        } else {
            Ok(Err(Finish::ExaustiveDraw))
        }
    }
    pub fn draw_ridge(seat: Wind, state: &mut State) -> Result<Result<Self, Finish>, failure::Error> {
        use self::Phase::*;
        if let Some(tile) = state.table.wall.draw_ridge() {
            Ok(Ok(ChooseDrawAction(seat, tile)))
        } else {
            Ok(Err(Finish::ExaustiveDraw))
        }
    }

}

pub fn shuffle_dice() -> usize {
    let r: usize = rand::random();
    (r % 6) + 1
}


enum DrawAction {
    Discard(Tile),
    Kong(Tile),
    AddingKong(Tile),
}