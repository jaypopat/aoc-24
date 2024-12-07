use regex::Regex;

pub fn main() {
    let input = include_str!("../input.txt");
    let result = process_input(input);
    println!("{}", result);
}

fn process_input(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            // println!("{:?}",cap);
            let x: u32 = cap[1].parse().unwrap();
            let y: u32 = cap[2].parse().unwrap();
            x * y
        })
        .sum()
}
