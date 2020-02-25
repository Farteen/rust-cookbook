fn main () {
    let mut s = String::new();
    s.push('H');
    s.push('I');
    println!("s: {}", s);

    let s = "Hello".to_string();
    println!("s: {}", s);

    let s = String::from("Hello");
    println!("s: {}", s);

    let s = "汉语";
    println!("s: {}", s);

    let mut s = "Hello ".to_string();
    s.push_str("World");

    for ch in "Tubular".chars() {
        println!("{}", ch);
    }

    println!();

    for ch in "ÿ".chars() {
        println!("{}", ch);
    }

    println!();


    let (first, second) = "HelloThere".split_at(5);
    println!("first: {}, second: {}", first, second);

    let haiku = "\
    she watches \n\
    satisfied after love\n
    he lies\n\
    looking up at nothing\n\
    ";
    for line in haiku.lines() {
        println!("\t{}", line);
    }

    for s in "Never;Give;Up".split(';') {
        println!("{}", s);
    }

    let s: Vec<_> = "::Hi::There::".split("::").collect();
    println!("{:?}", s);

    let s: Vec<_> = "Mr. T.".split_terminator('.').collect();
    println!("{:?}", s);

    for s in "I'm2fast4you".split(char::is_numeric) {
        println!("{}", s);
    }

    for s in "It's not your fault, it's mine".splitn(3, char::is_whitespace) {
        println!("{}", s);
    }

    for c in "The Dark Knight rises".matches(char::is_uppercase) {
        println!("{}", c);
    }

    let saying = "The early bird gets the worm";
    let starts_with_the = saying.starts_with("The");
    println!(
        "Does \"{}\" start iwth \"The\"?: {}",
        saying, starts_with_the
    );

    let starts_with_bird = saying.starts_with("bird");
    println!(
        "Does \"{}\" start with \"bird\"?: {}",
        saying,
        starts_with_bird
    );

    let ends_with_worm = saying.ends_with("worm");
    println!("Does \"{}\" end with \"worm\"?: {}", saying, ends_with_worm);


    let contains_bird = saying.contains("bird");
    println!("Does \"{}\" contain \"bird\"?: {}", saying, contains_bird);


    let a_lot_of_whitespace = "  I   love spaaaace   ";
    let s: Vec<_> = a_lot_of_whitespace.split(' ').collect();
    println!("{:?}", s);
    let s: Vec<_> = a_lot_of_whitespace.split_whitespace().collect();
    println!("{:?}", s);

    let username = "  P3ngu1n\n".trim();
    println!("{}", username);
    let username = "   P3ngun1n\n".trim_left();
    println!("{}", username);

    let username = "   P3ngu1n\n".trim_right();
    println!("{}", username);


    let num = "12".parse::<i32>();
    if let Ok(num) = num {
        println!("{} * {} = {}", num, num, num * num);
    }

    let s = "My dad is the best dad";
    let new_s = s.replace("dad", "mom");
    println!("new_s: {}", new_s);

    let lowercase = s.to_lowercase();
    println!("lowercase: {}", lowercase);

    let uppercase = s.to_uppercase();
    println!("uppercase: {}", uppercase);

    let greek = "sjdflsd";
    println!("lowercase greek: {}", greek.to_lowercase());

    let hello = "Hello! ";
    println!("Three times hello: {}", hello.repeat(3));
}