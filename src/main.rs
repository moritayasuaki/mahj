#![feature(non_ascii_idents)]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate rand;

use std::io;
use std::net;
use std::thread;
use std::sync::mpsc;
use std::ops;
use std::mem;
use std::sync;
use std::fmt;
use std::iter;

use io::{Write, Read, BufRead};

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct Flís(u8);
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct FlísTýpe(u8);
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct LiturTýpe(u8);
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct Metorð(u8);

#[derive(Debug,Clone,PartialEq,Eq)]
struct ValdFlís([u64; 3]);

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct ValdMetorð(u32);

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct ValdLitur(u16);

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct ValdLiturMetorð([u32; 4]);

impl Flís {
    const NÚMER: usize = 136;
    pub fn auðkenni(self) -> usize {
        self.0 as usize
    }
    pub fn í_flístýpe(self) -> FlísTýpe {
        FlísTýpe(self.0 / 4)
    }
    pub fn frá_auðkenni(au: usize) -> Self {
        Flís((au % Self::NÚMER) as u8)
    }
    pub fn gera_ítreki() -> impl Iterator<Item=Self> {
        (0..Self::NÚMER).map(Self::frá_auðkenni)
    }
    pub fn gera_fylki() -> [Flís; Self::NÚMER] {
        let mut f : [Flís; Self::NÚMER] = unsafe {
            mem::uninitialized()
        };
        for i in 0..Self::NÚMER {
            f[i] = Self::frá_auðkenni(i);
        }
        f
    }
}

impl FlísTýpe {
    const NÚMER: usize = 34;
    const LETUR: [char; Self::NÚMER] = [
        '🀇','🀈','🀉','🀊','🀋','🀌','🀍','🀎','🀏',
        '🀐','🀑','🀒','🀓','🀔','🀕','🀖','🀗','🀘',
        '🀙','🀚','🀛','🀜','🀝','🀞','🀟','🀠','🀡',
        '🀀','🀁','🀂','🀃',
        '🀄','🀅','🀆'];
    const _MYNT_BILINU: ops::Range<usize> = 0..9;
    const _BAMBUS_BILINU: ops::Range<usize> = 9..18;
    const _HRINGUR_BILINU: ops::Range<usize> = 18..27;
    const _VINDUR_BILINU: ops::Range<usize> = 27..31;
    const _DREKI_BILINU: ops::Range<usize> = 31..34;
    const _HEIÐUR_BILINU: ops::Range<usize> = 27..34;

    pub fn auðkenni(self) -> usize {
        self.0 as usize
    }
    pub fn í_letur(self) -> char {
        Self::LETUR[self.auðkenni()]
    }
    pub fn frá_letur(letur: char) -> Option<Self> {
        for i in 0..Self::NÚMER {
            if Self::LETUR[i] == letur {
                return Some(Self::frá_auðkenni(i));
            }
        }
        None
    }
    pub fn frá_auðkenni(au: usize) -> Self {
        FlísTýpe((au % Self::NÚMER) as u8)
    }
    pub fn í_liturtýpe(self) -> LiturTýpe {
        LiturTýpe::frá_auðkenni(self.auðkenni() / 9)
    }
    pub fn í_metorð(self) -> Metorð {
        Metorð::frá_auðkenni(self.auðkenni() % 9)
    }
    pub fn frá_litur_og_metorð(l: LiturTýpe, m: Metorð) -> Self {
        Self::frá_auðkenni(l.auðkenni() * 9 + m.auðkenni())
    }
    pub fn gera_ítreki() -> impl Iterator<Item=Self> {
        (0..Self::NÚMER).map(Self::frá_auðkenni)
    }
}

impl LiturTýpe {
    const NÚMER: usize = 4;
    pub fn auðkenni(self) -> usize {
        self.0 as usize
    }
    pub fn frá_auðkenni(au: usize) -> Self {
        LiturTýpe((au % Self::NÚMER) as u8)
    }
    pub fn er_töluorð(self) -> bool {
        self.0 < 3
    }
    pub fn er_heiður(self) -> bool {
        !self.er_töluorð()
    }
    pub fn gera_ítreki() -> impl Iterator<Item=Self> {
        (0..Self::NÚMER).map(Self::frá_auðkenni)
    }
}

impl Metorð {
    const NÚMER: usize = 9;
    pub fn auðkenni(self) -> usize {
        self.0 as usize
    }
    pub fn frá_auðkenni(au: usize) -> Self {
        Metorð((au % Self::NÚMER) as u8)
    }
    pub fn er_endastöð(self) -> bool {
        match self.auðkenni() {
        0 => true,
        8 => true,
        _ => false
        }
    }
    pub fn er_einfalt(self) -> bool {
        !self.er_endastöð()
    }
    pub fn gera_ítreki() -> impl Iterator<Item=Self> {
        (0..Self::NÚMER).map(Self::frá_auðkenni)
    }
}
impl ValdFlís {
    pub fn frá_ítreki<'a>(flísar: impl Iterator<Item=Flís>) -> Self {
        let mut f = [0, 0, 0];
        for flís in flísar {
            let a = flís.auðkenni();
            f[a / 64] |= 1 << (a % 64)
        }
        ValdFlís(f)
    }
    pub fn í_veci<'a>(self) -> Vec<Flís> {
        let mut v = Vec::new();
        let m = self.0;
        for i in 0..Flís::NÚMER {
            if (m[i % 64] & (i as u64)) != 0 {
                v.push(Flís::frá_auðkenni(i));
            }
        }
        v
    }
}

impl ValdMetorð {
    pub fn frá_ítreki<'a>(metorð: impl Iterator<Item=Metorð>) -> Self {
        let mut m = 0;
        for metorði in metorð {
            m += 1 << (3 * metorði.auðkenni())
        }
        ValdMetorð(m)
    }
    pub fn er_tómur(&self) -> bool {
        self.0 == 0
    }
    pub fn ein_tegund(&self) -> Option<Metorð> {
        if self.er_tómur() {
            return None;
        }
        let p = self.0;
        let p = p | p >> 1 | p >> 2;
        let p = p & 0o111111111;
        if (p & (p-1)) != 0 {
            return None;
        }
        let p = p.trailing_zeros() / 3;
        Some(Metorð::frá_auðkenni(p as usize))
    }
    pub fn finna_pung(&self) -> Option<Metorð> {
        let p = self.0;
        let p = p + 0o111111111;
        let p = p & 0o444444444;
        let p = p.trailing_zeros() / 3;
        Some(Metorð::frá_auðkenni(p as usize))
    }
    pub fn finna_kong(&self) -> Option<Metorð> {
        let p = self.0;
        let p = p & 0o444444444;
        let p = p.trailing_zeros() / 3;
        Some(Metorð::frá_auðkenni(p as usize))
    }
    pub fn finna_chow(&self) -> Option<Metorð> {
        let p = self.0;
        let p = p | p >> 1 | p >> 2;
        let p = p & 0o111111111;
        let p = p & p >> 3 & p >> 6;
        let p = p.trailing_zeros() / 3;
        Some(Metorð::frá_auðkenni(p as usize))
    }
}

impl ValdLitur {
    pub fn frá_ítreki<'a>(litir: impl Iterator<Item=LiturTýpe>) -> Self {
        let mut l = 0;
        for litur in litir {
            l += 1 << (4 * litur.auðkenni())
        }
        ValdLitur(l)
    }
    pub fn er_tómur(&self) -> bool {
        self.0 == 0
    }
    pub fn ein_tegund(&self) -> Option<LiturTýpe> {
        if self.er_tómur() {
            return None;
        }
        let p = self.0;
        let p = p | p >> 1 | p >> 2 | p >> 3;
        let p = p & 0x1111;
        if (p & (p-1)) != 0 {
            return None;
        }
        let p = p.trailing_zeros() / 4;
        Some(LiturTýpe::frá_auðkenni(p as usize))
    }
}

struct Request {
    thread: u32,
    req: u32
}

type Höndla = Option<thread::JoinHandle<io::Result<()>>>;

fn main() -> io::Result<()> { println!("binding localhost:8080 ...");
    let hlustandi = net::TcpListener::bind("localhost:8080")?;
    // let mut handles = Vec::new();
    let mut höndfong: [Höndla; 4] = [None, None, None, None];
    let (tx, _rx) = mpsc::channel::<Request>();
    for höndla in &mut höndfong {
        let (mut fals, veffang) = hlustandi.accept()?;
        let tx = tx.clone();
        println!("accepted client {} ", veffang);
        *höndla = Some(thread::spawn(move || sub(fals, veffang, tx)));
    }
    for höndla in &mut höndfong {
        if let Some(þráður) = höndla.take() {
                if þráður.join().is_err() {
                return Err(io::Error::new(io::ErrorKind::Other, "failed to thread join"));
            }
        }
    }
    Ok(())
}


#[derive(Debug,Copy,Clone,PartialEq,Eq)]
enum Command {
    Tsumo,
    Kong(FlísTýpe),
    Pung(FlísTýpe),
    Chow(FlísTýpe),
    Call(FlísTýpe),
    Discard(FlísTýpe),
    Mahjong(FlísTýpe)
}

fn sub(mut fals : net::TcpStream, _veffang : net::SocketAddr, _tx: mpsc::Sender<Request>) -> io::Result<()> {
    let r = io::BufReader::new(fals.try_clone()?);
    for line in r.lines() {
        þatta_line(&mut fals, &line?)?;
    }
    Ok(())
}
fn reyna_flísar_í_kong(flísar : Vec<FlísTýpe>) -> Option<FlísTýpe> {
    if flísar.len() != 4 {
        return None
    }
    let vald_litur = ValdLitur::frá_ítreki(flísar.iter().map(|f| f.í_liturtýpe()));
    let o_litur = vald_litur.ein_tegund();
    let vald_metorð = ValdMetorð::frá_ítreki(flísar.iter().map(|f| f.í_metorð()));
    let o_metorð = vald_metorð.finna_kong();

    if let (Some(litur), Some(metorð)) = (o_litur, o_metorð) {
        return Some(FlísTýpe::frá_litur_og_metorð(litur, metorð));
    }
    None
}

fn reyna_flísar_í_pung(flísar : Vec<FlísTýpe>) -> Option<FlísTýpe> {
    if flísar.len() != 3 {
        return None
    }
    let vald_litur = ValdLitur::frá_ítreki(flísar.iter().map(|f| f.í_liturtýpe()));
    let o_litur = vald_litur.ein_tegund();
    let vald_metorð = ValdMetorð::frá_ítreki(flísar.iter().map(|f| f.í_metorð()));
    let o_metorð = vald_metorð.finna_pung();

    if let (Some(litur), Some(metorð)) = (o_litur, o_metorð) {
        return Some(FlísTýpe::frá_litur_og_metorð(litur, metorð));
    }
    None
}

fn reyna_flísar_í_chow(flísar : Vec<FlísTýpe>) -> Option<FlísTýpe> {
    if flísar.len() != 3 {
        return None
    }
    let vald_litur = ValdLitur::frá_ítreki(flísar.iter().map(|f| f.í_liturtýpe()));
    let o_litur = vald_litur.ein_tegund();
    if o_litur.is_none() {
        return None
    }
    let litur = o_litur.unwrap();
    if litur.er_heiður()  {
        return None
    }
    let vald_metorð = ValdMetorð::frá_ítreki(flísar.iter().map(|f| f.í_metorð()));
    let o_metorð = vald_metorð.finna_chow();

    if let Some(metorð) = o_metorð {
        return Some(FlísTýpe::frá_litur_og_metorð(litur, metorð));
    }
    return None
}

fn þatta_flís(letúr: char) -> io::Result<FlísTýpe> {
    FlísTýpe::frá_letur(letúr)
        .ok_or(io::Error::new(io::ErrorKind::Other, "no such command"))
}
fn þatta_flísar<'a>(mut tokens: impl Iterator<Item=&'a str>) -> io::Result<Vec<FlísTýpe>> {
    let flísar = tokens.next().ok_or(io::ErrorKind::Other)?;
    flísar.chars()
        .map(þatta_flís)
        .collect()
}

fn þatta_kong_arg<'a>(tokens: impl Iterator<Item=&'a str>) -> io::Result<Command> {
    let flísar = þatta_flísar(tokens)?;
    reyna_flísar_í_kong(flísar)
        .map(|c| Command::Kong(c))
        .ok_or(io::Error::new(io::ErrorKind::Other, "no such command"))
}

fn þatta_pung_arg<'a>(tokens: impl Iterator<Item=&'a str>) -> io::Result<Command> {
    let flísar = þatta_flísar(tokens)?;
    reyna_flísar_í_pung(flísar)
        .map(|c| Command::Pung(c))
        .ok_or(io::Error::new(io::ErrorKind::Other, "no such command"))
}

fn þatta_chow_arg<'a>(tokens: impl Iterator<Item=&'a str>) -> io::Result<Command> {
    let flísar = þatta_flísar(tokens)?;
    reyna_flísar_í_chow(flísar)
        .map(|c| Command::Pung(c))
        .ok_or(io::Error::new(io::ErrorKind::Other, "no such command"))
}

fn þatta_discard_arg<'a>(tokens: impl Iterator<Item=&'a str>) -> io::Result<Command> {
    let flísar = þatta_flísar(tokens)?;
    if flísar.len() == 1 {
        Ok(Command::Discard(flísar[0]))
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "no such command"))
    }
}

fn þatta_command<'a>(mut tokens: impl Iterator<Item=&'a str>) -> io::Result<Command> {
    let command = tokens.next().ok_or(io::ErrorKind::Other)?;
    match command.as_ref() {
    "tsumo" => Ok(Command::Tsumo),
    "pung" => þatta_pung_arg(tokens),
    "chow" => þatta_chow_arg(tokens),
    "discard" => þatta_discard_arg(tokens),
    _ => Err(io::Error::new(io::ErrorKind::Other, "no such command"))
    }
}

fn þatta_line(fals: &mut impl Write, line: &str) -> io::Result<()> {
    let words = line.split_whitespace();
    let command: Command = þatta_command(words)?;
    writeln!(fals, "{:?}", command)
}

struct Veggur { // wall
    flísar: [Flís; Flís::NÚMER],
    brjóta_vísitölu: usize,
    dauður_vísitölu: usize,
    næstur_vísitölu: usize
}

impl Veggur {
    fn nýtt() -> Self {
        Veggur {
            flísar: Flís::gera_fylki(),
            brjóta_vísitölu: 0,
            dauður_vísitölu: 0,
            næstur_vísitölu: 0
        }
    }
    fn summa(a: usize, b: usize) -> usize {
        let s = a + b;
        if s >= Flís::NÚMER {
            s - Flís::NÚMER
        } else {
            s
        }
    }
    fn munur(a: usize, b: usize) -> usize {
        if a >= b {
            a - b
        } else {
            a + Flís::NÚMER - b
        }
    }
    fn fækkun(a: usize) -> usize {
        if a == 0 {
            Flís::NÚMER - 1
        } else {
            a - 1
        }
    }
    fn hækkun(a: usize) -> usize {
        if a == (Flís::NÚMER - 1) {
            0
        } else {
            a + 1
        }
    }

    fn eftil(&self) -> usize {
        Self::munur(self.næstur_vísitölu, self.dauður_vísitölu)
    }

    fn draga(&mut self) -> Option<Flís> {
        if self.eftil() <= 14 {
            return None;
        }
        self.næstur_vísitölu = Self::fækkun(self.næstur_vísitölu);
        let flís = self.flísar[self.næstur_vísitölu % Flís::NÚMER];
        Some(flís)
    }

    fn draga_extra(&mut self) -> Option<Flís> {
        if self.eftil() <= 14 {
            return None;
        }
        let flís = self.flísar[self.dauður_vísitölu % Flís::NÚMER];
        self.dauður_vísitölu = Self::hækkun(self.dauður_vísitölu);
        Some(flís)
    }

    fn stokka(&mut self) {
        stokka_fylski(&mut self.flísar);
        let b = rand::random::<usize>() % Flís::NÚMER; 
        self.brjóta_vísitölu = b;
        self.dauður_vísitölu = b;
        self.næstur_vísitölu = b;
    }
}

fn stokka_fylski(f: &mut [impl Copy]) {
    for i in 0..(f.len()) {
        let j = rand::random::<usize>() % (i + 1);
        if i != j {
            let t = f[i];
            f[i] = f[j];
            f[j] = t;
        }
    }
}

enum TangjaTýpe { // connect
    PongStela,
    PongHylja,
    KongStela,
    KongHylja,
    KongLeggja,
    ChowStela,
    ChowHylja,
    Auga
}

struct TangjaAuðkenni(u8); 

struct Tangja(TangjaTýpe, TangjaAuðkenni, FlísTýpe);

struct Hönd { // hand
    flísar: [Flís; 14],
    nflísar: usize,
    tengja: [Tangja; 4],
    ntangja: usize,
}

struct Fljót { // river
    flísar: [Flís; 24],
    nflísar: usize,
    richi: usize,
}

struct Borð {
    höndur: [Hönd; 4],
    fljót: [Fljót; 4],
    veggir: Veggur,
}

struct Ástand {
    stig: [usize; 4],
}

impl Hönd { // hand
    fn bæta(&mut self, f: Flís) { // add
        self.flísar[self.nflísar] = f;
        self.nflísar += 1;
    }
    fn eyða(&mut self, ft: FlísTýpe) -> Option<Flís> { // del
        for i in 0..self.nflísar {
            if self.flísar[i].í_flístýpe() == ft {
                let f = self.flísar[i];
                self.nflísar -= 1;
                self.flísar[i] = self.flísar[self.nflísar];
                return Some(f);
            }
        }
        None
    }
}


#[test]
fn test_þatta_kong() {
    let p = þatta_kong_arg(vec!["🀖🀖🀖"].into_iter());
    assert!(p.is_err());
    let p = þatta_kong_arg(vec!["🀀🀀🀀🀀"].into_iter()).unwrap();
    assert!(p == Command::Kong(FlísTýpe(27)));
    let p = þatta_kong_arg(vec!["🀖🀖"].into_iter());
    assert!(p.is_err());
}
#[test]
fn test_þatta_pung() {
    let p = þatta_pung_arg(vec!["🀖🀖🀖"].into_iter()).unwrap();
    assert!(p == Command::Pung(FlísTýpe(15)));
    let p = þatta_pung_arg(vec!["🀀🀀🀀🀀"].into_iter());
    assert!(p.is_err());
    let p = þatta_pung_arg(vec!["🀖🀖"].into_iter());
    assert!(p.is_err());
}
#[test]
fn test_þatta_chow() {
    let p = þatta_chow_arg(vec!["🀙🀚🀛"].into_iter()).unwrap();
    assert!(p == Command::Pung(FlísTýpe(18)));
    let p = þatta_chow_arg(vec!["🀙🀚"].into_iter());
    assert!(p.is_err());
    let p = þatta_chow_arg(vec!["🀋🀌🀍🀎"].into_iter());
    assert!(p.is_err());
}
#[test]
fn test_vald_metorð_frá_ítreki() {
    let ítreki = vec![Metorð(0), Metorð(1), Metorð(2)].into_iter();
    let p = ValdMetorð::frá_ítreki(ítreki);
    assert!(p == ValdMetorð(0o000000111))
}
#[test]
fn test_vald_metorð_ein_tegund() {
    let vm = ValdMetorð(0o000000111);
    assert!(vm.ein_tegund().is_none());
    let vm = ValdMetorð(0o0);
    assert!(vm.ein_tegund().is_none());
    let vm = ValdMetorð(0o100);
    assert!(vm.ein_tegund().unwrap() == Metorð(2))
}
