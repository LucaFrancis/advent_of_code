extern crate core;

use std::io;
use std::io::prelude::*;

use crate::ex1::{exercise1_1, exercise1_2};
use crate::ex2::{exercise2_1, exercise2_2};
use crate::ex3::{exercise3_1, exercise3_2};
use crate::ex4::{exercise4_1, exercise4_2};
use crate::ex5::{exercise5_1, exercise5_2};
use crate::ex6::{exercise6_1, exercise6_2};

mod ex1;
mod ex2;
mod file_utils;
mod ex3;
mod ex4;
mod ex5;
mod ex6;

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
                    "3.1" => { exercise3_1(); }
                    "3.2" => { exercise3_2(); }
                    "4.1" => { exercise4_1(); }
                    "4.2" => { exercise4_2(); }
                    "5.1" => { exercise5_1(); }
                    "5.2" => { exercise5_2(); }
                    "6.1" => { exercise6_1(); }
                    "6.2" => { exercise6_2(); }
                    _ => {}
                }
            }
            Err(why) => {
                panic!("{}", why);
            }
        }
    }
}
