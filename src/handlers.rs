use actix_web::web::Path;
use actix_web::Result;
use actix_web::test::TestRequest;
use actix_web::test;
use actix_web::http::StatusCode;
use actix_web::FromRequest;
use rug::Integer;

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
    return result;
}

// Calculates factorial recursively
fn factorial_recursive(n: u32) -> Integer {
    if n <= 1 {
        return Integer::from(1);
    } else {
        return Integer::from(n) * factorial_recursive(n - 1);
    }
}

// Handle index route
pub fn index() -> &'static str {
    "Hello world!\r\n"
}

/// extract path info from "/echo/{message}" url
/// {message} -  - deserializes to a String
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_factorial_iterative() {
        assert_eq!(factorial_iterative(5), 120);
    }

    #[test]
    fn test_factorial_recursive() {
        assert_eq!(factorial_recursive(5), 120);
    }

    #[test]
    fn test_index() {
        assert_eq!(index(), "Hello world!\r\n");

    }

    #[test]
    fn test_echo_handler() {
        let req = TestRequest::default().param("message", "hello").to_http_request(); 
        let fut = Path::<Echo>::extract(&req).and_then(echo_handler); 
        assert_eq!(test::block_on(fut).unwrap(), "hello");
    }

}
