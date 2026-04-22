use std::{collections::HashMap, io};

fn main() {
    println!("Write a sequence of integers (e.g., 1 5 10):");

    let numbers: Vec<i32>; // The variable that will hold the final list

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let result: Result<Vec<i32>, _> = input
            .split_whitespace()
            .map(|word| word.parse::<i32>())
            .collect();

        match result {
            Ok(list) => {
                if list.is_empty() { 
                    println!("The list is empty!");
                    continue;
                }
                numbers = list; // Assign the list to numbers
                break; // exit loop - creation of list went well.
            }
            Err(_) => {
                println!("That's not a valid sequence of integers");
            }
        }
    }

    println!("here is the list of numbers: {:?}", numbers);
    println!("here is the median: {:?}", get_median(&numbers[..]));
    println!("here is the count of each value: {:#?}", get_mode(&numbers[..]));


}

fn get_median(numbers: &[i32])-> f64 {
    let mut sorted = numbers.to_vec();
    sorted.sort_unstable();

    let len: usize  = sorted.len(); 
    let mid = len / 2;
    
    match len {
        0 => 0.0, // returns 0 if no numbers in list.
        n if n % 2 == 0 => (sorted[mid - 1] + sorted[mid]) as f64 / 2.0,
        _ => sorted[mid] as f64,
    }
}

fn get_mode(numbers: &[i32])  -> HashMap<i32, i32> {
    let mut mode: HashMap<i32, i32> = HashMap::new();
    for element in numbers {
        let count = mode.entry(*element).or_insert(0);
        *count += 1;
    }
    mode
}
