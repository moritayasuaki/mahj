use std::io;
use std::net;
use std::thread;

use io::Write;

fn main() -> io::Result<()> {
    println!("binding localhost:8080 ...");
    let listener = net::TcpListener::bind("localhost:8080")?;
    let mut handles = Vec::new();
    for _ in 0..4 {
        let (mut sock, addr) = listener.accept()?;
        println!("accepted client {} ", addr);
        let handle = thread::spawn(move || sub(sock, addr));
        handles.push(handle);
    }

    for handle in handles {
        handle.join();
    }
    Ok(())
}

fn sub(mut sock : net::TcpStream, addr : net::SocketAddr) -> io::Result<()> {
    writeln!(sock, "hello! {}", addr)
}
