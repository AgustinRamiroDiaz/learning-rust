use std::{
    error::Error,
    fmt::{self, Display, Formatter},
}; // imports with the "use" keyword

#[derive(Debug)]
struct ErrorA {}

impl Display for ErrorA {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "ErrorA")
    }
}

impl Error for ErrorA {} // automatically implements the Error trait for ErrorA given that it implements Display and Debug

#[derive(Debug)]
struct ErrorB {}

impl Display for ErrorB {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "ErrorB")
    }
}

impl Error for ErrorB {}

// static dispatch (compile-time)
fn error_selector_wrong(letter: char) -> Option<impl Error> {
    match letter {
        'a' => Some(ErrorA {}), // tries to compile with output type ErrorA
        'b' => Some(ErrorB {}), // tries to compile with output type ErrorB
        _ => None,
    }
}

// dynamic dispatch (runtime)
// this has a performance penalty at runtime
fn error_selector(letter: char) -> Option<Box<dyn Error>> {
    // we need to "Box" is so that the compiler knows the size of the return type at compile time
    match letter {
        'a' => Some(Box::new(ErrorA {})),
        'b' => Some(Box::new(ErrorB {})),
        _ => None,
    }
}

// showing some useful methods around options and errors
fn error_manipulation() -> Result<(), Box<dyn Error>> {
    error_selector('a').ok_or(println!("No error"))?; // the "?" operator is a shortcut for "match" and "return" in a single line

    Ok(())
}

// Bonus: implementing traits

trait Waver {
    fn wave(&self) -> String;
}

// you can implement your traits for types you don't own types
impl Waver for i32 {
    fn wave(&self) -> String {
        format!("Hi, I'm {}", self)
    }
}

fn main() {
    4.wave() // "Hi, I'm 4"
}

impl Error for i32 {} // Orphan rules: you can't implement a trait for a type you don't own
