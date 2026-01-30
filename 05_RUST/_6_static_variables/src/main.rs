mod rwlock;
mod mutex;
fn main() {
    println!();
    println!("Using Mutex---------");
    mutex::call_mutex();

    println!();
    println!("Using RwLock---------");
    rwlock::call_rw();
}
