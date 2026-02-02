pub fn main() {
    println!();
    println!("This code execution from _2_control_statements.");
    let mut st: u8 = 3;
    let ed: u8 = 8;
    for i in st..=ed {
        if i % 2 == 0 {
            println!("{i} is even inside for");
        } else {
            println!("{i} is odd inside for");
        }
    }

    while st<ed{
        match st{
            5=>println!("5 it is"),
            6=>break,
            _=>println!("{st} inside while"),
        }
        st+=1;

    }

    let mut st = 1;
    loop{
        println!("{st} inside loop");
        if st== 3{
            println!("here st is 3 and it is inside if statement.");
        }
        st+=1;
        match st{
            8=>break,
            _=>continue,
        }
    }
}

