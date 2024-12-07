pub fn main() {

    const PLUS: char = '+';
    const TIMES: char = '*';
    const CAT: char = '|';

    const OPS: [char; 2] = ['+', '*'];

    let equations = parse_input();

    let valid_eq_sum = get_valid_eq_len(&equations);
    println!("{}",valid_eq_sum);



}
fn get_valid_eq_len(equations: &[(u32, Vec<u32>)]) -> u32 {
    println!("{:?}", equations);

    let mut valid_count = 0;

    for (test_value, numbers) in equations {
    }
    valid_count
}


fn parse_input()-> Vec<(u32, Vec<u32>)> {
    let contents = include_str!("../input.txt");
    let mut equations: Vec<(u32, Vec<u32>)> = Vec::new();


    for line in contents.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            if let Ok(key) = parts[0].trim().parse::<u32>() {
                let values: Vec<u32> = parts[1]
                    .trim()
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                equations.push((key, values));
            }
        }
    }

    return equations;

}
