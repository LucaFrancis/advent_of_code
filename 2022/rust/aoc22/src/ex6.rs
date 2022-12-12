use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use crate::Exercise;

use crate::file_utils::read_file;

pub(crate) fn get_ex6() -> Exercise {
    Exercise {
        id: 6,
        first_part: exercise6_1,
        second_part: exercise6_2,
    }
}

fn exercise6_1() {
    for line in read_file("../../inputs/6/input").split("\n") {
        let mut deq: VecDeque<char> = VecDeque::new();
        let mut marker_position = 4;
        for char in line.chars() {
            if deq.len() < 4 {
                deq.push_back(char);
                continue;
            }
            //check if marker
            if has_unique_elements(deq.iter()) {
                break;
            }
            // cycle chars
            deq.push_back(char);
            deq.pop_front();
            marker_position += 1;

        }
        println!("{}", marker_position);
    };

}

fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn exercise6_2() {
    for line in read_file("../../inputs/6/input").split("\n") {
        let mut deq: VecDeque<char> = VecDeque::new();
        let mut marker_position = 14;
        for char in line.chars() {
            if deq.len() < 14 {
                deq.push_back(char);
                continue;
            }
            //check if marker
            if has_unique_elements(deq.iter()) {
                break;
            }
            // cycle chars
            deq.push_back(char);
            deq.pop_front();
            marker_position += 1;

        }
        println!("{}", marker_position);
    };
}