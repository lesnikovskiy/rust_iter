fn print_elements(elements: &[String]) {
    elements.iter().for_each(|el| println!("{}", el))
}

fn short_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
}

fn main() {
    let colors = vec![String::from("red"), String::from("green"), String::from("blue")];
    let mut colors = to_uppercase(&colors);
    short_strings(&mut colors[0..2]);
    print_elements(&colors);
}
