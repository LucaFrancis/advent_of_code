use crate::Exercise;
use std::cmp::max;

use crate::file_utils::read_file;

pub(crate) fn get_ex8() -> Exercise {
    Exercise {
        id: 8,
        first_part: exercise8_1,
        second_part: exercise8_2,
    }
}

fn exercise8_1() {
    let mut trees: Vec<Vec<u32>> = Vec::new();
    for line in read_file("../../inputs/8/input").split('\n') {
        let mut treeline: Vec<u32> = vec![];
        for char in line.chars() {
            let height: u32 = char.to_digit(10).unwrap();
            treeline.push(height)
        }
        trees.push(treeline)
    }

    let height = trees.len();
    let width = trees.first().unwrap().len();
    assert!(trees.iter().all(|treeline| treeline.len() == width));

    let mut visibility_map: Vec<Vec<i32>> = Vec::new();
    for row in 1..height - 1 {
        let mut treeline: Vec<i32> = Vec::new();
        for column in 1..width - 1 {
            treeline.push(is_visible(trees.clone(), row, column))
        }
        visibility_map.push(treeline);
    }

    let inside = visibility_map
        .iter()
        .map(|treeline| treeline.iter().sum::<i32>())
        .sum::<i32>();
    let outside = (height * 2 + width * 2 - 4) as i32;

    println!("Visible Trees inside: {}", inside);
    println!("Visible Trees outside: {}", outside);
    println!("Visible Trees: {}", inside + outside);
}

fn is_visible(trees: Vec<Vec<u32>>, row: usize, column: usize) -> i32 {
    let current_tree = trees[row][column];

    let top = trees
        .iter()
        .map(|treeline| treeline[column])
        .collect::<Vec<u32>>()[0..row]
        .to_vec();
    let bottom = trees
        .iter()
        .map(|treeline| treeline[column])
        .collect::<Vec<u32>>()[row + 1..trees.len()]
        .to_vec();
    let left = trees[row][0..column].to_vec();
    let right = trees[row][column + 1..trees.len()].to_vec();
    if vec![top, bottom, right, left]
        .into_iter()
        .any(|direction| direction.iter().all(|tree| tree < &current_tree))
    {
        return 1;
    }
    0
}

fn get_scenic_score(trees: Vec<Vec<u32>>, row: usize, column: usize) -> i32 {
    let current_tree = trees[row][column];

    let mut top = trees
        .iter()
        .map(|treeline| treeline[column])
        .collect::<Vec<u32>>()[0..row]
        .to_vec();
    top.reverse();
    let bottom = trees
        .iter()
        .map(|treeline| treeline[column])
        .collect::<Vec<u32>>()[row + 1..trees.len()]
        .to_vec();
    let mut left = trees[row][0..column].to_vec();
    left.reverse();
    let right = trees[row][column + 1..trees.len()].to_vec();

    let top_score = get_scenic_score_for_direction(current_tree, top);
    let left_score = get_scenic_score_for_direction(current_tree, left);
    let bottom_score = get_scenic_score_for_direction(current_tree, bottom);
    let right_score = get_scenic_score_for_direction(current_tree, right);

    top_score * left_score * bottom_score * right_score
}

fn get_scenic_score_for_direction(height: u32, sight: Vec<u32>) -> i32 {
    if sight.is_empty() {
        return 0;
    }
    let mut sight_line = 0;
    for tree in sight {
        sight_line += 1;
        if tree >= height {
            break;
        }
    }
    sight_line
}

fn exercise8_2() {
    let mut trees: Vec<Vec<u32>> = Vec::new();
    for line in read_file("../../inputs/8/input").split('\n') {
        let mut treeline: Vec<u32> = vec![];
        for char in line.chars() {
            let height: u32 = char.to_digit(10).unwrap();
            treeline.push(height)
        }
        trees.push(treeline)
    }

    let height = trees.len();
    let width = trees.first().unwrap().len();
    assert!(trees.iter().all(|treeline| treeline.len() == width));
    let mut highest_scenic = 0;
    for row in 0..height {
        for column in 0..width {
            highest_scenic = max(highest_scenic, get_scenic_score(trees.clone(), row, column))
        }
    }

    println!("Highest Scenic score: {}", highest_scenic);
}
