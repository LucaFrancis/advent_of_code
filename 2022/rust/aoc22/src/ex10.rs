use crate::Exercise;
use crate::file_utils::read_file;

pub(crate) fn get_ex10() -> Exercise {
    Exercise {
        id: 10,
        first_part: exercise10_1,
        second_part: exercise10_2,
    }
}

fn exercise10_1() {
    let mut cycle: i32 = 1;
    let mut x: i32 = 1;
    let mut signal = 0;
    for instruction in read_file("../../inputs/10/input").split('\n') {
        if instruction == "noop" {
            signal = get_signal(signal, cycle, x);
            cycle += 1;
        } else {
            let value = instruction.split(' ').last().unwrap();
            signal = get_signal(signal, cycle, x);
            cycle += 1;
            signal = get_signal(signal, cycle, x);
            x += value.parse::<i32>().unwrap();
            cycle += 1;
        }
    }
    println!("{}", signal);
}

fn update_ctr(position: i32, x: i32) -> i32 {
    if is_visible(position, x) {
        print!("#")
    } else {
        print!(".")
    }
    update_position(position)
}

fn is_visible(position: i32, x: i32) -> bool {
    vec![x-1, x, x+1].contains(&position)
}

fn update_position(position: i32) -> i32 {
    if position == 39 {
        println!();
        0
    } else {
        position + 1
    }
}

fn get_signal(signal: i32, cycle: i32, x: i32) -> i32 {
    if vec![20, 60, 100, 140, 180, 220].contains(&cycle) {
        return signal + cycle * x;
    }
    signal
}

fn exercise10_2() {
    let mut cycle: i32 = 1;
    let mut x: i32 = 1;
    let mut signal = 0;
    let mut position = 0;
    for instruction in read_file("../../inputs/10/input").split('\n') {
        if instruction == "noop" {
            position = update_ctr(position, x);
            signal = get_signal(signal, cycle, x);
            cycle += 1;
        } else {
            let value = instruction.split(' ').last().unwrap();
            position = update_ctr(position, x);
            signal = get_signal(signal, cycle, x);
            cycle += 1;
            position = update_ctr(position, x);
            signal = get_signal(signal, cycle, x);
            x += value.parse::<i32>().unwrap();
            cycle += 1;
        }
    }
}