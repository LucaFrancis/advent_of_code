use crate::Exercise;
use std::cmp::max;
use std::collections::HashMap;

use crate::file_utils::read_file;

pub(crate) fn get_ex7() -> Exercise {
    Exercise {
        id: 7,
        first_part: exercise7_1,
        second_part: exercise7_2,
    }
}

#[derive(Debug, Clone)]
struct Directory {
    parent: String,
    children: Vec<String>,
    size: i32,
    file_size: i32,
    depth: i32,
}

fn exercise7_1() {
    let mut directory: HashMap<String, Directory> = HashMap::new();
    directory.insert(
        String::from("/"),
        Directory {
            parent: String::from(""),
            size: 0,
            file_size: 0,
            depth: 0,
            children: Vec::new(),
        },
    );

    let mut current_directory: String = "/".to_string();
    let mut max_depth = 0;
    for line in read_file("../../inputs/7/input").split('\n') {
        print!("[{}]<{}>:\t", line, current_directory);
        if line.starts_with("$ cd") {
            (directory, current_directory) =
                handle_cd(directory.clone(), current_directory.clone(), line);
            continue;
        }
        if line.starts_with("dir") {
            (directory, current_directory, max_depth) = handle_dir(
                directory.clone(),
                current_directory.clone(),
                line,
                max_depth,
            );
            continue;
        }
        let files_size = match line.split(' ').next().unwrap().parse::<i32>() {
            Ok(size) => size,
            Err(_) => {
                println!("Skipping instruction");
                continue;
            }
        };
        let current_directory_name = current_directory.clone();
        let mut curr = directory.get(&*current_directory).unwrap().clone();
        curr.file_size += files_size;
        match directory.insert(current_directory_name.clone(), curr) {
            None => {
                println!("Added {}, to {}", files_size, current_directory_name);
            }
            Some(_) => {
                println!("Added {}, to {}", files_size, current_directory_name);
            }
        }
    }

    println!("{}", max_depth);
    for distance_from_bottom in 0..max_depth + 1 {
        let mut complete_directory = directory.clone();
        for (name, dir) in directory
            .iter()
            .filter(|(_, d)| d.depth == max_depth - distance_from_bottom)
        {
            let size = get_size_of_children(directory.clone(), name.to_string());
            let mut new_dir = dir.clone();
            let final_size = new_dir.file_size + size;
            new_dir.size = new_dir.file_size + size;
            complete_directory.insert(name.to_string(), new_dir);
            let depth_str = "\t".repeat((max_depth - distance_from_bottom) as usize);
            println!("{}{}({})", depth_str, name, final_size);
        }
        directory = complete_directory;
    }

    println!(
        "{}",
        directory
            .iter()
            .map(|(_, d)| d.size)
            .filter(|d| d <= &100000)
            .sum::<i32>()
    )
}

fn handle_cd(
    directory: HashMap<String, Directory>,
    current_directory: String,
    line: &str,
) -> (HashMap<String, Directory>, String) {
    if line.starts_with("$ cd /") {
        println!("Skipping instruction");
        return (directory, current_directory);
    }
    if line.starts_with("$ cd ..") {
        let parent = directory.get(&*current_directory).unwrap().clone().parent;
        println!("Move up one level to {}", parent);
        return (directory, parent);
    }
    println!("Move into {}", current_directory);
    return (
        directory,
        current_directory + "/" + line.split(' ').last().unwrap(),
    );
}

fn handle_dir(
    mut directory: HashMap<String, Directory>,
    current_directory: String,
    line: &str,
    max_depth: i32,
) -> (HashMap<String, Directory>, String, i32) {
    // New Directory Name
    let directory_name = current_directory.clone() + "/" + line.split(' ').last().unwrap();

    // Update current directory children
    let mut curr = directory.get(&*current_directory).unwrap().clone();
    curr.children.push(directory_name.parse().unwrap());
    let curr_depth = curr.depth;
    match directory.insert(current_directory.clone(), curr) {
        None => {
            println!("Added {} as child to {}", directory_name, current_directory);
        }
        Some(_) => {
            println!("Added {} as child to {}", directory_name, current_directory);
        }
    }

    // Insert new directory
    match directory.insert(
        directory_name.parse().unwrap(),
        Directory {
            parent: current_directory.clone(),
            children: Vec::new(),
            size: 0,
            file_size: 0,
            depth: curr_depth + 1,
        },
    ) {
        None => {
            println!("Added {} to directory", directory_name);
        }
        Some(_) => {
            println!("Added {} to directory", directory_name);
        }
    }

    // Update depth
    (directory, current_directory, max(max_depth, curr_depth + 1))
}

fn get_size_of_children(directory: HashMap<String, Directory>, name: String) -> i32 {
    directory
        .iter()
        .filter(|(_, dir)| dir.parent == name)
        .map(|(_, dir)| dir.size)
        .sum()
}

fn exercise7_2() {
    let mut directory: HashMap<String, Directory> = HashMap::new();
    directory.insert(
        String::from("/"),
        Directory {
            parent: String::from(""),
            size: 0,
            file_size: 0,
            depth: 0,
            children: Vec::new(),
        },
    );

    let mut current_directory: String = "/".to_string();
    let mut max_depth = 0;
    for line in read_file("../../inputs/7/input").split('\n') {
        print!("[{}]<{}>:\t", line, current_directory);
        if line.starts_with("$ cd") {
            (directory, current_directory) =
                handle_cd(directory.clone(), current_directory.clone(), line);
            continue;
        }
        if line.starts_with("dir") {
            (directory, current_directory, max_depth) = handle_dir(
                directory.clone(),
                current_directory.clone(),
                line,
                max_depth,
            );
            continue;
        }
        let files_size = match line.split(' ').next().unwrap().parse::<i32>() {
            Ok(size) => size,
            Err(_) => {
                println!("Skipping instruction");
                continue;
            }
        };
        let current_directory_name = current_directory.clone();
        let mut curr = directory.get(&*current_directory).unwrap().clone();
        curr.file_size += files_size;
        match directory.insert(current_directory_name.clone(), curr) {
            None => {
                println!("Added {}, to {}", files_size, current_directory_name);
            }
            Some(_) => {
                println!("Added {}, to {}", files_size, current_directory_name);
            }
        }
    }

    println!("{}", max_depth);
    for distance_from_bottom in 0..max_depth + 1 {
        let mut complete_directory = directory.clone();
        for (name, dir) in directory
            .iter()
            .filter(|(_, d)| d.depth == max_depth - distance_from_bottom)
        {
            let size = get_size_of_children(directory.clone(), name.to_string());
            let mut new_dir = dir.clone();
            let final_size = new_dir.file_size + size;
            new_dir.size = new_dir.file_size + size;
            complete_directory.insert(name.to_string(), new_dir);
            let depth_str = "\t".repeat((max_depth - distance_from_bottom) as usize);
            println!("{}{}({})", depth_str, name, final_size);
        }
        directory = complete_directory;
    }
    let unused_space = 70000000 - directory.get("/").unwrap().size;
    let needed_space = 30000000 - unused_space;
    let mut candidates = directory
        .iter()
        .map(|(_, d)| d.size)
        .filter(|size| size >= &needed_space)
        .collect::<Vec<i32>>();
    candidates.sort();

    println!("{}", candidates.first().unwrap());
}
