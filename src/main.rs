#![feature(non_ascii_idents)]
#![allow(dead_code)]
#![allow(unused_imports)]

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
        FlísTýpe((au % 34) as u8)
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
    pub fn taka_til_pung(&self) -> Option<Metorð> {
        let p = self.0;
        let p = p + 0o111111111;
        let p = p & 0o444444444;
        let p = p.trailing_zeros() / 3;
        Some(Metorð::frá_auðkenni(p as usize))
    }
    pub fn taka_til_kong(&self) -> Option<Metorð> {
        let p = self.0;
        let p = p & 0o444444444;
        let p = p.trailing_zeros() / 3;
        Some(Metorð::frá_auðkenni(p as usize))
    }
    pub fn taka_til_chow(&self) -> Option<Metorð> {
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
    let mut s = String::new();
    for f in Flís::gera_ítreki() {
        s.push(f.í_flístýpe().í_letur());
    }
    writeln!(fals, "{}", s)?;
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
    let o_metorð = vald_metorð.taka_til_kong();

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
    let o_metorð = vald_metorð.taka_til_pung();

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
    let o_metorð = vald_metorð.taka_til_chow();

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