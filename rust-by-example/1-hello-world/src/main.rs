use std::fmt::{Display, write};

// Debug traits
#[derive(Debug)]
struct SomeData {
    name: String,
    age: u16,
    description: String,
}

impl Display for SomeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, age: {}, {}", self.name, self.age, self.description)
    }
}



struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {write!(f, ", ")?;}
            
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}
#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RGB ({red}, {green}, {blue}) 0x{red:0>2X}{green:0>2X}{blue:0>2X}", red=self.red, green=self.green, blue=self.blue)
    }
}

fn main() {
    /**
     * Multi line comments are fun too!
     */

    // A single line comment?
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    println!("Lmao Yeet {}", 31);
    println!("Numbered {0} {1} {2}", "a", "b", "c");
    println!(
        "Namespaced {subject} {verb} {noun}",
        subject = "Theirs",
        verb = "Doing",
        noun = "Something"
    );

    // Padding is fun
    println!("{number:0<5}", number = 1);
    println!("{number:0<pad$}", number = 3, pad = 12);

    let pi = 3.141592;
    println!("Pi ixs roughly {pi:.3}");

    // Debug
    println!(
        "{0:#?}",
        SomeData {
            name: String::from("Hello"),
            age: 42,
            description: String::from("Some person")
        }
    );

    // Display
    println!(
        "{0}",
        SomeData {
            name: String::from("Hello"),
            age: 42,
            description: String::from("Some person")
        }
    );

    let v = List(vec![1,2,3]);
    println!("{v}");

    // Formatting
    println!("{:#?}", Color {red: 12, green: 12, blue: 255});
    println!("{}", Color {red: 0, green: 3, blue: 254});
}
