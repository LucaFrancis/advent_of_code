extern crate core;

use std::io;
use std::io::prelude::*;
use crate::ex10::get_ex10;
use crate::ex11::get_ex11;
use crate::ex12::get_ex12;
use crate::ex1::get_ex1;
use crate::ex2::get_ex2;
use crate::ex3::get_ex3;
use crate::ex4::get_ex4;
use crate::ex5::get_ex5;
use crate::ex6::get_ex6;
use crate::ex7::get_ex7;
use crate::ex8::get_ex8;
use crate::ex9::get_ex9;


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
mod ex12;

pub(crate) struct Exercise {
    id: i32,
    first_part: fn() -> (),
    second_part: fn() -> (),
}

fn main() {
    let exercises = vec![
        get_ex1(),
        get_ex2(),
        get_ex3(),
        get_ex4(),
        get_ex5(),
        get_ex6(),
        get_ex7(),
        get_ex8(),
        get_ex9(),
        get_ex10(),
        get_ex11(),
        get_ex12()
    ];
    let stdin = io::stdin();
    println!("Choose exercise (day.round, e.g. 2.1): ");
    let lines = stdin.lock();
    for line in lines.lines() {
        match line {
            Ok(line) => {
                let split = line.split('.').collect::<Vec<&str>>();
                if split.len() == 2 {
                    let exercise: i32 = match split[0].parse() {
                        Ok(s) => { s }
                        Err(_) => { continue; }
                    };
                    let part: usize = match split[1].parse() {
                        Ok(s) => { s }
                        Err(_) => { continue; }
                    };
                    match exercises.iter().find(|e| e.id == exercise) {
                        None => {}
                        Some(ex) => {
                            if part == 1 {
                                (ex.first_part)()
                            }
                            if part == 2 {
                                (ex.second_part)()
                            }
                            continue;
                        }
                    }
                }
            }
            Err(why) => {
                panic!("{}", why);
            }
        }
    }
}
