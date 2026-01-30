use std::sync::Mutex;

static TOTAL_REQUESTS: Mutex<usize> = Mutex::new(0);
static GET_REQUESTS: Mutex<usize> = Mutex::new(0);
static POST_REQUESTS: Mutex<usize> = Mutex::new(0);
static DELETE_REQUESTS: Mutex<usize> = Mutex::new(0);

enum Request {
    Get {
        endpoint: String,
    },
    Post {
        endpoint: String,
        playload_size: u32,
    },
    Delete(u32),
}

fn handle_request(req: Request) -> String {
    {
        *TOTAL_REQUESTS.lock().unwrap() += 1;
    }
    match req {
        Request::Get { endpoint } => {
            {
                *GET_REQUESTS.lock().unwrap() += 1;
            }
            format!("GET endpoint : {endpoint}")
        }
        Request::Post { endpoint, playload_size } => {
            {
                *POST_REQUESTS.lock().unwrap() += 1;
            }
            format!("POST endpoint : {endpoint} playload_size : {playload_size}")
        }
        Request::Delete(id) => {
            {
                *DELETE_REQUESTS.lock().unwrap() += 1;
            }
            format!("DELETE id : {id}")
        }
    }
}
pub fn call_mutex() {
    let request1 = Request::Get { endpoint: "/images".to_string() };
    let request2 = Request::Post { endpoint: "/register".to_string(), playload_size: 50 };
    let request3 = Request::Delete(44);

    let res1 = handle_request(request1);
    let res2 = handle_request(request2);
    let res3 = handle_request(request3);

    println!("{res1} \n=> Total count of GET requst is {}.",GET_REQUESTS.lock().unwrap());
    println!(
        "{res2} \n=> Total count of POST requst is {}.",POST_REQUESTS.lock().unwrap()
    );
    println!(
        "{res3} \n=> Total count of DELETE requst is {}.",DELETE_REQUESTS.lock().unwrap()
    );
    println!("The total number of requests are : {}",TOTAL_REQUESTS.lock().unwrap());

}
