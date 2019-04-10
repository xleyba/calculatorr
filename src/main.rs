#[macro_use] extern crate serde_derive;
extern crate num;
use num::bigint::BigInt;
use num::bigint::ToBigInt;
use actix_web::{App, Path, Result, http, server, HttpRequest};
use std::env;
use colored::*;



#[derive(Deserialize)]
struct Number {
    number: i32,
}

#[derive(Deserialize)]
struct Echo {
    message: String,
}


fn factorial_iterative(n: i32) -> BigInt {
    let mut result = 1.to_bigint().unwrap();
    for x in 1..=n {
        result = result * x.to_bigint().unwrap();
    }

    return result
}

fn factorial_recursive(n: i32) -> BigInt {
    if n <= 1 {
        return 1.to_bigint().unwrap()
    } else {
        return n.to_bigint().unwrap() * factorial_recursive(n - 1)
    }
}


fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

/// extract path info from "/users/{userid}/{friend}" url
/// {number} -  - deserializes to a u32
fn echo_handler(msg: Path<Echo>) -> Result<String> {
    println!("Entró ");
    Ok(format!("{}", msg.message))
}

/// extract path info from "/users/{userid}/{friend}" url
/// {number} -  - deserializes to a u32
fn factorial_iter_handler(number: Path<Number>) -> Result<String> {
    let n = number.number;
    Ok(format!("{}", factorial_iterative(n)))
}

/// extract path info from "/users/{userid}/{friend}" url
/// {number} -  - deserializes to a u32
fn factorial_recur_handler(number: Path<Number>) -> Result<String> {
    let n = number.number;
    Ok(format!("{}", factorial_recursive(n)))
}

fn main() {

    let mut workers: usize = 2;
    let mut port = 9596;

    println!("{}", "===========================================================".yellow().bold());
    println!("{}", "                    Calculator v 0.1.0".yellow().bold());
    println!("{}", "===========================================================".yellow().bold());
    println!("{}", "   Please use env variables for configuration:".yellow().bold());
    println!("{}", "       CALC_PORT=port number".yellow().bold());
    println!("{}", "       CALC_WORKERS=workers for server".yellow().bold());
    println!("{}", "-----------------------------------------------------------");
    println!("Starting configuration......\n");

    let key = "CALC_PORT";
    println!("- Port:");
    match env::var(key) {
        Ok(val) => {
            println!("... Config variable?:  {}", "exists!".green());
            match val.as_str().parse::<u16>() {
                Ok(n) => {
                    println!("... Valid?:            {}", "Yes".to_string().green());
                    println!("... Port set to:       {}", n.to_string().green());
                    port = n;
                },
                Err(e) => {
                    println!("... Valid?:            {} - {}", "No".red(), e.to_string().red());
                    println!("... Port set to:       {} - (by default)", "9596".green());
                },
            }
        }
        Err(_e) => {
            println!("... Config variable?:  {}", "No".red());
            println!("... Port set to:       {} - (by default)", "9596".green());
        }
    } 

    let key = "CALC_WORKERS";
    println!("\n- Workers:");
    match env::var(key) {
        Ok(val) => {
            println!("... Config variable?:  {}", "exists!".green());
            match val.as_str().parse::<usize>() {
                Ok(n) => {
                    println!("... Valid?:            {}", "Yes".to_string().green());
                    println!("... Workers set to:    {}", n.to_string().green());
                    workers = n;
                },
                Err(e) => {
                    println!("... Valid?:            {} - {}", "No".red(), e.to_string().red());
                    println!("... Workers set to:    {} - (by default)", "2".green());
                },
            }
        }
        Err(_e) => {
            println!("... Config variable?:  {}", "No".red());
            println!("... Workers set to:    {} - (by default)", "2".green());
        },
    } 

    println!("{}", "-----------------------------------------------------------");
    println!("Starting server.... Press Ctrl-C to stop it.");


    server::new(|| {App::new()
        .resource("/", |r| r.f(index))
        .resource("/echo/{message}",                        // <- define path parameters
            |r| r.method(http::Method::GET).with(echo_handler))
        .resource("/factorialIterative/{number}",                        // <- define path parameters
            |r| r.method(http::Method::GET).with(factorial_iter_handler))
        .resource("/factorialRecursive/{number}",                        // <- define path parameters
            |r| r.method(http::Method::GET).with(factorial_recur_handler))
    })
    .bind(format!("{}{}", "127.0.0.1:", port))
    .unwrap()
    .workers(workers)
    .run();
}