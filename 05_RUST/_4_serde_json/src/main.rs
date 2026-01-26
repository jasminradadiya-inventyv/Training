use serde_json;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
struct Technology {
    frontend: String,
    backend: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
struct Project {
    name: String,
    tech: Technology,
    category: String,
    no_of_emp: u8,
    no_of_user: u16,
    is_completed: bool,
    organization: String,
}

fn main() {
    let p1 = Project {
        name: "WebRTC".to_string(),
        tech: Technology {
            frontend: "Angular".to_string(),
            backend: "Rust".to_string(),
        },
        category: String::from("Communication"),
        no_of_emp: 50,
        no_of_user: 800,
        is_completed: false,
        organization: String::from("inventyv"),
    };
    
    println!();
    let json_string_ser = serde_json::to_string(&p1).unwrap();
    println!("json_string_ser : {json_string_ser}");
    println!();
    
    let json_from_str_deser: Project = serde_json::from_str(&json_string_ser).unwrap();
    println!("json_from_str_deser : {:#?}",json_from_str_deser);
    println!();
    
    let raw_string = r#"{"name": "WebRTC", "tech": { "frontend": "Angular", "backend": "Rust"}, "category": "Communication", "no_of_emp": 50, "no_of_user": 800, "is_completed": false, "organization": "inventyv"}"#;
    let json_from_raw_string:Project = serde_json::from_str(raw_string).unwrap();
    println!("json_from_raw_string : {:#?}",json_from_raw_string);
    println!();
}
