use std::fs::File;
use std::io::Read;
use std::path::Path;

pub(crate) fn exercise1() {
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
    let number_elves: Vec<Vec<i32>> = elves.iter().map(
        |inventory| inventory.iter().map(
            |food| match food.parse::<i32>() {
                Ok(food) => food,
                Err(why) => panic!("could not parse food to int: {}", why)
            }).collect())
        .collect();

    let mut summed_inventory: Vec<i32> = number_elves.iter().map(|inventory| inventory.iter().sum::<i32>()).collect();
    summed_inventory.sort();
    summed_inventory.reverse();
    for i in 0..3  {
        println!("{}.: {}", i+1, summed_inventory[i])
    }
}

fn read_file(path_to_file: &str) -> String {
    let path = Path::new(path_to_file);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Err(why) => panic!("couldnt read {}: {}", display, why),
        Ok(_) => return text
    }
}