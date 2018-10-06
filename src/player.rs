use std::net;
use std::io;
use std::io::{Write, BufRead};
use failure;

pub struct SharedStdin;
pub struct SharedStdout;
impl Write for SharedStdout {
    fn write(&mut self, data:&[u8]) -> io::Result<usize> {
        io::stdout().write(data)
    }
    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }
}

impl Iterator for SharedStdin {
    type Item = Result<String, failure::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut string = String::new();
        let r = io::stdin().read_line(&mut string);
        if let Err(e) = r {
            Some(Err(e.into()))
        } else {
            Some(Ok(string))
        }
    }
}

pub struct Player {
    pub name: String,
    pub tx: Option<Box<dyn Write>>,
    pub rx: Option<Box<dyn Iterator<Item=Result<String, failure::Error>>>>
}

impl Player {
    pub fn new() -> Self {
        Player {
            name: String::new(),
            tx: None,
            rx: None,
        }
    }
    pub fn from_socket(listener: &net::TcpListener) -> Result<Player, failure::Error> {
        let (out, addr) = listener.accept()?;
        let input = io::BufReader::new(out.try_clone()?)
            .lines()
            .map(|r| r.map_err(|e| e.into()));
        Ok(Player {
            name: format!("{}", addr).to_string(),
            tx: Some(Box::new(out)),
            rx: Some(Box::new(input)),
        })
    }
    pub fn from_stdio() -> Result<Player, failure::Error> {
        Ok(Player {
            name: "stdio".to_string(),
            tx: Some(Box::new(SharedStdout)),
            rx: Some(Box::new(SharedStdin)),
        })
    }
    pub fn get_line(&mut self) -> Result<String, failure::Error> {
        if let Some(ref mut rx) = self.rx {
            if let Some(res) = rx.next() {
                res
            } else {
                Err(failure::err_msg("RX error"))
            }
        } else {
            Err(failure::err_msg("RX port not found"))
        }
    }
}

impl Write for Player {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(ref mut tx) = self.tx {
            tx.write(buf)
        } else {
            Ok(0)
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        if let Some(ref mut tx) = self.tx {
            tx.flush()
        } else {
            Ok(())
        }
    }
}