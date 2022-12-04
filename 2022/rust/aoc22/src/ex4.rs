use crate::file_utils::read_file;

pub(crate) fn exercise4_1() {
    let mut score: u32 = 0;
    for line in read_file("../../inputs/4/input").split("\n") {
        let jobs: Vec<&str> = line.split(",").collect();
        let section1: Vec<&str> = jobs[0].split("-").collect();
        let section2: Vec<&str> = jobs[1].split("-").collect();

        let s1_start = section1[0].parse::<i32>().unwrap();
        let s1_end = section1[1].parse::<i32>().unwrap();

        let s2_start = section2[0].parse::<i32>().unwrap();
        let s2_end = section2[1].parse::<i32>().unwrap();

        if is_fully_contained(s1_start, s1_end, s2_start, s2_end) {
            score += 1
        }
    };
    println!("{}", score)
}

fn is_fully_contained(s1_start: i32, s1_end: i32, s2_start: i32, s2_end: i32) -> bool {
    return (s1_end <= s2_end && s1_start >= s2_start) || (s2_end <= s1_end && s2_start >= s1_start);
}

fn is_overlap(s1_start: i32, s1_end: i32, s2_start: i32, s2_end: i32) -> bool {
    return s1_start <= s2_end && s2_start <= s1_end;
}

pub(crate) fn exercise4_2() {
    let mut score: u32 = 0;
    for line in read_file("../../inputs/4/input").split("\n") {
        let jobs: Vec<&str> = line.split(",").collect();
        let section1: Vec<&str> = jobs[0].split("-").collect();
        let section2: Vec<&str> = jobs[1].split("-").collect();

        let s1_start = section1[0].parse::<i32>().unwrap();
        let s1_end = section1[1].parse::<i32>().unwrap();

        let s2_start = section2[0].parse::<i32>().unwrap();
        let s2_end = section2[1].parse::<i32>().unwrap();

        if is_overlap(s1_start, s1_end, s2_start, s2_end) {
            score += 1
        }
    };
    println!("{}", score)
}