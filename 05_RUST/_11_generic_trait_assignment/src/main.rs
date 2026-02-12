use std::collections::HashMap;
use std::fmt::{Display};

struct Inventory<'a, T> {
    items: HashMap<String,&'a T>,
}

#[derive(Clone,Debug)]
struct Product {
    name: String,
    price: f32,
}
trait DisplayItem {
    fn display(&self) -> Result<String, InventoryError>;
}

#[derive(Debug)]
enum InventoryError {
    DuplicateId,
    InvalidId,
    ItemNotFound,
}
impl Display for InventoryError{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)-> std::fmt::Result{
        match self {
            InventoryError::ItemNotFound => write!(f,"Item not found "),
            InventoryError::InvalidId => write!(f,"Invalid item"),
            InventoryError::DuplicateId => write!(f,"Item already exist"),
        }
    }
}

impl<'a,T> Inventory<'a,T> {
    fn new() -> Self {
        Inventory { items: HashMap::new() }
    }

    fn add_item(&mut self, id:String, item:&'a T) -> Result<(), InventoryError> {
        if id.trim().is_empty() {
            return Err(InventoryError::InvalidId);
        }
        if self.items.contains_key(&id) {
            return Err(InventoryError::DuplicateId);
        }
        self.items.insert(id, item);
        Ok(())
    }
}

impl<'a, T: DisplayItem + Clone> Inventory<'a,T> {
    fn get_item(&self, id:String) -> Result<&'a T, InventoryError> {
        self.items
            .get(&id)
            .cloned()
            .ok_or(InventoryError::ItemNotFound)
    }
    
    fn display_all(&self) -> Result<String, InventoryError> {

        let format_items = |id:&String,item:&T| -> Result<String,InventoryError>{
            Ok(format!("\nId: {}\n{}", id, item.display()?))
        };
        if self.items.is_empty() {
            return Err(InventoryError::ItemNotFound)
        }
        let mut output = String::new();
        for (id, item) in &self.items {
            output.push_str(&format_items(id,item)?);
        }
        Ok(output)
    }
}

impl DisplayItem for Product {
    fn display(&self) -> Result<String, InventoryError> {
        Ok(format!("Product: {}\nPrice: {}", self.name, self.price))
    }
}

fn main() {
    let mut inventory = Inventory::<Product>::new();

    // let id1 = "1".to_string();
    let p1 = Product {
        name: "Laptop".to_string(),
        price: 75000.0,
    };
    

    // let id2 = "2".to_string();
    let p2 = Product {
        name: "Phone".to_string(),
        price: 30000.0,
    };

    inventory.add_item("1".to_string(),&p1).unwrap();
    inventory.add_item("2".to_string(), &p2).unwrap();
    let found = inventory.get_item("1".to_string());
    println!();
    println!("Item from found is {:#?}",found);

    println!("{}", inventory.display_all().unwrap());
}



// ------------------------------------ w/o lifetime and closure---------------------------

// use std::collections::HashMap;
// use std::fmt::Display;

// struct Inventory<T> {
//     items: HashMap<String, T>,
// }

// #[derive(Clone,Debug)]
// struct Product {
//     name: String,
//     price: f32,
// }
// trait DisplayItem {
//     fn display(&self) -> Result<String, InventoryError>;
// }

// #[derive(Debug)]
// enum InventoryError {
//     DuplicateId,
//     InvalidId,
//     ItemNotFound,
// }
// impl Display for InventoryError{
//     fn fmt(&self,f:&mut std::fmt::Formatter<'_>)-> std::fmt::Result{
//         match self {
//             InventoryError::ItemNotFound => write!(f,"Item not found "),
//             InventoryError::InvalidId => write!(f,"Invalid item"),
//             InventoryError::DuplicateId => write!(f,"Item already exist"),
//         }
//     }
// }

// impl<T> Inventory<T> {
//     fn new() -> Self {
//         Inventory { items: HashMap::new() }
//     }

//     fn add_item(&mut self, id: String, item: T) -> Result<(), InventoryError> {
//         if id.trim().is_empty() {
//             return Err(InventoryError::InvalidId);
//         }
//         if self.items.contains_key(&id) {
//             return Err(InventoryError::DuplicateId);
//         }
//         self.items.insert(id, item);
//         Ok(())
//     }
// }

// impl<T: DisplayItem + Clone> Inventory<T> {
//     fn get_item(&self, id: String) -> Result<T, InventoryError> {
//         self.items
//             .get(&id)
//             .cloned()
//             .ok_or(InventoryError::ItemNotFound)
//     }
//     fn display_all(&self) -> Result<String, InventoryError> {
//         if self.items.is_empty() {
//             return Err(InventoryError::ItemNotFound);
//         }
//         let mut output = String::new();
//         for (id, item) in &self.items {
//             output.push_str(&format!("\nId: {}\n{}", id, item.display()?));
//         }
//         Ok(output)
//     }
// }

// impl DisplayItem for Product {
//     fn display(&self) -> Result<String, InventoryError> {
//         Ok(format!("Product: {}\nPrice: {}", self.name, self.price))
//     }
// }

// fn main() {
//     let mut inventory = Inventory::<Product>::new();

//     let p1 = Product {
//         name: "Laptop".to_string(),
//         price: 75000.0,
//     };

//     let p2 = Product {
//         name: "Phone".to_string(),
//         price: 30000.0,
//     };

//     inventory.add_item("1".to_string(), p1).unwrap();
//     inventory.add_item("2".to_string(), p2).unwrap();
//     let found = inventory.get_item("".to_string());
//     println!();
//     println!("Item from found is {:#?}",found);

//     println!("{}", inventory.display_all().unwrap());
// }
