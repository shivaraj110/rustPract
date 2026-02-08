use crate::bubble_sort::bubble_sort;
use crate::bubble_sort::bubble_sort_while;
mod bubble_sort;
mod string;
fn main() {
    string::string_match();
    let mut array: [i32; 9] = [23, 43, 56, 78, 11, 14, 16, 65, 34];
    bubble_sort(&mut array);
    println!("Sorted array: (For loop) {:?}", array);
    array = [23, 43, 56, 78, 11, 14, 16, 69, 34];
    bubble_sort_while(&mut array);
    println!("Sorted array (While loop): {:?}", array);
}
