extern crate core;

use std::io;
use std::io::prelude::*;

use crate::ex1::{exercise1_1, exercise1_2};
use crate::ex2::{exercise2_1, exercise2_2};

mod ex1;
mod ex2;
mod file_utils;

macro_rules! add_one {
    ($input:expr) => {
        $input + 1
    }
}

fn main() {
    println!("{}", add_one!(1));
    let stdin = io::stdin();
    println!("Choose exercise (day.round, e.g. 2.1): ");
    let lines = stdin.lock();
    for line in lines.lines() {
        match line {
            Ok(line) => {
                match line.as_str() {
                    "1.1" => { exercise1_1(); }
                    "1.2" => { exercise1_2(); }
                    "2.1" => { exercise2_1(); }
                    "2.2" => { exercise2_2(); }
                    _ => {}
                }
            }
            Err(why) => {
                panic!("{}", why);
            }
        }
    }
}
