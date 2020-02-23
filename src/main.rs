use std::thread;

fn main() {
    println!("{:?}", thread::current());
    let child = thread::spawn(move || {
        println!("{:?}", thread::current());
        println!("hello from a new thread");
    });

    println!("hello from the main thread");

    child.join().expect("failed to join the child thread");

    let sum = parallel_sum(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    println!("The sum of the numbers 1 to 10 is {}", sum);
}

fn parallel_sum(range: &[i32]) -> i32 {
    const NUM_THREADS: usize = 4;
    if range.len() < NUM_THREADS {
        sum_bucket(range)
    } else {
        let bucket_size = range.len() / NUM_THREADS;
        let mut count = 0;

        let mut threads = Vec::new();
        while count + bucket_size < range.len() {
            let bucket = range[count..count + bucket_size].to_vec();

            let thread = thread::Builder::new()
            .name("calculation".to_owned())
            .spawn(move|| {
                println!("{:?}", thread::current());
                sum_bucket(&bucket)
            })
            .expect("Failed to create the thread");

            threads.push(thread);
            count += bucket_size
        }
        let mut sum = sum_bucket(&range[count..]);

        for thread in threads {
            sum += thread.join().expect("Failed to join thread");
        }
        sum
    }
}

fn sum_bucket(range: &[i32]) -> i32 {
    let mut sum = 0;
    for num in range {
        sum += *num;
    }
    sum
}