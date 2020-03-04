use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let resource = Arc::new(RwLock::new("Hello world".to_string()));

    let reader_a = {
        let resource = resource.clone();
        thread::spawn(move || {
            for _ in 0..40 {
                let resource = resource
                .read()
                .expect("failed to lock resource for reading");
                println!("Reader A says: {}", resource);
            }
        })
    };

    let reader_b = {
        let resource = resource.clone();
        thread::spawn(move || {
            for _ in 0..40 {
                let resource = resource
                .read()
                .expect("failed to lock resource for reading");
                println!("Reader B says: {}", resource);
            }
        })
    };

    let writer = {
        let resource = resource.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let mut resource = resource
                .write()
                .expect("failed to lock resource for writing");
                resource.push('!');
            }
        })
    };

    reader_a.join().expect("Reader A panicked");
    reader_b.join().expect("Reader B panicked");
    writer.join().expect("Writer panicked");
}