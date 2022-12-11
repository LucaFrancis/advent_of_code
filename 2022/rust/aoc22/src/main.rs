extern crate core;

use std::io;
use std::io::prelude::*;

use crate::ex1::{exercise1_1, exercise1_2};
use crate::ex10::{exercise10_1, exercise10_2};
use crate::ex11::{exercise11_1, exercise11_2};
use crate::ex2::{exercise2_1, exercise2_2};
use crate::ex3::{exercise3_1, exercise3_2};
use crate::ex4::{exercise4_1, exercise4_2};
use crate::ex5::{exercise5_1, exercise5_2};
use crate::ex6::{exercise6_1, exercise6_2};
use crate::ex7::{exercise7_1, exercise7_2};
use crate::ex8::{exercise8_1, exercise8_2};
use crate::ex9::{exercise9_1, exercise9_2};

mod ex1;
mod ex2;
mod file_utils;
mod ex3;
mod ex4;
mod ex5;
mod ex6;
mod ex7;
mod ex8;
mod ex9;
mod ex10;
mod ex11;

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
                    "7.1" => { exercise7_1(); }
                    "7.2" => { exercise7_2(); }
                    "8.1" => { exercise8_1(); }
                    "8.2" => { exercise8_2(); }
                    "9.1" => { exercise9_1(); }
                    "9.2" => { exercise9_2(); }
                    "10.1" => { exercise10_1(); }
                    "10.2" => { exercise10_2(); }
                    "11.1" => { exercise11_1(); }
                    "11.2" => { exercise11_2(); }
                    _ => {}
                }
            }
            Err(why) => {
                panic!("{}", why);
            }
        }
    }
}
