use std::io;

fn main() {
    println!("Generate the n:th Fibonacci number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let i: i32 = input
        .trim()
        .parse()
        .expect("Please type a valid number!");

    let x = fibonacci(i);
    println!("Here is the {i}th fibonacci number: {x}");
}

fn fibonacci(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }

    let mut f1 = 0;
    let mut f2 = 1;
    for _ in 1..n {
        let next = f1 + f2;
        f1 = f2;
        f2 = next;
    }
    f2
}

