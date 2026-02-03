use std::collections::{HashMap, HashSet};

#[derive(Debug,Clone,PartialEq, Eq, Hash)]
#[allow(dead_code)]
struct Custom{
    name:String,
    email:String
}

fn main(){
    let users = hashmap();
    hashset(users.clone());
}

fn hashmap()->HashMap<u32, Custom>{
    let mut users: HashMap<u32, Custom> = HashMap::new();

    users.try_reserve(10).expect("Unable to reserve that much space.");

    users.insert(1,Custom { name: "JR".to_string(), email: "j@r.com".to_string()});
    users.insert(2,Custom { name: "NR".to_string(), email: "n@r.com".to_string()});
    users.insert(3,Custom { name: "MT".to_string(), email: "m@t.com".to_string()});
    users.insert(4,Custom { name: "NG".to_string(), email: "n@g.com".to_string()});
    users.insert(5,Custom { name: "SG".to_string(), email: "s@g.com".to_string()});
    users.insert(6,Custom { name: "NG".to_string(), email: "n@g.com".to_string()});
    users.insert(7,Custom { name: "JJ".to_string(), email: "j@j.com".to_string()});

    for (id, details) in &users {
        println!("ID -> {id}: {:#?}",details);
    }

    let cloned_users = users.clone();

    println!();
    println!("Cloned Users");
    for (id, details) in &cloned_users {
        println!("ID -> {id}: {:#?}", details);
    }

    users.retain(|_, user| user.name.starts_with('J'));

    println!();
    println!("Only retain which user's name starts with 'J'");
    for (id, details) in &users {
        println!("ID {id}: {:#?}", details);
    }
    
    println!();
    println!("To extend some new userws again.");
    let mut other_users: HashMap<_, _> = HashMap::new();
    other_users.insert(10, Custom { name: "PP".to_string(), email: "p@p.com".to_string()});
    other_users.insert(11, Custom { name: "JR".to_string(), email: "j@r.com".to_string()});
    other_users.insert(12, Custom { name: "JR".to_string(), email: "j@r.in".to_string()});
    
    users.extend(other_users);
    
    for (id, details) in &users {
        println!("ID {id}: {:#?}", details);
    }
    
    println!();
    println!("unwrap using get.");
    let user_with_id_7 = users.get(&7).unwrap();
    println!("{user_with_id_7:#?}");
    
    users

}

fn hashset(users:HashMap<u32,Custom>){
    let mut users_set: HashSet<Custom> = HashSet::new();

    for (_,user) in users{
        users_set.insert(user.clone());
    }

    println!();
    println!("added entries");
    println!("Users : {:#?}", users_set);

    let removed_entry = users_set.take(&Custom { email: "p@p.com".to_string(), name: "PP".to_string() });
    println!("Removed ele : {:?}",removed_entry);

    println!("Remaining users : {:#?}", users_set);
    
    // Keep only emails ending with ".com"
    users_set.retain(|user| user.email.ends_with(".com"));
    println!("Remaining users : {:#?}", users_set);
}