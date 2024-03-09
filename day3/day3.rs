use std::fs;
fn main() {
    println!("hello world");

    let contents = fs::read_to_string("./input").expect("Should have been able to read the file");

    second_function(contents);
}

fn second_function(contents: String) {
    // let name: &str = "some name";
    println!("{contents}");
}
