use crate::file_utils::read_file;
use crate::Exercise;

pub(crate) fn get_ex12() -> Exercise {
    Exercise {
        id: 12,
        first_part: exercise12_1,
        second_part: exercise12_2,
    }
}

pub(crate) fn exercise12_1() {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in read_file("../../inputs/12/test").split('\n') {
        let mut row: Vec<u32> = Vec::new();
        for char in line.chars() {
            row.push(char.to_string().parse().unwrap())
        }
        grid.push(row)
    }
}

pub(crate) fn exercise12_2() {}
