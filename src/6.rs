   // This program demonstrates how to create a new thread and have it execute a closure
   use std::thread;
   
   fn main() {
       let handle = thread::spawn(move || {
           println!("Hello, world!");
       });
   
       handle.join();
   }
  