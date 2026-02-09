use crate::create_article::create_article;
mod bubble_sort;
mod create_article;
mod string;
fn main() {
    // string::string_match();
    // let mut array: [i32; 9] = [23, 43, 56, 78, 11, 14, 16, 65, 34];
    // bubble_sort(&mut array);
    // println!("Sorted array: (For loop) {:?}", array);
    // array = [23, 43, 56, 78, 11, 14, 16, 69, 34];
    // bubble_sort_while(&mut array);
    // println!("Sorted array (While loop): {:?}", array);
    let title = String::from("lmao");
    let content = String::from("lmfao");
    let has_images = true;
    let published = false;
    let has_meta = true;
    let article = create_article(title, content, has_images, published, has_meta);
    println!("article created successfully {}", article);
}
