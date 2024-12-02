use std::{collections::HashMap, iter::zip};

fn main() {
    println!("{}", part1("inputs/day1/test1"));
    println!("{}", part1("inputs/day1/day1"));
    println!("{}", part2("inputs/day1/test1"));
    println!("{}", part2("inputs/day1/day1"));
}

fn part1(path: &str) -> i32 {
    let txt = std::fs::read_to_string(path).unwrap();
    let txt = txt.lines();
    let (mut lefts, mut rights): (Vec<i32>, Vec<i32>) = txt
        .map(|line| {
            let nums = line.split("   ").collect::<Vec<&str>>();
            if let [num1, num2] = nums[..] {
                return (num1.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap());
            }
            panic!("Invalid line")
        })
        .unzip();
    lefts.sort();
    rights.sort();
    zip(lefts, rights).fold(0, |acc, (left, right)| acc + left.abs_diff(right) as i32)
}

fn part2(path: &str) -> i32 {
    let txt = std::fs::read_to_string(path).unwrap();
    let txt = txt.lines();
    let (nums, appears): (Vec<i32>, HashMap<&str, i32>) = txt.fold(
        (Vec::new(), HashMap::new()),
        |(mut nums, mut appears), line| {
            let line: Vec<&str> = line.split("   ").collect::<Vec<&str>>();
            if let [num1, num2] = line[..] {
                let num1 = num1.parse::<i32>().unwrap();
                nums.push(num1);
                let num2 = appears.entry(num2).or_insert(0);
                *num2 += 1;
            }
            (nums, appears)
        },
    );
    nums.iter().fold(0, |acc, num| {
        acc + num * appears.get(num.to_string().as_str()).unwrap_or(&0)
    })
}
