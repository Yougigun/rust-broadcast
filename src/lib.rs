use std::{collections::HashMap, sync::mpsc::Sender};

pub struct Hub {
    // map to store named channels
    channels: HashMap<String, Channel>,
    // channel to subscribers list
    subscribers: HashMap<String, Vec<Subscriber>>,
    // ip
    ip: String,
    // port
    port: u16, 
}

type Channel = Sender<String>;

use axum::extract::ws::WebSocket;
pub struct Subscriber {
    // name
    name: String,
    // ip
    ip: String,
    // port
    port: u16,
    // channel
    channel: String,
    socket: WebSocket,
}

impl Hub {
    // new hub
    pub fn new(ip: String, port: u16) -> Self {
        Hub {
            channels: HashMap::new(),
            ip,
            port,
            subscribers: HashMap::new(),
        }
    }

    fn run_health_check(&self) {
        // check if all subscribers are alive
    }

    // run hub server
    pub fn run(&mut self) {
        println!("Hub is running on {}:{}", self.ip, self.port);
    }
    // create a new channel
    pub fn create_channel(&mut self, name: String) -> Sender<String> {
        let (tx, _rx) = std::sync::mpsc::channel();
        self.channels.insert(name.clone(), tx);
        self.channels.get(&name).unwrap().clone()
    }

    //
}
pub struct Producer;
pub struct Consumer;

#[cfg(test)]
mod tests {
    use super::*;
}
