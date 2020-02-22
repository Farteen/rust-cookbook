
fn by_moving() {
    let hello = "hello ".to_string();
    let world = "world!";

    let hello_world = hello + world;
    println!("{}", hello_world);
}

fn by_cloning() {
    let hello = "hello ".to_string();
    let world = "world!";
    let hello_clone = hello.clone();
    let hello_world = hello_clone + world;
    // frame var -L hello hello_clone inspect two variables
    println!("{}", hello_world);
}

fn by_mutating() {
    let mut hello = "hello ".to_string();
    let world = "world!";
    hello.push_str(world);

    println!("{}", hello);
}

fn main() {
    by_moving();
    by_cloning();
    by_mutating();
}
