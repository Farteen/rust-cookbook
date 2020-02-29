fn main() {
    let fib: Vec<_> = fibonacci().take(10).collect();
    println!("First 10 numbers of the fibonacci sequence: {:?}", fib);

    let mut squared_vec = SquaredVec::new();
    squared_vec.push(1);
    squared_vec.push(2);
    squared_vec.push(3);

    for (index, num) in squared_vec.iter().enumerate() {
        println!("{}^2 is {}", index + 1, num);
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci {curr: 0, next: 1}
}

struct Fibonacci {
    curr: u32,
    next: u32
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let old = self.curr;
        self.curr = self.next;
        self.next += old;
        Some(old)
    }
}

use std::ops::Mul;
struct SquaredVec<T>
where
T: Mul + Copy,
{
    vec: Vec<T::Output>,
}

impl<T> SquaredVec<T>
where
T: Mul + Copy
{
    fn new() -> Self {
        SquaredVec { vec: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.vec.push(item * item)
    }
}

use std::ops::Deref;
impl<T> Deref for SquaredVec<T>
where
T: Mul + Copy,
{
    type Target = [T::Output];
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}