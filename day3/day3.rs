use std::fs;
fn main() {
    println!("main function starts");

    let contents = fs::read_to_string("./input").expect("Should have been able to read the file");

    let sum = sum_adjacent_numbers(contents);
    println!("Sum of part numbers: {}", sum);
}

fn sum_adjacent_numbers(schematic: String) -> i32 {
    let sum = 0;

    let mut rows: Vec<&str> = schematic.trim().split('\n').collect();

    let mut rows_main: Vec<&str> = Vec::new();

    for row in rows.iter() {
        rows_main.push(row.trim_end_matches('\r'));
    }

    for i in 0..rows.len() {
        println!("the vector at {i} is - {:}", rows[i]);
    }

    rows = rows_main.clone();

    let parsed_rows: Vec<Vec<i32>> = rows
        .iter()
        .map(|row| {
            row.split('.')
                .filter_map(|substr| substr.parse::<i32>().ok())
                .collect()
        })
        .collect();

    println!("\nthe full vector is \n {:#?}", rows);
    println!("\nthe parsed vector is \n {:?}", parsed_rows);

    println!("\nthe cleaned vector is \n {:#?}", rows_main);

    println!(
        "\nthe test character is -- {:?}",
        rows_main[0].chars().nth(9)
    );

    println!("length of vector: {}", rows.len());

    sum
}
