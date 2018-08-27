use std::io;
use std::net;
use std::thread;
use std::sync::mpsc;

use io::Write;

struct Request {
    thread : u32,
    req : u32
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
    writeln!(sock, "hello! {}", addr)
}
