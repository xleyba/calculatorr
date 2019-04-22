
use actix_web::{Result};
use actix_web::web::Path;
use rug::{Integer};


// Var for factorial handlers
#[derive(Deserialize)]
pub struct Number {
    number: u32,
}

// Var for echo handlers
#[derive(Deserialize)]
pub struct Echo {
    message: String,
}

// Calculates factorial iteratively
fn factorial_iterative(n: u32) -> Integer {
    let mut result = Integer::from(n);
    for x in 2..n {
        result = result * x;
    }
    return result
}

// Calculates factorial recursively
fn factorial_recursive(n: u32) -> Integer {
    if n <= 1 {
        return Integer::from(1)
    } else {
        return Integer::from(n) * factorial_recursive(n - 1)
    }
}

// Handle index route
pub fn index() -> &'static str {
    "Hello world!\r\n"
}  

/// extract path info from "/users/{userid}/{friend}" url
/// {number} -  - deserializes to a u32
pub fn echo_handler(msg: Path<Echo>) -> Result<String> {
    let message = msg.message.clone();
    debug!("Received message: {}", message);
    Ok(format!("{}", message))
}

/// extract path info from "/users/{userid}/{friend}" url
/// {number} -  - deserializes to a u32
pub fn factorial_iter_handler(number: Path<Number>) -> Result<String> {
    let n = number.number;
    debug!("Received number: {}", n);
    Ok(format!("{}", factorial_iterative(n)))
}

/// extract path info from "/users/{userid}/{friend}" url
/// {number} -  - deserializes to a u32
pub fn factorial_recur_handler(number: Path<Number>) -> Result<String> {
    let n = number.number;
    debug!("Received number: {}", n);
    Ok(format!("{}", factorial_recursive(n)))
}