// use std::{ sync::{ Arc, RwLock, atomic::{ AtomicI32, Ordering } }, thread, time::Duration };
// use chrono::{ DateTime, Local };

// static UNIQUE_ID: AtomicI32 = AtomicI32::new(0);

// #[derive(Debug)]
// struct MultiThread {
//     id: i32,
//     record_added_time: String,
//     thread_id: String,
// }

// fn main() {
//     let shared_data: Arc<RwLock<Vec<MultiThread>>> = Arc::new(RwLock::new(Vec::new()));

//     let data = Arc::clone(&shared_data);
//     thread::spawn(move || {
//         loop {
//             thread::sleep(Duration::from_secs(10));

//             let id = UNIQUE_ID.fetch_add(1, Ordering::SeqCst) + 1;
//             let now = Local::now();
//             let obj = MultiThread {
//                 id,
//                 record_added_time: now.to_rfc3339(),
//                 thread_id: format!("{:?}", thread::current().id()),
//             };
//             {
//                 let mut vec_to_add_data = data.write().unwrap();
//                 vec_to_add_data.push(obj);
//                 println!("Record added. Total = {}", vec_to_add_data.len());
//             }
//         }
//     });

//     let print_data = Arc::clone(&shared_data);
//     thread::spawn(move || {
//         loop {
//             {
//                 let vec_to_print_data = print_data.read().unwrap();
//                 println!("-----CURRENT RECORDS-----");
//                 for record in vec_to_print_data.iter() {
//                     println!(
//                         "ID : {}, Time : {}, Thread : {}",
//                         record.id,
//                         record.record_added_time,
//                         record.thread_id
//                     );
//                 }
//             }
//             thread::sleep(Duration::from_secs(5));
//         }
//     });

//     let data_to_clean_even = Arc::clone(&shared_data);
//     thread::spawn(move || {
//         loop {
//             let now = Local::now();
//             {
//                 let mut vec_to_clean_even = data_to_clean_even.write().unwrap();
//                 println!("Elements before remove even : {}", vec_to_clean_even.len());
//                 vec_to_clean_even.retain(|record| {
//                     if record.id % 2 != 0 {
//                         return true;
//                     }
//                     if let Ok(time) = DateTime::parse_from_rfc3339(&record.record_added_time) {
//                         let age = (now - time.with_timezone(&Local)).num_seconds();
//                         age <= 20
//                     } else {
//                         true
//                     }
//                 });
//                 println!("Elements after remove even : {}", vec_to_clean_even.len());
//             }
//             thread::sleep(Duration::from_secs(5));
//         }
//     });

//     let data_to_clean_odd = Arc::clone(&shared_data);
//     thread::spawn(move || {
//         loop {
//             let now = Local::now();
//             {
//                 let mut vec_to_clean_odd = data_to_clean_odd.write().unwrap();
//                 println!("Elements before remove odd : {}", vec_to_clean_odd.len());
//                 vec_to_clean_odd.retain(|record| {
//                     if record.id % 2 == 0 {
//                         return true;
//                     }
//                     if let Ok(time) = DateTime::parse_from_rfc3339(&record.record_added_time) {
//                         let age = (now - time.with_timezone(&Local)).num_seconds();
//                         age <= 20
//                     } else {
//                         true
//                     }
//                 });
//                 println!("Elements after remove odd : {}", vec_to_clean_odd.len());
//             }
//             thread::sleep(Duration::from_secs(5));
//         }
//     });

//     let data_to_count_even = Arc::clone(&shared_data);
//     thread::spawn(move || {
//         loop {
//             let mut count = 0;
//             let vec_to_count_even = data_to_count_even.read().unwrap();
//             for record in vec_to_count_even.iter() {
//                 if record.id % 2 == 0 {
//                     count += 1;
//                 }
//             }
//             println!("Number of even id record : {}", count);
//             thread::sleep(Duration::from_secs(5));
//         }
//     });

//     let data_to_count_odd = Arc::clone(&shared_data);
//     thread::spawn(move || {
//         loop {
//             let mut count = 0;
//             let vec_to_count_odd = data_to_count_odd.read().unwrap();
//             for record in vec_to_count_odd.iter() {
//                 if record.id % 2 != 0 {
//                     count += 1;
//                 }
//             }
//             println!("Number of even id record : {}", count);
//             thread::sleep(Duration::from_secs(5));
//         }
//     });

//     loop {
//         thread::sleep(Duration::from_secs(60));
//     }
// }







//------------------------CODE WITH BETTER PRINT AND LOGS--------------
use std::{
    sync::{ Arc, RwLock, atomic::{ AtomicI32, Ordering } },
    thread,
    time::{ Duration, Instant },
};
// use rand::RngExt;
use rand::{distr::{Alphanumeric,SampleString}, rng};
use chrono::{ DateTime, Local };

static UNIQUE_ID: AtomicI32 = AtomicI32::new(0);

#[derive(Debug)]
struct MultiThread {
    id: i32,
    record_added_time: String,
    thread_id: String,
}

fn random_id(len: usize) -> String {
    Alphanumeric.sample_string(&mut rng(), len)
}

fn main() {
    let start = Arc::new(Instant::now());
    let shared_data: Arc<RwLock<Vec<MultiThread>>> = Arc::new(RwLock::new(Vec::new()));

    // ================= PRODUCER =================

    let data = Arc::clone(&shared_data);
    let start_producer = Arc::clone(&start);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(10));

            let id = UNIQUE_ID.fetch_add(1, Ordering::SeqCst) + 1;

            let obj = MultiThread {
                id,
                record_added_time: Local::now().to_rfc3339(),
                thread_id: random_id(5),
                // thread_id: format!("{:?}", thread::current().id()),
            };

            {
                let mut vec = data.write().unwrap();
                vec.push(obj);

                println!(
                    "[{:>4}s][PRODUCER] Added ID={} | Total={}",
                    start_producer.elapsed().as_secs(),
                    id,
                    vec.len()
                );
            }
        }
    });

    // ================= PRINTER =================

    let print_data = Arc::clone(&shared_data);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            {
                let vec = print_data.read().unwrap();

                // println!(
                //     "\n[{:>4}s] ===== CURRENT RECORDS =====",
                //     start_printer.elapsed().as_secs()
                // );

                for r in vec.iter() {
                    println!(
                        "ID={} | Time={} | Thread={} | Current Thread Id={:?}",
                        r.id,
                        r.record_added_time,
                        r.thread_id,
                        thread::current().id()
                    );
                }
            }
        }
    });

    // ================= EVEN CLEANER =================

    let data_to_clean_even = Arc::clone(&shared_data);
    let start_even = Arc::clone(&start);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(20));
            let now = Local::now();

            {
                let mut vec = data_to_clean_even.write().unwrap();
                let before = vec.len();

                vec.retain(|record| {
                    if record.id % 2 != 0 {
                        return true;
                    }

                    if let Ok(time) = DateTime::parse_from_rfc3339(&record.record_added_time) {
                        let age = (now - time.with_timezone(&Local)).num_seconds();
                        age <= 20
                    } else {
                        true
                    }
                });

                let after = vec.len();
                println!(
                    "[{:>4}s][EVEN CLEANER] Removed {}",
                    start_even.elapsed().as_secs(),
                    before - after
                );
            }
        }
    });

    // ================= ODD CLEANER =================

    let data_to_clean_odd = Arc::clone(&shared_data);
    let start_odd = Arc::clone(&start);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(20));
            let now = Local::now();

            {
                let mut vec = data_to_clean_odd.write().unwrap();
                let before = vec.len();

                vec.retain(|record| {
                    if record.id % 2 == 0 {
                        return true;
                    }

                    if let Ok(time) = DateTime::parse_from_rfc3339(&record.record_added_time) {
                        let age = (now - time.with_timezone(&Local)).num_seconds();
                        age <= 20
                    } else {
                        true
                    }
                });

                let after = vec.len();
                println!(
                    "[{:>4}s][ODD CLEANER] Removed {}",
                    start_odd.elapsed().as_secs(),
                    before - after
                );
            }
        }
    });

    // ================= EVEN COUNTER =================

    let data_to_count_even = Arc::clone(&shared_data);
    let start_even_count = Arc::clone(&start);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            let mut count = 0;

            {
                let vec = data_to_count_even.read().unwrap();
                for r in vec.iter() {
                    if r.id % 2 == 0 {
                        count += 1;
                    }
                }
            }

            println!("[{:>4}s][EVEN COUNT] {}", start_even_count.elapsed().as_secs(), count);
        }
    });

    // ================= ODD COUNTER =================

    let data_to_count_odd = Arc::clone(&shared_data);
    let start_odd_count = Arc::clone(&start);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            let mut count = 0;

            {
                let vec = data_to_count_odd.read().unwrap();
                for r in vec.iter() {
                    if r.id % 2 != 0 {
                        count += 1;
                    }
                }
            }

            println!("[{:>4}s][ODD COUNT] {}", start_odd_count.elapsed().as_secs(), count);
        }
    });

    // keep main alive
    // loop {
    thread::sleep(Duration::from_secs(201));
    // }
}
