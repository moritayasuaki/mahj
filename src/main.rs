#![feature(non_ascii_idents)]

use std::io;
use std::net;
use std::thread;
use std::sync::mpsc;
use std::ops;

use io::Write;


#[derive(Copy,Clone)]
struct FlísTýpe(u8);
#[derive(Copy,Clone)]
struct LiturTýpe(u8);

impl FlísTýpe {
    const NÚMER: usize = 34;
    const LETUR: [char; Self::NÚMER] = [
        '🀀','🀁','🀂','🀃','🀄','🀅','🀆','🀇','🀈','🀉','🀊','🀋','🀌','🀍','🀎','🀏',
        '🀐','🀑','🀒','🀓','🀔','🀕','🀖','🀗','🀘','🀙','🀚','🀛','🀜','🀝','🀞','🀟',
        '🀠','🀡'];
    const VEÐUR_BILINU: ops::Range<usize> = 0..4;
    const DREKI_BILINU: ops::Range<usize> = 4..7;
    const HEIÐUR_BILINU: ops::Range<usize> = 0..7;
    const MYNT_BILINU: ops::Range<usize> = 7..16;
    const BAMBUS_BILINU: ops::Range<usize> = 16..25;
    const HRINGUR_BILINU: ops::Range<usize> = 25..34;

    pub fn auðkenni(self) -> usize {
        self.0 as usize
    }
    pub fn í_letur(self) -> char {
        Self::LETUR[self.auðkenni()]
    }
    pub fn frá_auðkenni(au: usize) -> Self {
        FlísTýpe((au % Self::NÚMER) as u8)
    }
    pub fn í_liturtýpe(self) -> LiturTýpe {
        match self.auðkenni() {
        0...6 => LiturTýpe(0),
        7...15 => LiturTýpe(1),
        16...24 => LiturTýpe(2),
        25...33 => LiturTýpe(3),
        _ => panic!("Ógilt auðkenni")
        }
    }
    pub fn er_veður(self) {
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
    pub fn er_heiður(self) -> bool {
        self.0 == 0
    }
    pub fn er_töluorð(self) -> bool {
        !self.er_heiður()
    }
}


struct Request {
    thread: u32,
    req: u32
}

fn main() -> io::Result<()> {
    println!("binding localhost:8080 ...");
    let listener = net::TcpListener::bind("localhost:8080")?;
    let mut handles = Vec::new();
    let (tx, _rx) = mpsc::channel::<Request>();
    for _ in 0..4 {
        let (mut sock, addr) = listener.accept()?;
        println!("accepted client {} ", addr);
        let tx = tx.clone();
        let handle = thread::spawn(move || sub(sock, addr, tx));
        handles.push(handle);
    }

    for handle in handles {
        handle.join();
    }
    Ok(())
}

fn sub(mut sock : net::TcpStream, addr : net::SocketAddr, tx: mpsc::Sender<Request>) -> io::Result<()> {
    writeln!(sock, "hello! {} {}", FlísTýpe(1).í_letur(), addr)
}
