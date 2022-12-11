use std::cmp::Ordering::{Greater, Less};
use std::num::Wrapping;

use crate::file_utils::read_file;

#[derive(Clone)]
struct Monkey {
    id: u32,
    current_items: Vec<Wrapping<u32>>,
    operation: String,
    value: Option<u32>,
    divisible_test_value: u32,
    true_monkey_id: usize,
    false_monkey_id: usize,
    activity: u32,
}


pub(crate) fn exercise11_1() {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey in read_file("../../inputs/11/test").split("\n\n") {
        monkeys.push(construct_monkey(monkey))
    }
    monkeys.sort_by(|m, m2| if m.id < m2.id { Less } else { Greater });
    monkeys = do_business(monkeys, true, 20, true);
    let mut monkey_business_levels: Vec<Wrapping<u32>> = monkeys.iter().map(|m| Wrapping(m.activity)).collect();
    monkey_business_levels.sort();
    println!("Business Level: {}", monkey_business_levels.pop().unwrap() * monkey_business_levels.pop().unwrap());
}


fn construct_monkey(description: &str) -> Monkey {
    let mut id: Option<u32> = None;
    let mut current_items: Vec<Wrapping<u32>> = Vec::new();
    let mut operation: String = String::from("");
    let mut value: Option<u32> = None;
    let mut divisible_test_value: Option<u32> = None;
    let mut true_monkey_id: Option<usize> = None;
    let mut false_monkey_id: Option<usize> = None;
    let activity: u32 = 0;
    for line in description.split('\n') {
        if line.starts_with("Monkey") {
            id = Some(line.split(' ').last().unwrap().trim_end_matches(':').parse().unwrap())
        }
        if line.starts_with("  Starting items: ") {
            current_items = line.split(": ").last().unwrap().split(", ").map(|s| Wrapping(s.parse().unwrap())).collect()
        }
        if line.starts_with("  Operation: ") {
            let operation_value = line.split(' ').last().unwrap();
            if operation_value != "old" {
                value = Some(operation_value.parse().unwrap());
            }
            operation = if line.contains('*') { "*" } else { "+" }.parse().unwrap();
        }
        if line.starts_with("  Test: ") {
            divisible_test_value = Some(line.split(' ').last().unwrap().parse().unwrap());
        }
        if line.starts_with("    If true: ") {
            true_monkey_id = Some(line.split(' ').last().unwrap().parse().unwrap());
        }
        if line.starts_with("    If false: ") {
            false_monkey_id = Some(line.split(' ').last().unwrap().parse().unwrap());
        }
    }
    Monkey {
        id: id.unwrap(),
        current_items,
        operation,
        value,
        divisible_test_value: divisible_test_value.unwrap(),
        true_monkey_id: true_monkey_id.unwrap(),
        false_monkey_id: false_monkey_id.unwrap(),
        activity,
    }
}

fn do_business(mut monkeys: Vec<Monkey>, decrease_worry: bool, rounds: usize, verbose: bool) -> Vec<Monkey> {
    for round in 1..rounds + 1 {
        for index in 0..monkeys.len() {
            if verbose {
                println!("Monkey {}:", monkeys[index].id);
            }
            monkeys = process_monkey(monkeys, index, decrease_worry, verbose)
        }
        if round == 1 || round == 20 || round % 1000 == 0 {
            println!("== After round {} ==", round);
            for monkey in monkeys.clone() {
                println!("Monkey {} inspected items {} times.", monkey.id, monkey.activity);
            }
        }
    }
    monkeys
}

fn process_monkey(monkeys: Vec<Monkey>, index: usize, decrease_worry: bool, verbose: bool) -> Vec<Monkey> {
    let mut new_monkeys = monkeys.clone();
    for item in monkeys[index].current_items.iter() {
        if verbose {
            println!("  Monkey inspects an item with a worry level of {}.", item);
        }
        let mut new_worry_level: Wrapping<u32>;
        new_monkeys[index].activity += 1;

        let operation_value: Wrapping<u32> = if monkeys[index].value.is_none() { *item } else { Wrapping(monkeys[index].value.unwrap() as u32) };
        if monkeys[index].operation == "*" {
            new_worry_level = *item * operation_value;
            if verbose {
                println!("    Worry level is multiplied by {} to {}.", operation_value, new_worry_level);
            }
        } else if monkeys[index].operation == "+" {
            new_worry_level = *item + operation_value;
            if verbose {
                println!("    Worry level increases by {} to {}.", operation_value, new_worry_level);
            }
        } else {
            panic!()
        }

        if decrease_worry {
            new_worry_level /= 3;
            if verbose {
                println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", new_worry_level);
            }
        }

        let monkey_id = if new_worry_level.0 % monkeys[index].divisible_test_value as u32 == 0 {
            if verbose {
                println!("    Current worry level is divisible by {}.", monkeys[index].divisible_test_value);
            }
            monkeys[index].true_monkey_id
        } else {
            if verbose {
                println!("    Current worry level is not divisible by {}.", monkeys[index].divisible_test_value);
            }
            monkeys[index].false_monkey_id
        };
        if verbose {
            println!("    Item with worry level {} is thrown to monkey {}.", new_worry_level, monkey_id);
        }
        new_monkeys[monkey_id as usize].current_items.push(new_worry_level);
    }
    new_monkeys[index].current_items = Vec::new();
    new_monkeys
}

pub(crate) fn exercise11_2() {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey in read_file("../../inputs/11/test").split("\n\n") {
        monkeys.push(construct_monkey(monkey))
    }
    monkeys.sort_by(|m, m2| if m.id < m2.id { Less } else { Greater });
    monkeys = do_business(monkeys, false, 10000, false);
    let mut monkey_business_levels: Vec<Wrapping<u32>> = monkeys.iter().map(|m| Wrapping(m.activity)).collect();
    monkey_business_levels.sort();
    let best_activity = monkey_business_levels.pop().unwrap();
    let second_best_activity = monkey_business_levels.pop().unwrap();
    println!("Best Activity: {}", best_activity);
    println!("Second Best Activity: {}", second_best_activity);
    println!("Business Level: {}", best_activity * second_best_activity);
}