pub fn string_match() {
    let string = String::from("Lmao");
    let substring = string.chars().nth(1);
    match substring {
        Some(c) => {
            println!("{}", c);
        }

        None => {
            println!("Outta range!");
        }
    }
}
