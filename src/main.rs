use std::io::{Result, Write};
use std::net::{TcpListener};

fn main() -> Result<()> {
    println!("binding localhost:8080 ...");
    let listener = TcpListener::bind("localhost:8080")?;
    loop {
        let (mut sock, addr) = listener.accept()?;
        println!("accepted client {} ", addr);
        sock.write(b"hello!\n")?;
        sock.flush()?;
    }
}

