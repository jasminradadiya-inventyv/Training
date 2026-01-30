use std::sync::RwLock;

static TOTAL_REQUESTS: RwLock<usize> = RwLock::new(0);
static GET_REQUESTS: RwLock<usize> = RwLock::new(0);
static POST_REQUESTS: RwLock<usize> = RwLock::new(0);
static DELETE_REQUESTS: RwLock<usize> = RwLock::new(0);

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
        *TOTAL_REQUESTS.write().unwrap() += 1;
    }

    match req {
        Request::Get { endpoint } => {
            {
                *GET_REQUESTS.write().unwrap() += 1;
            }
            format!("GET endpoint : {endpoint}")
        }
        Request::Post { endpoint, playload_size } => {
            {
                *POST_REQUESTS.write().unwrap() += 1;
            }
            format!("POST endpoint : {endpoint} playload_size : {playload_size}")
        }
        Request::Delete(id) => {
            {
                *DELETE_REQUESTS.write().unwrap() += 1;
            }
            format!("DELETE id : {id}")
        }
    }
}

pub fn call_rw() {

    let request1 = Request::Get {
        endpoint: "/images".to_string(),
    };
    let request2 = Request::Post {
        endpoint: "/register".to_string(),
        playload_size: 52,
    };
    let request3 = Request::Delete(44);

    let res1 = handle_request(request1);
    let res2 = handle_request(request2);
    let res3 = handle_request(request3);

    println!(
        "{res1} \n=> Total count of GET request is {}.",
        *GET_REQUESTS.read().unwrap()
    );

    println!(
        "{res2} \n=> Total count of POST request is {}.",
        *POST_REQUESTS.read().unwrap()
    );

    println!(
        "{res3} \n=> Total count of DELETE request is {}.",
        *DELETE_REQUESTS.read().unwrap()
    );

    println!(
        "The total number of requests are : {}",
        *TOTAL_REQUESTS.read().unwrap()
    );
}
