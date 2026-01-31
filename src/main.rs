use crate::bubble_sort::bubble_sort;
mod bubble_sort;
mod string;
fn main() {
    string::string_match();
    let mut array: [i32; 9] = [23, 43, 56, 78, 11, 14, 16, 65, 34];
    bubble_sort(&mut array);
    println!("Sorted array: {:?}", array);
}
