fn print_elements(elements: &[String]) {
    elements.iter().for_each(|el| println!("{}", el))
}

fn short_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn main() {
    let mut colors = vec![String::from("red"), String::from("green"), String::from("blue")];

    short_strings(&mut colors[0..2]);
    print_elements(&colors);
}
