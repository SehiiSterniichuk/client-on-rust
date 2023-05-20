mod config {
    pub mod request_type;
    pub mod response_type;
    pub mod status;
}

mod lab1 {
    pub mod matrix;
}

mod client;
mod print_writer;
mod buffered_reader;
mod custom_error;

mod prefix {
    pub const SIZE: &str = "size: ";
    pub const THREADS: &str = "number-of-threads: ";
    pub const ID: &str = "id: ";
    // pub const ERROR: &str = "error: ";
    pub const TIME: &str = "execution-time: ";
}

use client::Client;
use std::thread;
use std::time::Duration;
use scoped_threadpool::Pool;

fn main() {
    const HOST: &str = "localhost";
    // 'str' is a string slice, which is an immutable reference to a sequence of UTF-8 bytes.
    // String slices are efficient and lightweight, and they don't have ownership of the underlying data
    // 'String' is a heap-allocated, growable string type. It is owned and mutable, allowing you to modify the string's contents.
    const PORT: u16 = 1234;
    const CPU_CORES: i32 = 8;
    const CPU_LOGICAL_CORES: i32 = 16;
    const MIN_THREADS: i32 = CPU_CORES / 2;
    let thread_numbers: Vec<i32> = vec![
        CPU_LOGICAL_CORES * 16,
        CPU_LOGICAL_CORES * 8,
        CPU_LOGICAL_CORES * 4,
        1,
        MIN_THREADS,
        CPU_CORES,
        CPU_LOGICAL_CORES,
        CPU_LOGICAL_CORES * 2,
    ];

    let dimension_numbers: Vec<i32> = vec![
        MIN_THREADS * 256 * 4,
        // MIN_THREADS * 256 * 16,
        // MIN_THREADS * 256 / 2,
        MIN_THREADS * 256 * 2,
        // MIN_THREADS * 256 * 8,
        MIN_THREADS * 256,
    ];

    thread::sleep(Duration::from_secs(1));
    /*
    Double Colon Operator (::):
Used to access associated items of a module, such as constants, functions, or types (structs, enums, traits).
Used to access associated functions of a struct, enum, or trait without an instance.
Used to access associated constants of a struct, enum, or trait.
Used to access methods on a type itself (static methods).
Example: std::vec::Vec::new(), String::from("hello").
    */
    let mut counter: i32 = 0;
    let mut pool = Pool::new(6);
    pool.scoped(|scope| {
        //The tasks are automatically joined when the scoped block ends, so there's no need to call join explicitly.
        for thread in thread_numbers {
            for size in &dimension_numbers {
                let host = HOST.to_owned();
                let port = PORT;
                let size = *size;
                let thread = thread;
                counter += 1;
                let counter = counter;
                scope.execute(move || {
                    let mut client = Client::new(&host, port, size, counter, thread, -1);
                    client.run();
                });
                /*
                 The move keyword is used to move ownership of
                  variables (host, port, size, thread, counter) into the closure (anonymous function: |...| { ... } syntax).
                   This allows the closure to take ownership and access those variables.
                */
            }
        }
    });
    let mut killer = Client::new(HOST, PORT, -1, -1, -1, -1);
    killer.run();

    println!("Clients have finished work");
    /*
    Fundamentally, macros! are a way of writing code that writes other code, which is known as metaprogramming
    */
}
