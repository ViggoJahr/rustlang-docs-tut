use rand::Rng;

pub fn add_rand(x: i32) -> i32 {
    let mut rng = rand::thread_rng(); 
    
    // Generate a random number ( between 1 and 100) and add it
    x + rng.gen_range(1..=100) 
}