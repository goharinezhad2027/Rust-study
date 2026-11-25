fn change_name(name: &mut String) {
    name.push_str(" OS");
}

fn print_name(name: &String) {
    println!("Current name: {}", name);
}

fn main() {
    let mut language = String::from("Rust");

    print_name(&language);

    change_name(&mut language);

    print_name(&language);
}