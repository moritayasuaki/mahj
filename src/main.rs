#![feature(non_ascii_idents)]

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
struct Vald_Metorð(u32);

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct Vald_Litur(u16);

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct Vald_Litur_Metorð([u32; 4]);

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
    pub fn make_iter() -> impl Iterator<Item=Self> {
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
            if (Self::LETUR[i] == letur) {
                return Some(Self::frá_auðkenni(i));
            }
        }
        None
    }
    pub fn frá_auðkenni(au: usize) -> Self {
        FlísTýpe::frá_auðkenni(au)
    }
    pub fn í_liturtýpe(self) -> LiturTýpe {
        LiturTýpe::frá_auðkenni(self.auðkenni() / 9)
    }
    pub fn í_raðtala(self) -> Metorð {
        Metorð::frá_auðkenni(self.auðkenni() % 9)
    }
    pub fn make_iter() -> impl Iterator<Item=Self> {
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
    pub fn make_iter() -> impl Iterator<Item=Self> {
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
    pub fn make_iter() -> impl Iterator<Item=Self> {
        (0..Self::NÚMER).map(Self::frá_auðkenni)
    }
}

impl Vald_Metorð {
    pub fn frá_ítreki(metorð: impl Iterator<Item=Metorð>) -> Self {
        let mut m = 0;
        for metorði in metorð {
            m += 1 << metorði.auðkenni()
        }
        Vald_Metorð(m)
    }
}

impl Vald_Litur {
    pub fn frá_ítrek(litir: impl Iterator<Item=LiturTýpe>) -> Self {
        let mut l = 0;
        for litur in litir {
            l += 1 << litur.auðkenni()
        }
        Vald_Litur(l)
    }
}

struct Request {
    thread: u32,
    req: u32
}

type Höndla = Option<thread::JoinHandle<io::Result<()>>>;


fn main() -> io::Result<()> {
    println!("binding localhost:8080 ...");
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
            þráður.join();
        };
    }
    Ok(())
}

#[derive(Debug,Clone)]
enum Command {
    Tsumo,
    Kong(FlísTýpe),
    Pung(FlísTýpe),
    Chow(FlísTýpe),
    Call(FlísTýpe),
    Discard(FlísTýpe),
    Mahjoong(FlísTýpe)
}

fn sub(mut fals : net::TcpStream, veffang : net::SocketAddr, _tx: mpsc::Sender<Request>) -> io::Result<()> {
    let mut s = String::new();
    for f in Flís::make_iter() {
        s.push(f.í_flístýpe().í_letur());
    }
    writeln!(fals, "{}", s)?;
    let r = io::BufReader::new(fals.try_clone()?);
    for line in r.lines() {
        parse_line(&mut fals, &line?)?;
    }
    Ok(())
}

fn reyna_flísar_í_pung(flísar : Vec<FlísTýpe>) -> Option<FlísTýpe> {
    if flísar.len() != 3 {
        return None
    }
    if (flísar[0] != flísar[1]) | (flísar[1] != flísar[2]) {
        return None
    }
    Some(flísar[0])
}

fn reyna_flísar_í_chow(flísar : Vec<FlísTýpe>) -> Option<FlísTýpe> {
    if flísar.len() != 3 {
        return None
    }
    let l = flísar[0].í_liturtýpe();
    if l.er_heiður() {
        return None
    }
    if l.er_heiður() {
        return None
    }
    if (l != flísar[1].í_liturtýpe()) | (l != flísar[2].í_liturtýpe()) {
        return None
    }
    Some(flísar[0]) // todo
}
fn parse_command<'a>(mut tokens: impl Iterator<Item=&'a str>) -> io::Result<Command> {
    fn parse_flís(letúr: char) -> io::Result<FlísTýpe> {
        FlísTýpe::frá_letur(letúr).ok_or(io::Error::new(io::ErrorKind::Other, "no such command"))
    }
    fn parse_flísar<'a>(mut tokens: impl Iterator<Item=&'a str>) -> io::Result<Vec<FlísTýpe>> {
        let flísar = tokens.next().ok_or(io::ErrorKind::Other)?;
        flísar.chars().map(parse_flís).collect()
    }
    fn parse_pung_arg<'a>(mut tokens: impl Iterator<Item=&'a str>) -> io::Result<Command> {
        let flísar = parse_flísar(tokens)?;
        reyna_flísar_í_pung(flísar).map(|c| Command::Pung(c)).ok_or(io::Error::new(io::ErrorKind::Other, "no such command"))
    }
    fn parse_chow_arg<'a>(mut tokens: impl Iterator<Item=&'a str>) -> io::Result<Command> {
        let flísar = parse_flísar(tokens)?;
        reyna_flísar_í_chow(flísar).map(|c| Command::Pung(c)).ok_or(io::Error::new(io::ErrorKind::Other, "no such command"))
    }
    fn parse_discard_arg<'a>(mut tokens: impl Iterator<Item=&'a str>) -> io::Result<Command> {
        let flísar = parse_flísar(tokens)?;
        if flísar.len() == 1 {
            Ok(Command::Discard(flísar[0]))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "no such command"))
        }
    }
    let command = tokens.next().ok_or(io::ErrorKind::Other)?;
    match command.as_ref() {
    "tsumo" => Ok(Command::Tsumo),
    "pung" => parse_pung_arg(tokens),
    "chow" => parse_chow_arg(tokens),
    "discard" => parse_discard_arg(tokens),
    _ => Err(io::Error::new(io::ErrorKind::Other, "no such command"))
    }

}

fn parse_line(fals: &mut net::TcpStream, line: &str) -> io::Result<()> {
    let mut words = line.split_whitespace();
    let command: Command = parse_command(words)?;
    writeln!(fals, "{:?}", command)
}
