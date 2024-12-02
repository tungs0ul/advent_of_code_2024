use std::{cmp::Ordering, collections::HashSet};

fn main() {
    println!("{}", part1("inputs/day2/test"));
    println!("{}", part1("inputs/day2/input"));
    println!("{}", part2("inputs/day2/test"));
    println!("{}", part2("inputs/day2/input"));
}

fn check_safe(nums: &[i32]) -> bool {
    let tmp: HashSet<Ordering> = nums
        .windows(2)
        .map(|window| {
            if let [a, b] = window {
                if a.abs_diff(*b) > 3 {
                    return std::cmp::Ordering::Equal;
                }
                return a.cmp(b);
            }
            panic!("not valid line");
        })
        .collect();
    if tmp.len() > 1 || tmp.contains(&Ordering::Equal) {
        return false;
    }
    true
}

fn part1(path: &str) -> i32 {
    let txt = std::fs::read_to_string(path).unwrap();
    let txt = txt.lines();
    txt.filter_map(|line| {
        let cmp = line
            .split(' ')
            .map(|txt| txt.parse::<i32>().expect("Invalid number"))
            .collect::<Vec<i32>>();

        match check_safe(&cmp) {
            true => Some(1),
            false => None,
        }
    })
    .sum::<i32>()
}

fn part2(path: &str) -> i32 {
    let txt = std::fs::read_to_string(path).unwrap();
    let txt: std::str::Lines<'_> = txt.lines();
    txt.filter_map(|line| {
        let cmp = line
            .split(' ')
            .map(|txt| txt.parse::<i32>().expect("Invalid number"))
            .collect::<Vec<i32>>();

        if check_safe(&cmp) {
            return Some(1);
        }
        for i in 0..cmp.len() {
            let mut tmp = cmp.clone();
            tmp.remove(i);
            if check_safe(&tmp) {
                return Some(1);
            }
        }
        None
    })
    .sum::<i32>()
}
