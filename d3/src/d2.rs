use regex::Regex;


pub fn main() {
    let input = include_str!("../input.txt");
    let result = process_input(input);
    println!("{}", result);
}



fn process_input(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut total = 0;

    for cap in re.captures_iter(input) {
        if let Some(m) = cap.get(0) {
            match m.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ if enabled => {
                    let x: u32 = cap[1].parse().unwrap();
                    let y: u32 = cap[2].parse().unwrap();
                    total += x * y;
                },
                _ => {}
            }
        }
    }

    total
}
