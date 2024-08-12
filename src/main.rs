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

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el|
            el
                .chars()
                .map(|c| c.to_string())
                .collect()
        )
        .collect::<Vec<Vec<String>>>()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(fallback), |el| el.to_string())
}

fn main() {
    let original = vec![String::from("red"), String::from("green"), String::from("blue")];
    let mut colors = vec![];
    move_elements(original, &mut colors);
    let mut colors = to_uppercase(&colors);
    short_strings(&mut colors);
    print_elements(&colors);

    let exploded = explode(&colors);
    println!("{:#?}", exploded);

    let found_color = find_color_or(&colors, "R", "Orange");
    println!("{:#?}", found_color);
}
