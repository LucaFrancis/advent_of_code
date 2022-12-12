use crate::file_utils::read_file;
use crate::Exercise;

pub(crate) fn get_ex1() -> Exercise {
    Exercise {
        id: 1,
        first_part: exercise1_1,
        second_part: exercise1_2,
    }
}

fn exercise1_1() {
    let text = read_file("../../inputs/1/input");
    let lines = text.split("\n");
    let mut elves: Vec<Vec<&str>> = Vec::new();
    let mut elf: Vec<&str> = Vec::new();
    for line in lines {
        if line == "" {
            elves.push(elf.clone());
            elf = Vec::new()
        } else {
            elf.push(line)
        }
    }
    let number_elves: Vec<Vec<i32>> = elves
        .iter()
        .map(|inventory| {
            inventory
                .iter()
                .map(|food| match food.parse::<i32>() {
                    Ok(food) => food,
                    Err(why) => panic!("could not parse food to int: {}", why),
                })
                .collect()
        })
        .collect();

    let mut summed_inventory: Vec<i32> = number_elves
        .iter()
        .map(|inventory| inventory.iter().sum::<i32>())
        .collect();
    summed_inventory.sort();
    summed_inventory.reverse();
    println!("{}", summed_inventory[0])
}

fn exercise1_2() {
    let text = read_file("../../inputs/1/input");
    let lines = text.split("\n");
    let mut elves: Vec<Vec<&str>> = Vec::new();
    let mut elf: Vec<&str> = Vec::new();
    for line in lines {
        if line.is_empty() {
            elves.push(elf.clone());
            elf = Vec::new()
        } else {
            elf.push(line)
        }
    }
    let number_elves: Vec<Vec<i32>> = elves
        .iter()
        .map(|inventory| {
            inventory
                .iter()
                .map(|food| match food.parse::<i32>() {
                    Ok(food) => food,
                    Err(why) => panic!("could not parse food to int: {}", why),
                })
                .collect()
        })
        .collect();

    let mut summed_inventory: Vec<i32> = number_elves
        .iter()
        .map(|inventory| inventory.iter().sum::<i32>())
        .collect();
    summed_inventory.sort();
    summed_inventory.reverse();
    for i in 0..3 {
        println!("{}.: {}", i + 1, summed_inventory[i])
    }
}
