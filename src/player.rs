use std::net;
use std::io;
use std::io::{Write, BufRead};
use failure;

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
    fn from_socket(listener: &net::TcpListener) -> Result<Player, failure::Error> {
        let (out, addr) = listener.accept()?;
        let input = io::BufReader::new(out.try_clone()?).lines().map(|r| r.map_err(|e| e.into()));
        Ok (Player {
            name: format!("{}", addr).to_string(),
            tx: Some(Box::new(out)),
            rx: Some(Box::new(input))
        })
    }
    fn from_stdin() -> Result<Player, failure::Error> {
        Err(failure::err_msg("hoge"))
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