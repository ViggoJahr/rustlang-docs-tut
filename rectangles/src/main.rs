#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width:30, 
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    // att skriva: rect1:? ger en oneliner på struct, medan :#? delar upp den.
    println!("rect1 is {rect1:#?}"); 


    // Om man vill använda dbg! istället så tar den ownership istället för refernece som prinln! gör:

    let scale: u32 = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    //We don’t want dbg! to take ownership of rect1, so we use a reference to rect1
    dbg!(&rect2);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}