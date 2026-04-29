fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));

        println!("And also, {num} plus random is {}!", add_rand::add_rand(num));

}
