use std::collections::HashMap;

use crate::Exercise;
use crate::file_utils::read_file;

pub(crate) fn get_ex2() -> Exercise {
    Exercise {
        id: 2,
        first_part: exercise2_1,
        second_part: exercise2_2,
    }
}

fn exercise2_1() {
    let mapping = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
        ("A", 1),
        ("B", 2),
        ("C", 3)
    ]);
    let text = read_file("../../inputs/2/input");
    let lines = text.split("\n");
    let mut score = 0;
    for line in lines {
        let moves: Vec<&str> = line.split(" ").collect();
        let enemy = match mapping.get(moves[0]) {
            None => { panic!("could not decipher enemy move") }
            Some(value) => { value }
        };
        let me = match mapping.get(moves[1]) {
            None => { panic!("could not decipher my move") }
            Some(value) => { value }
        };
        score += *me + get_score(*enemy, *me);
    }
    println!("{}", score)
}

fn exercise2_2() {
    let mapping = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
        ("A", 1),
        ("B", 2),
        ("C", 3)
    ]);
    let text = read_file("../../inputs/2/input");
    let lines = text.split("\n");
    let mut score = 0;
    for line in lines {
        let moves: Vec<&str> = line.split(" ").collect();
        let enemy = match mapping.get(moves[0]) {
            None => { panic!("could not decipher enemy move") }
            Some(value) => { value }
        };
        let me = match mapping.get(moves[1]) {
            None => { panic!("could not decipher my move") }
            Some(value) => { value }
        };
        let action = get_action(*enemy, *me);
        score += action + get_score(*enemy, action);
    }
    println!("{}", score)
}

fn get_score(enemy: i32, me: i32) -> i32 {
    let index: usize = ((me - 1) - (enemy - 1)).rem_euclid(3) as usize;
    return [3, 6, 0][index];
}

fn get_action(enemy: i32, guide: i32) -> i32 {
    let index: usize = ((guide - 1) + (enemy + 1)).rem_euclid(3) as usize;
    return [1, 2, 3][index];
}
