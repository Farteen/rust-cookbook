use std::collections::VecDeque;

fn main() {
    let mut orders = VecDeque::new();
    orders.push_back("oysters");

    orders.push_back("fish and chips");
    
    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared);
    }

    orders.push_back("mozarella sticks");

    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared);
    }

    let mut some_queue = VecDeque::with_capacity(5);
    some_queue.push_back("A");
    some_queue.push_back("B");
    some_queue.push_back("C");
    some_queue.push_back("D");
    some_queue.push_back("E");
    println!("some_queue: {:?}", some_queue);

    some_queue.swap_remove_back(2);
    println!("some queue after swap_remove_back: {:?}", some_queue);

    some_queue.swap_remove_front(2);
    println!("some_queue after swap_remove_front: {:?}", some_queue);
}