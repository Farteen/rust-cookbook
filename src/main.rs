use std::env;

fn main() {
    println!("Got follwing parameters: ");
    for arg in env::args() {
        println!("- {}", arg);
    }

    let mut args = env::args();

    if let Some(arg) = args.nth(0) {
        println!("The path to this program is: {}", arg);
    }

    if let Some(arg) = args.nth(1) {
        println!("The second parameter is: {}", arg);
    }

    if let Some(arg) = args.nth(2) {
        println!("The second parameter is: {}", arg);
    }

    let args: Vec<_> = env::args().collect();
    println!("The path to this program is: {}", args[0]);

    if args.len() > 1 {
        println!("The first parameter is: {}", args[1]);
    }

    if args.len() > 2 {
        println!("The second parameter is: {}", args[2]);
    }

}