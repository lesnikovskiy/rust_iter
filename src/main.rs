fn print_elements(elements: &[String]) {
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el))
}

fn main() {
    let colors = vec![String::from("red"), String::from("green"), String::from("blue")];

    print_elements(&colors[1..3]);
}
