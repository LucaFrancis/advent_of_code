use regex::Regex;
use crate::Exercise;

use crate::file_utils::read_file;

pub(crate) fn get_ex5() -> Exercise {
    Exercise {
        id: 5,
        first_part: exercise5_1,
        second_part: exercise5_2,
    }
}

struct Instruction {
    amount: i32,
    source: usize,
    dest: usize,
}

fn exercise5_1() {
    let mut stacks: Vec<Vec<String>> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();
    for line in read_file("../../inputs/5/input").split("\n") {
        if line.trim().starts_with("[") {
            get_start_config_line(&mut stacks, line);
            continue;
        }
        if line.starts_with("move") {
            let my_captures: Vec<&str> = re.find_iter(line).map(|x| x.as_str()).collect();
            instructions.push(Instruction {
                amount: my_captures[0].parse().unwrap(),
                source: my_captures[1].parse().unwrap(),
                dest: my_captures[2].parse().unwrap(),
            });
            continue;
        }
    };

    let mut new_stack: Vec<Vec<String>> = Vec::new();
    for mut stack in stacks {
        stack.reverse();
        new_stack.push(stack)
    }
    for instruction in instructions {
        println!("Moving {} from {} to {}", instruction.amount, instruction.source, instruction.dest);
        for _ in 0..instruction.amount {
            let value = new_stack.get_mut(instruction.source - 1).unwrap().pop().unwrap();
            println!("Took {} from {}", value, instruction.source);
            let v = value.clone();
            new_stack[instruction.dest - 1].push(value);
            println!("Put {} on {}", v, instruction.dest);
        }
    }

    for stack in new_stack {
        print!("{}", stack.last().unwrap())
    }
    println!("\n")
}

fn get_start_config_line(stacks: &mut Vec<Vec<String>>, line: &str) {
    let seperated_lines = line.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % 4 == 0 {
                Some('|')
            } else {
                None
            }
                .into_iter()
                .chain(std::iter::once(c))
        })
        .collect::<String>();
    let containers = seperated_lines.split("|").collect::<Vec<&str>>();
    for (index, container) in containers.iter().enumerate() {
        if index >= stacks.len() {
            stacks.push(Vec::new())
        }
        let char = (**container).chars().nth(1).unwrap().to_string();
        if char != " " {
            stacks.get_mut(index).unwrap().push((**container).chars().nth(1).unwrap().to_string())
        }
    }
}


fn exercise5_2() {
    let mut stacks: Vec<Vec<String>> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();
    for line in read_file("../../inputs/5/input").split("\n") {
        if line.trim().starts_with("[") {
            get_start_config_line(&mut stacks, line);
            continue;
        }
        if line.starts_with("move") {
            let my_captures: Vec<&str> = re.find_iter(line).map(|x| x.as_str()).collect();
            instructions.push(Instruction {
                amount: my_captures[0].parse().unwrap(),
                source: my_captures[1].parse().unwrap(),
                dest: my_captures[2].parse().unwrap(),
            });
            continue;
        }
    };

    let mut new_stack: Vec<Vec<String>> = Vec::new();
    for mut stack in stacks {
        stack.reverse();
        new_stack.push(stack)
    }
    for instruction in instructions {
        println!("Moving {} from {} to {}", instruction.amount, instruction.source, instruction.dest);
        let mut values = Vec::new();
        for _ in 0..instruction.amount {
            let value = new_stack.get_mut(instruction.source - 1).unwrap().pop().unwrap();
            println!("Took {} from {}", value, instruction.source);
            values.push(value)
        }
        values.reverse();
        new_stack[instruction.dest - 1].extend_from_slice(&values);
    }

    for stack in new_stack {
        print!("{}", stack.last().unwrap())
    }
    println!("\n")
}