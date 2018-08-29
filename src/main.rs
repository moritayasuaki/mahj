#![feature(non_ascii_idents)]

use std::io;
use std::net;
use std::thread;
use std::sync::mpsc;

use io::Write;


#[derive(Copy,Clone)]
struct Flís(u8);

impl Flís {
    const CHARS: [char; 34] = [
        '🀀','🀁','🀂','🀃','🀄','🀅','🀆','🀇','🀈','🀉','🀊','🀋','🀌','🀍','🀎','🀏',
        '🀐','🀑','🀒','🀓','🀔','🀕','🀖','🀗','🀘','🀙','🀚','🀛','🀜','🀝','🀞','🀟',
        '🀠','🀡'];
    fn id(&self) -> usize {
        self.0 as usize
    }
    fn to_char(&self) -> char {
        Self::CHARS[self.id()]
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
    let (tx, rx) = mpsc::channel::<Request>();
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
    writeln!(sock, "hello! {} {}", Flís(1).to_char(), addr)
}
