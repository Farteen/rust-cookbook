extern crate rand;
use rand::Rng;
use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();

    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            println!("Sending: {}", i);
            tx.send(i).expect("Disconnect from receiver");
        });
    }

    for _ in 0..10 {
        let msg = rx.recv().expect("Disconnected from sender");
        println!("received: {}", msg);
    }

    let (tx, rx) = channel();
    const DISCONNECT: &str = "Goodbye!";
    thread::spawn(move || {
        let mut rng = rand::thread_rng();

        loop {
            let msg = match rng.gen_range(0, 5) {
                0 => "hi",
                1 => DISCONNECT,
                2 => "Howdy there, cowboy",
                3 => "How are you?",
                4 => "I'm good, thanks",
                _ => unreachable!(),
            };
            println!("sending: {}", msg);
            tx.send(msg).expect("Disconnected from receiver");
            if msg == DISCONNECT {
                break;
            }
        }
    });

    for msg in rx {
        println!("received: {}", msg);
    }
}