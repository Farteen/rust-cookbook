fn main() {
    let fruits = vec!["apple", "tomato", "pear"];

    println!("Fruits: {:?}", fruits);

    let mut fruits = Vec::new();
    fruits.push("apple");
    fruits.push("tomato");
    fruits.push("pear");
    println!("fruits: {:?}", fruits);

    let last = fruits.pop();
    if let Some(last) = last {
        println!("Removed {} from {:?}", last, fruits);
    }

    fruits.insert(1, "grape");
    println!("fruits after insertion: {:?}", fruits);

    fruits.swap(0, 1);
    println!("fruits after swap: {:?}", fruits);

    let first = fruits.first();
    if let Some(first) = first {
        println!("First fruit: {}", first);
    }

    let last = fruits.last();
    if let Some(last) = last {
        println!("Last fruit: {}", last);
    }

    let second = fruits.get(1);
    if let Some(second) = second {
        println!("Second fruit: {}", second);
    }

    let second = fruits[1];
    println!("Second fruit: {}", second);

    let bunch_of_zeros = vec![0; 5];
    println!("bunch_of_zeros: {:?}", bunch_of_zeros);

    let mut nums = vec![1, 2, 3, 4];
    let second_num = nums.remove(1);
    println!("Removed {} from {:?}", second_num, nums);

    let mut names = vec!["Aaron", "Felicia", "Alex", "Daniel"];
    names.retain(|name| name.starts_with('A'));
    println!("Names starting with A: {:?}", names);

    println!("Does 'names' contain \"Alex\"? {}", names.contains(&"Alex"));


    let mut nums = vec![1, 2, 2, 3, 4, 4, 4, 5];
    nums.dedup();
    println!("Deduped, unsorted nums: {:?}", nums);

    nums.sort();
    println!("Manually sorted nums: {:?}", nums);

    nums.dedup();
    println!("Dequped, sorted nums: {:?}", nums);
 
    nums.reverse();
    println!("nums after being reversed: {:?}", nums);

    let mut alphabet = vec!['a', 'b', 'c'];
    print!("The first two letters of the alphabet are:");

    for letter in alphabet.drain(..2) {
        print!("{} ", letter);
    }
    println!();
    println!("alphabet after being drained: {:?}", alphabet);

    let mut fridge = vec!["Beer", "Leftovers", "Mayonaise"];
    println!("Is the fridge empty {}", fridge.is_empty());
    fridge.clear();
    println!("Is the fridge empty {}", fridge.is_empty());

    let mut colors = vec!["red", "green", "blue", "yellow"];
    println!("colors before splitting: {:?}", colors);
    let half = colors.len() / 2;
    let mut second_half = colors.split_off(half);
    println!("colors after splitting: {:?}", colors);
    println!("second_half: {:?}", second_half);

    colors.append(&mut second_half);
    println!("colors after appending: {:?}", colors);
    println!("second_half after appending: {:?}", second_half);

    let mut stuff = vec!["1", "2", "3", "4", "5"];
    println!("Original stuff: {:?}", stuff);
    let stuff_to_insert = vec!["a", "b", "c"];
    let removed_stuff: Vec<_> = stuff.splice(1..4, stuff_to_insert).collect();
    println!("Spliced stuff: {:?}", stuff);
    println!("Removed stuff: {:?}", removed_stuff);

    // preallocation
    let mut large_vec: Vec<i32> = Vec::with_capacity(1000000);
    println!("large_vec after creation:");
    println!("len:\t\t{}", large_vec.len());
    println!("capacity:\t{}", large_vec.capacity());

    large_vec.shrink_to_fit();
    println!("large_vec after shrinking:");
    println!("len:\t\t{}", large_vec.len());
    println!("capacity:\t{}", large_vec.capacity());

    let mut nums = vec![1, 2, 3, 4];
    let second_num = nums.swap_remove(1);
    println!("Removed {} from {:?}", second_num, nums);


}