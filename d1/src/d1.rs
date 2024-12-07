pub fn main() {
    let contents = include_str!("../input.txt");

    let mut left = vec![];
    let mut right = vec![];

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse::<i32>().unwrap());
        right.push(parts[1].parse::<i32>().unwrap());
    }

    // println!("Left: {:?}", left);
    // println!("Right: {:?}", right);

    left.sort();
    right.sort();

    let mut total_distance = 0;
    for i in 0..left.len() {
        total_distance += (left[i] - right[i]).abs();
    }

    println!("Total distance: {}", total_distance);
}
