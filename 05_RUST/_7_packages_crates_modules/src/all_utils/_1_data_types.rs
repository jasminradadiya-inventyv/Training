macro_rules! append_item {
    ([$($old:expr),*], [$($new:expr),*]) => {
        [$($old,)* $($new,)*]
    };
}
pub fn main() {
    println!();
    println!("This code execution from _1_data_types.");
    let d: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of d is: {:?}", d);
    let mut new_array = [0; 6];

    for (i, &val) in d.iter().enumerate() {
        new_array[i] = val;
    }
    new_array[5] = 6;
    println!("The value of new_array is: {:?}", new_array);

    new_array[..5].copy_from_slice(&d);
    new_array[5] = 6;
    println!("The value of new_array is: {:?}", new_array);

    let arr = append_item!([1, 2, 3, 4, 5], [6]);
    println!("{:?}", arr);
}
