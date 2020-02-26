fn main() {
    let names = vec!["Joe", "Miranda", "Alice"];

    let mut iter = names.iter();

    let mut alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let nums = 0..10;

    let all_nums = 0..;

    for num in nums {
        println!("{}", num);
    }

    println!();

    for (index, letter) in "abc".chars().enumerate() {
        println!("#{}.letter in the alphabet: {}", index + 1, letter);
    }

    if let Some(name) = iter.next() {
        println!("First name: {}", name);
    }

    if let Some(name) = iter.next() {
        println!("Second name: {}", name);
    }

    if let Some(name) = iter.next() {
        println!("Third name: {}", name);
    }

    let letter = alphabet.nth(3);
    if let Some(letter) = letter {
        println!("the fourth letter in the alphabet is: {}", letter);
    }

    let current_first = alphabet.nth(0);
    if let Some(current_first) = current_first {
        println!("The first item in the iterator is currently: {}", current_first);
    }

    let current_first = alphabet.nth(0);
    if let Some(current_first) = current_first {
        println!("The first item in the iterator is currently: {}", current_first);
    }

    let last_letter = alphabet.last();
    if let Some(last_letter) = last_letter {
        println!("The last letter of the alphabet is: {}", last_letter);
    }

    let nums: Vec<_> = (1..10).collect();
    println!("nums: {:?}", nums);

    let nums = (1..10).collect::<Vec<_>>();
    println!("nums: {:?}", nums);


    let nums: Vec<_> = all_nums.take(5).collect();
    println!("The first five numbers are: {:?}", nums);

    let nums: Vec<_> = (0..11).skip(2).collect();
    println!("The last 8 letters in a range from zero to 10 is {:?}", nums);

    let nums: Vec<_> = (0..).take_while(|x| x * x < 50).collect();
    println!("All positive numbers that are less than 50 when squared {:?}", nums);

    let names = ["Alfred", "Andy", "Jose", "Luke"];
    let names: Vec<_> = names.iter().skip_while(|x| x.starts_with('A')).collect();
    println!("Names that don't start with 'A': {:?}", names);

    let countries = [
        "U.S.A",
        "Germany",
        "France",
        "Italy",
        "India",
        "Pakistan",
        "Burma",
    ];
    let countries_with_i: Vec<_> = countries
    .iter()
    .filter(|country| country.contains('i'))
    .collect();
    println!("Countries containing the letter 'i' {:?}", countries_with_i);

    if let Some(country) = countries.iter().find(|country| country.starts_with('I')) {
        println!("First country starting with the letter 'I' {}", country);
    }

    if let Some(pos) = countries
    .iter()
    .position(|country| country.starts_with('I')) {
        println!("It's index is: {}", pos);
    }

    let are_any = countries.iter().any(|country| country.len() == 5);
    println!("Is there at least one country that has exactly five letters? {}", are_any);

    let are_all = countries.iter().all(|country| country.len() == 5);
    println!("Do all countries have exactly five letters? {}", are_all);

    let sum: i32 = (1..11).sum();
    let product: i32 = (1..11).product();
    let max = (1..11).max();// Option<T>
    let min = (1..11).min();

    let some_numbers: Vec<_> = (1..4).cycle().take(10).collect();
    println!("some_numbers: {:?}", some_numbers);
    let some_numbers: Vec<_> = (1..4).chain(10..14).collect();
    println!("some_numbers: {:?}", some_numbers);

    let swiss_post_codes = [8957, 5000, 5034];
    let swiss_town = ["Spreitenbach", "Aarau", "Suhr"];
    let zipped: Vec<_> = swiss_post_codes.iter().zip(swiss_town.iter()).collect();
    println!("zipped: {:?}", zipped);

    let zipped: Vec<_> = (b'A'..)
    .zip(1..)
    .take(10)
    .map(|(ch, num)| (ch as char, num))
    .collect();
    println!("zipped: {:?}", zipped);


    let numbers_as_string: Vec<_> = (1..11).map(|x| x.to_string()).collect();
    println!("numbers_as_strings: {:?}", numbers_as_string);

    (1..11).for_each(|x| println!("{}", x));
    println!();

    let squares: Vec<_> = (1..50)
    .filter_map(|x| if x % 3 == 0 {Some(x * x)} else { None })
    .collect();
    println!("Suqares of all numbers under 50 that are divisible by 3: {:?}", squares);

    let alphabet: Vec<_> = (b'A'..b'z' + 1)
    .map(|c| c as char)
    .filter(|c| c.is_alphabetic())
    .collect();
    println!("alphabet: {:?}", alphabet);
}