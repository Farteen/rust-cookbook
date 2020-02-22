fn main() {
    let color = "red";
    let favorite = format!("My favorite color is {}", color);

    println!("{}", favorite);

    let hello = "hello ";
    let world = "world!";
    let hello_world = format!("{}{}", hello, world);
    println!("{}", hello_world);

    let favorite_num = format!("My favorite number is {}", 42);
    println!("{}", favorite_num);

    let duck_duck_goose = format!("{0}, {0}, {0}, {1}!", "duck", "goose");
    println!("{}", duck_duck_goose);

    let introduction = format!(
        "My name is {surname}, {forename} {surname}",
        surname="Bond",
        forename="James"
    );
    println!("{}", introduction);
}

