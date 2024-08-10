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

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn main() {
    let original = vec![String::from("red"), String::from("green"), String::from("blue")];
    let mut colors = vec![];
    move_elements(original, &mut colors);
    let mut colors = to_uppercase(&colors);
    short_strings(&mut colors);
    print_elements(&colors);
}
