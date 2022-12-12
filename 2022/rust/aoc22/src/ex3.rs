use crate::Exercise;
use crate::file_utils::read_file;

pub(crate) fn get_ex3() -> Exercise {
    Exercise {
        id: 3,
        first_part: exercise3_1,
        second_part: exercise3_2,
    }
}

fn exercise3_1() {
    let mut score: u32 = 0;
    for line in read_file("../../inputs/3/input").split("\n") {
        let (first_inventory, second_inventory) = line.split_at(line.len() / 2);
        for char in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            if first_inventory.contains(char) {
                if second_inventory.contains(char) {
                    let char_prio = if char.is_uppercase() { (char as u32) - 38 } else { (char as u32) - 96 };
                    score += char_prio;
                    break;
                }
            }
        }
    };
    println!("{}", score)
}

fn exercise3_2() {
    let mut score: u32 = 0;
    let file =  read_file("../../inputs/3/input");
    let inventories: Vec<&str> = file.split("\n").collect();
    let length = inventories.len();
    let mut iter = inventories.iter();
    for _ in 0..length/3 {
        let [first, second, third] = [
            iter.next_back().unwrap(),
            iter.next_back().unwrap(),
            iter.next_back().unwrap(),
        ];
        for char in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            if first.contains(char) {
                if second.contains(char) {
                    if third.contains(char) {
                        let char_prio = if char.is_uppercase() { (char as u32) - 38 } else { (char as u32) - 96 };
                        score += char_prio;
                        break;
                    }
                }
            }
        }
    };
    println!("{}", score)
}