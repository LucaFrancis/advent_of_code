use crate::Exercise;
use std::cmp::{max, min};

use crate::file_utils::read_file;

pub(crate) fn get_ex9() -> Exercise {
    Exercise {
        id: 9,
        first_part: exercise9_1,
        second_part: exercise9_2,
    }
}

fn exercise9_1() {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut count = 0;
    for instruction in read_file("../../inputs/9/input").split('\n') {
        let parts = instruction.split(' ').collect::<Vec<&str>>();
        let direction = parts[0];
        let steps = parts[1];

        for _ in 0..steps.parse().unwrap() {
            head = get_head_position(head, direction);
            tail = get_tail_position(head, tail);
            if !visited.contains(&tail) {
                visited.push(tail);
                count += 1;
            }
        }
    }
    print_field(head, Vec::from([tail]), visited.clone());
    println!("{}", count);
}

fn print_field(head: (i32, i32), tails: Vec<(i32, i32)>, visited: Vec<(i32, i32)>) {
    let visited_max = visited.iter().map(|location| location.0).max().unwrap_or(0);
    let visited_min = visited.iter().map(|location| location.0).min().unwrap_or(0);
    let max_x = max(visited_max, head.0);
    let max_y = max(visited_max, head.1);
    let min_x = min(visited_min, head.0);
    let min_y = min(visited_min, head.1);
    for y in (min_y - 5..max_y + 5).rev().collect::<Vec<_>>() {
        for x in min_x - 5..max_x + 5 {
            let tail = tails.iter().position(|&tail| tail == (x, y));
            if (x, y) == head {
                print!("H")
            } else if tail.is_some() {
                print!("{}", tail.unwrap() + 1);
            } else if (x, y) == (0, 0) {
                print!("s");
            } else if visited.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
    println!()
}

fn get_tail_position(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let x_distance = (head.0 - tail.0).abs();
    let y_distance = (head.1 - tail.1).abs();
    if x_distance > 1 && y_distance == 0 {
        // Move horizontal
        return if head.0 > tail.0 {
            (tail.0 + 1, tail.1)
        } else {
            (tail.0 - 1, tail.1)
        };
    }
    if y_distance > 1 && x_distance == 0 {
        // Move vertical
        return if head.1 > tail.1 {
            (tail.0, tail.1 + 1)
        } else {
            (tail.0, tail.1 - 1)
        };
    }
    if (x_distance == 2 && (y_distance == 1 || y_distance == 2))
        || (y_distance == 2 && (x_distance == 1 || x_distance == 2))
    {
        // Move diagonally
        return if head.0 < tail.0 {
            if head.1 < tail.1 {
                (tail.0 - 1, tail.1 - 1)
            } else {
                (tail.0 - 1, tail.1 + 1)
            }
        } else if head.1 < tail.1 {
            (tail.0 + 1, tail.1 - 1)
        } else {
            (tail.0 + 1, tail.1 + 1)
        };
    }
    if x_distance <= 1 && y_distance <= 1 {
        return tail;
    }
    panic!()
}

fn get_head_position(head: (i32, i32), direction: &str) -> (i32, i32) {
    match direction {
        "U" => {
            //UP
            (head.0, head.1 + 1)
        }
        "D" => {
            //DOWN
            (head.0, head.1 - 1)
        }
        "L" => {
            //LEFT
            (head.0 - 1, head.1)
        }
        "R" => {
            //RIGHT
            (head.0 + 1, head.1)
        }
        &_ => head,
    }
}

fn exercise9_2() {
    let mut head = (0, 0);
    let mut tails = [(0, 0)].repeat(9);
    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut count = 0;
    for instruction in read_file("../../inputs/9/input").split('\n') {
        let parts = instruction.split(' ').collect::<Vec<&str>>();
        let direction = parts[0];
        let steps = parts[1];
        for _ in 0..steps.parse().unwrap() {
            head = get_head_position(head, direction);
            for tail_index in 0..tails.len() {
                let previous = if tail_index == 0 {
                    head
                } else {
                    tails[tail_index - 1]
                };
                tails[tail_index] = get_tail_position(previous, tails[tail_index]);
                if tail_index == tails.len() - 1 && !visited.contains(&tails[tail_index]) {
                    visited.push(tails[tail_index]);
                    count += 1;
                }
            }
        }
    }
    print_field(head, tails, visited);
    println!("{}", count);
}
