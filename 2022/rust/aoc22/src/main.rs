extern crate core;

use std::io;
use std::io::prelude::*;
use crate::ex1::exercise1;

mod ex1;

fn main() {
    println!("Choose excercise (1-1): ");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                if line == "1" {
                    exercise1();
                    return;
                }
                println!("Choose excercise (1-1): ")
            }
            Err(_line) => {
                return;
            }
        }
    }

    exercise1()
}
