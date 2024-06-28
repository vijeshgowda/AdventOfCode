use std::fs;

fn main() {
    println!("main function starts");

    let contents = fs::read_to_string("./input").expect("Should have been able to read the file");

    let sum = sum_adjacent_numbers(contents);
    println!("Sum of part numbers: {}", sum);
}

fn is_symbol(c: char) -> bool {
    let symbols = vec![
        '*', '#', '$', '%', '@', '!', '$', '%', '^', '&', '*', '+', '-', '=', '<', '>', '/',
    ];
    if c != '.' && !symbols.contains(&c) {
        println!("character is_symbol --> {:?}", c);
    }
    symbols.contains(&c)
}

fn sum_adjacent_numbers(schematic: String) -> i32 {
    let mut sum = 0;

    let mut rows: Vec<&str> = schematic.trim().split('\n').collect();

    let mut rows_main: Vec<&str> = Vec::new();

    for row in rows.iter() {
        rows_main.push(row.trim_end_matches('\r'));
    }

    // for i in 0..rows.len() {
    //     println!("the vector at {i} is - {:}", rows[i]);
    // }

    rows = rows_main.clone();

    // let parsed_rows: Vec<Vec<i32>> = rows
    //     .iter()
    //     .map(|row| {
    //         row.split('.')
    //             .filter_map(|substr| substr.parse::<i32>().ok())
    //             .collect()
    //     })
    //     .collect();

    // let parsed_rows: Vec<i32> = rows
    //     .iter()
    //     .flat_map(|row| {
    //         row.split('.')
    //             .filter_map(|substr| substr.parse::<i32>().ok())
    //     })
    //     .collect();

    let mut sum_numbers: Vec<i32> = vec![];

    let mut check = false;

    for i in 0..rows.len() {
        let mut start_point: i32 = -1;
        let mut end_point: i32 = -1;

        for j in 0..rows[i].len() {
            let _length_of_row = rows[i].len();

            if rows[i].chars().nth(j).unwrap_or(' ').is_digit(10) {
                if start_point == -1 && end_point == -1 {
                    start_point = j as i32;
                }
                if start_point != -1
                    && end_point == -1
                    && !rows[i].chars().nth(j + 1).unwrap_or(' ').is_digit(10)
                {
                    end_point = j as i32;
                }
            }

            if start_point != -1 && end_point != -1 {
                let a = start_point as usize;
                let b = end_point as usize;

                //println!("row {}: a = {}, b = {}", i, a, b);

                if i > 0 && i < rows.len() - 1 && a > 0 && b < rows[i].len() - 1 {
                    if is_symbol(rows[i].chars().nth(b + 1).unwrap_or(' '))
                        || is_symbol(rows[i].chars().nth(a - 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a - 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(b + 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a + 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(b).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(b + 1).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a - 1).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(b).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a + 1).unwrap_or(' '))
                    {
                        let substring = rows[i][a..b + 1].parse::<i32>().ok();
                        match substring {
                            Some(value) => {
                                // value contains the parsed i32, push it to the vector
                                sum_numbers.push(value);
                                check = true;
                                // println!("the substring is {:#?}", value);
                            }
                            None => {
                                // Handle the case where parsing failed (e.g., print message)
                                println!("Error: Could not parse substring to i32 (skipping) -- 1 -- at row {} substring {} - {}", i+1, a, b);
                            }
                        }
                    }
                }

                if i > 0 && i < rows.len() - 1 && a == 0 && b < rows[i].len() - 1 {
                    if is_symbol(rows[i].chars().nth(b + 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(b + 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a + 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(b).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(b + 1).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(b).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a + 1).unwrap_or(' '))
                    {
                        let substring = rows[i][a..b + 1].parse::<i32>().ok();
                        match substring {
                            Some(value) => {
                                // value contains the parsed i32, push it to the vector
                                sum_numbers.push(value);
                                check = true;
                                // println!("the substring is {:#?}", value);
                            }
                            None => {
                                // Handle the case where parsing failed (e.g., print message)
                                println!("Error: Could not parse substring to i32 (skipping) -- 2");
                            }
                        }
                    }
                }

                if i > 0 && i < rows.len() - 1 && a > 0 && b == rows[i].len() - 1 {
                    if is_symbol(rows[i].chars().nth(a - 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a - 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a + 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(b).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a - 1).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(b).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a + 1).unwrap_or(' '))
                    {
                        let substring = rows[i][a..b + 1].parse::<i32>().ok();
                        match substring {
                            Some(value) => {
                                // value contains the parsed i32, push it to the vector
                                sum_numbers.push(value);
                                check = true;
                                // println!("the substring is {:#?}", value);
                            }
                            None => {
                                // Handle the case where parsing failed (e.g., print message)
                                println!("Error: Could not parse substring to i32 (skipping) -- 3");
                            }
                        }
                    }
                }

                if i == 0 && a > 0 && b < rows[i].len() - 1 {
                    if is_symbol(rows[i].chars().nth(b + 1).unwrap_or(' '))
                        || is_symbol(rows[i].chars().nth(a - 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a - 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(b + 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a + 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(b).unwrap_or(' '))
                    {
                        let substring = rows[i][a..b + 1].parse::<i32>().ok();
                        match substring {
                            Some(value) => {
                                // value contains the parsed i32, push it to the vector
                                sum_numbers.push(value);
                                check = true;
                                // println!("the substring is {:#?}", value);
                            }
                            None => {
                                // Handle the case where parsing failed (e.g., print message)
                                println!("Error: Could not parse substring to i32 (skipping) -- 4 -- at row {} substring {} - {}", i+1, a, b);
                            }
                        }
                    }
                }

                if i == 0 && a == 0 && b < rows[i].len() - 1 {
                    if is_symbol(rows[i].chars().nth(b + 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(b + 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a + 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(b).unwrap_or(' '))
                    {
                        let substring = rows[i][a..b + 1].parse::<i32>().ok();
                        match substring {
                            Some(value) => {
                                // value contains the parsed i32, push it to the vector
                                sum_numbers.push(value);
                                check = true;
                                // println!("the substring is {:#?}", value);
                            }
                            None => {
                                // Handle the case where parsing failed (e.g., print message)
                                println!("Error: Could not parse substring to i32 (skipping) -- 5");
                            }
                        }
                    }
                }

                if i == 0 && a > 0 && b == rows[i].len() - 1 {
                    if is_symbol(rows[i].chars().nth(a - 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a - 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a + 1).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(a).unwrap_or(' '))
                        || is_symbol(rows[i + 1].chars().nth(b).unwrap_or(' '))
                    {
                        let substring = rows[i][a..b + 1].parse::<i32>().ok();
                        match substring {
                            Some(value) => {
                                // value contains the parsed i32, push it to the vector
                                sum_numbers.push(value);
                                check = true;
                                // println!("the substring is {:#?}", value);
                            }
                            None => {
                                // Handle the case where parsing failed (e.g., print message)
                                println!("Error: Could not parse substring to i32 (skipping) -- 6");
                            }
                        }
                    }
                }

                if i == rows.len() - 1 && a > 0 && b < rows[i].len() - 1 {
                    if is_symbol(rows[i].chars().nth(b + 1).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a - 1).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(b + 1).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a + 1).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(b).unwrap_or(' '))
                    {
                        let substring = rows[i][a..b + 1].parse::<i32>().ok();
                        match substring {
                            Some(value) => {
                                // value contains the parsed i32, push it to the vector
                                sum_numbers.push(value);
                                check = true;
                                // println!("the substring is {:#?}", value);
                            }
                            None => {
                                // Handle the case where parsing failed (e.g., print message)
                                println!("Error: Could not parse substring to i32 (skipping) -- 7");
                            }
                        }
                    }
                }

                if i == rows.len() - 1 && a == 0 && b < rows[i].len() - 1 {
                    if is_symbol(rows[i].chars().nth(b + 1).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(b + 1).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a + 1).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(b).unwrap_or(' '))
                    {
                        let substring = rows[i][a..b + 1].parse::<i32>().ok();
                        match substring {
                            Some(value) => {
                                // value contains the parsed i32, push it to the vector
                                sum_numbers.push(value);
                                check = true;
                                // println!("the substring is {:#?}", value);
                            }
                            None => {
                                // Handle the case where parsing failed (e.g., print message)
                                println!("Error: Could not parse substring to i32 (skipping) -- 8");
                            }
                        }
                    }
                }

                if i == rows.len() - 1 && a > 0 && b == rows[i].len() - 1 {
                    if is_symbol(rows[i].chars().nth(a - 1).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a - 1).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a + 1).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(a).unwrap_or(' '))
                        || is_symbol(rows[i - 1].chars().nth(b).unwrap_or(' '))
                    {
                        let substring = rows[i][a..b + 1].parse::<i32>().ok();
                        match substring {
                            Some(value) => {
                                // value contains the parsed i32, push it to the vector
                                sum_numbers.push(value);
                                check = true;
                                // println!("the substring is {:#?}", value);
                            }
                            None => {
                                // Handle the case where parsing failed (e.g., print message)
                                println!("Error: Could not parse substring to i32 (skipping) -- 9");
                            }
                        }
                    }
                }

                if !check {
                    // println!(
                    //     " i = {}, a = {}, b = {}, the substring = {:?}",
                    //     i + 1,
                    //     a + 1,
                    //     b + 1,
                    //     rows[i][a..b + 1].parse::<i32>().ok()
                    // );
                }

                start_point = -1;
                end_point = -1;
                check = false;
            }
        }
    }

    // println!("\nthe full vector is \n {:#?}", rows);
    // println!("\nthe parsed vector is \n {:?}", parsed_rows);

    println!(
        "\nthe test character is -- {:?}",
        rows[1].chars().nth(3).unwrap_or(' ')
    );

    println!("length of vector: {}", rows.len());
    if sum == 0 {
        sum = sum_numbers.iter().sum::<i32>();
    }

    // println!("the sum_numbers {:?}", sum_numbers);

    sum
}
