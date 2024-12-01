use std::collections::HashMap;

fn main() {
    println!("{}", part1("inputs/test1"));
    println!("{}", part1("inputs/day1"));
    println!("{}", part2("inputs/test1"));
    println!("{}", part2("inputs/day1"));
}

fn part1(path: &str) -> i32 {
    let txt = std::fs::read_to_string(path).unwrap();
    let txt = txt.lines();
    let (mut nums1, mut nums2): (Vec<i32>, Vec<i32>) = txt
        .filter_map(|line| {
            let nums = line.split("   ").collect::<Vec<&str>>();
            if let [num1, num2] = nums[..] {
                return Some((num1.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap()));
            }
            None
        })
        .unzip();
    nums1.sort();
    nums2.sort();
    (0..nums1.len()).fold(0, |acc, e| acc + nums1[e].abs_diff(nums2[e])) as i32
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
    nums.iter()
        .map(|num| num * appears.get(num.to_string().as_str()).unwrap_or(&0))
        .sum()
}
