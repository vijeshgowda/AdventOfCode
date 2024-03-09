use std::fs;
fn main() {
    println!("main function starts");

    let contents = fs::read_to_string("./input").expect("Should have been able to read the file");

    second_function(contents);
}

fn second_function(contents: String) {
    let name: &str = "--- day3 input ---";
    println!("{name}\n{contents}");
}
