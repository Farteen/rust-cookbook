use std::collections::HashSet;

fn main() {
    let mut books = HashSet::new();
    books.insert("Harry Potter and the Philosopher's Stone");
    books.insert("The Name of the Wind");
    books.insert("A Game of Thrones");

    let is_new = books.insert("The Lies of Locke Lamora");
    if is_new {
        println!("We've just added a new book!");
    }

    let is_new = books.insert("A Game of Thrones");
    if !is_new {
        println!("Sorry, we already had that book in store");
    }

    if !books.contains("The Doors of Stone") {
        println!("We sadly don't have that book yet");
    }

    let was_removed = books.remove("The Darkness that comes before");
    if !was_removed {
        println!("Couldn't remove book; We didn't have it to begin with");
    }

    let one_to_five: HashSet<_> = (1..6).collect();
    let five_to_ten: HashSet<_> = (5..11).collect();
    let one_to_ten: HashSet<_> = (1..11).collect();
    let three_to_eight: HashSet<_> = (3..9).collect();

    let is_disjoint = one_to_five.is_disjoint(&five_to_ten);
    println!("is {:?} disjoint from {:?}?: {}", one_to_ten, five_to_ten, is_disjoint);

    let is_disjoint = one_to_five.is_disjoint(&three_to_eight);
    println!("is {:?} disjoint from {:?}?: {}", one_to_five, three_to_eight, is_disjoint);

    // difference symetric_difference
    // intersection union

}