use std::collections::HashMap;

fn main() {
    let mut tv_ratings = HashMap::new();

    tv_ratings.insert("The IT Crowd", 8);
    tv_ratings.insert("13 Reasons Why", 7);

    let contains_tv_show = tv_ratings.contains_key("House of Cards");
    println!("Did we rate House of Card? {}", contains_tv_show);

    if let Some(rating) = tv_ratings.get("Breaking Bad") {
        println!("I rate Breaking Bad {} out of 10", rating);
    }

    let old_rating = tv_ratings.insert("13 Reason Why", 9);
    if let Some(old_rating) = old_rating {
        println!("13 Reason Why's old rating was {} out of 10", old_rating);
    }

    let removed_value = tv_ratings.remove("The ID Crowd");
    if let Some(removed_value) = removed_value {
        println!("The removed series had a rating of {}", removed_value);
    }

    println!("All ratings");

    for (key, value) in &tv_ratings {
        println!("{}\t: {}", key, value);
    }


    let mut age = HashMap::with_capacity(10);
    age.insert("Dory", 8);
    age.insert("Nemo", 5);
    age.insert("Merlin", 10);
    age.insert("Bruce", 9);

    println!("All ages:");
    for age in age.values() {
        println!("{}", age);
    }

    for age in age.values_mut() {
        *age *= 10;
        println!("{}", age);
    }

    {
        // mut borrow
        age.entry("coral").or_insert(11);
    }
    age.entry("coral").or_insert(15);
    
}