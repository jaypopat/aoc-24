pub fn main() {
    let contents = include_str!("../input.txt");

    let reports: Vec<Vec<i32>> = parse_input(&contents);

    let mut safe_counter = 0;

    for report in reports {
        if check_safe_with_one_removal(&report){
            safe_counter += 1;
        }
    }
    println!("{}",safe_counter);
}
fn parse_input(contents: &str) -> Vec<Vec<i32>> {
    contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect()
        })
        .collect()
}

fn check_safe(report: &[i32]) -> bool {

    let first_diff = report[1] - report[0];

    if first_diff.abs() < 1 || first_diff.abs() > 3 {
        return false;
    }

    let is_increasing = first_diff > 0;

    for i in 1..report.len() {

        let diff = report[i]-report[i - 1];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        let curr_is_increasing = diff > 0;
        if (is_increasing && !curr_is_increasing) || (!is_increasing && curr_is_increasing) {
            return false;
        }
    }

    true
}

fn check_safe_with_one_removal(report: &[i32]) -> bool {
    if check_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i);

        if check_safe(&modified_report) {
            return true;
        }
    }
    false
}
