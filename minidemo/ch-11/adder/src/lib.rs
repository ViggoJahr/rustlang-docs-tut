// ---- Rektangel test - Kan en rektangel hålla en annan
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.width
    }
}

// ---- Funktion som adderar 2 till ett element
pub fn add_two(a: u64) -> u64 {
    a + 2
}

// ---- Funktion som ska säga: Hello, 'name'!
// Inlagd bugg -> String borde returnera namnet på personen
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

// ---- Gissnings tests, ger den rätt output?
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 6,
            height: 4
        };
        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 6,
            height: 4
        };
        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

        #[test]
    fn it_works() -> Result<(), String> {
        let result = add_two(1);

        if result == 3 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }


    #[ignore] // Detta gör att man inte kör testet.
    #[test] 
    fn another() {
        panic!("this test fails");
    }
    
    #[ignore] 
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    #[test]
    #[should_panic] // Ger 'ok' om den ger en panic.
    fn greater_than_100() {
        Guess::new(200);
    }
}
