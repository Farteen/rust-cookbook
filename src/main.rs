use std::sync::{Arc, RwLock};
use std::net::Ipv6Addr;
use std::collections::HashMap;
use std::{thread, time};
use std::sync::atomic::{AtomicUsize, Ordering};

struct Client {
    ip: Ipv6Addr,
}

struct ConnectionHandler {
    clients: RwLock<HashMap<usize, Client>>,
    next_id: AtomicUsize,
}

impl Client {
    fn new(ip:Ipv6Addr) -> Self {
        Client {ip}
    }
}

fn main() {}