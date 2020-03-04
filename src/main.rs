use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::ops::{Deref, DerefMut};
use std::cell::UnsafeCell;

fn main() {
    let _some_number = AtomicUsize::new(0);
    let some_number = AtomicUsize::new(0);

    let curr_val = some_number.load(Ordering::SeqCst);
    println!("The current value of some_number is {}", curr_val);

    some_number.store(123, Ordering::SeqCst);
    let curr_val = some_number.load(Ordering::SeqCst);
    println!("The current value of some_number is {}", curr_val);

    let old_val = some_number.swap(12_345, Ordering::SeqCst);
    let curr_val = some_number.load(Ordering::SeqCst);
    println!("The old value {}, the current value {}", old_val, curr_val);

    let comparison = 12_345;
    let new_val = 6_789;
    let old_val = some_number.compare_and_swap(comparison, new_val, Ordering::SeqCst);
    if old_val == comparison {
        println!("The value has been updated");
    }

    let mut some_normal_number = 12_345;
    let old_val = some_normal_number;
    if old_val == comparison {
        some_normal_number = new_val;
        println!("The value has been updated sequantially");
    }

    let old_val_one = some_number.fetch_add(12, Ordering::SeqCst);
    let old_val_two = some_number.fetch_sub(24, Ordering::SeqCst);
    let curr_val = some_number.load(Ordering::SeqCst);
    println!("some_number was first {}, then {} and is now {}", old_val_one, old_val_two, curr_val);

    let some_bool = AtomicBool::new(false);
    let old_val = some_bool.fetch_or(true, Ordering::SeqCst);
    let curr_val = some_bool.load(Ordering::SeqCst);
    println!("{} || true is {}", old_val, curr_val);

    let naive_mutex = Arc::new(NaiveMutex::new(1));
    let updater = {
        let naive_mutex = naive_mutex.clone();
        thread::spawn(move || {
            let mut val = naive_mutex.lock();
            *val = 2;
        })
    };
    let printer = {
        let naive_mutex = naive_mutex.clone();
        thread::spawn(move || {
            let val = naive_mutex.lock();
            println!("the value in the naive mutex is: {}", *val);
        })
    };

    updater.join().expect("The update thread panicked");
    printer.join().expect("The printer thread panicked");

}

pub struct NaiveMutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct NaiveMutextGuard<'a, T: 'a> {
    naive_mutex: &'a NaiveMutex<T>,
}

impl<T> NaiveMutex<T> {
    pub fn new(data: T) -> Self {
        NaiveMutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> NaiveMutextGuard<T> {
        while self.locked.compare_and_swap(false, true, Ordering::SeqCst) {}
        NaiveMutextGuard{naive_mutex: self}
    }
}

unsafe impl<T: Send> Sync for NaiveMutex<T> {}

impl<'a, T> Drop for NaiveMutextGuard<'a, T> {
    fn drop(&mut self) {
        self.naive_mutex.locked.store(false, Ordering::SeqCst);
    }
}

impl<'a, T> Deref for NaiveMutextGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            &*self.naive_mutex.data.get()
        }
    }
}

impl<'a, T> DerefMut for NaiveMutextGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            &mut *self.naive_mutex.data.get()
        }
    }
}