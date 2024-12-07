pub fn main() {
    let contents = include_str!("../input.txt");

    let mut left = vec![];
    let mut right = vec![];

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse::<i32>().unwrap());
        right.push(parts[1].parse::<i32>().unwrap());
    }

    let mut total_score = 0;

    // Calculate the score
    for &left_value in &left {
        let mut count = 0;
        for &right_value in &right {
            if left_value == right_value {
                count += 1;
            }
        }
        // Add to total score
        total_score += left_value * count;
    }

    println!("Total score: {}", total_score);
}
