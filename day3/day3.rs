use std::fs;
fn main() {
    println!("main function starts");

    let contents = fs::read_to_string("./input").expect("Should have been able to read the file");

    let sum = sum_adjacent_numbers(contents);
    println!("Sum of part numbers: {}", sum);
}

fn sum_adjacent_numbers(schematic: String) -> i32 {
    let mut sum = 0;

    let rows: Vec<&str> = schematic.trim().split('\n').collect();

    println!("the vector \n {:?}", rows);
    sum += 2;
    println!("length of vector: {}", rows.len());

    sum
}
