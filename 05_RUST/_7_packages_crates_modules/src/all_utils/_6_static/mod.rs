mod rwlock;
mod mutex;
pub fn main() {
    println!();
    println!("This code execution from _6_static.");
    println!("Using Mutex---------");
    mutex::call_mutex();

    println!();
    println!("Using RwLock---------");
    rwlock::call_rw();
}
