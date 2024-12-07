use itertools::Itertools;

pub fn main() {
    let equations = parse_input();
    let valid_eq_sum = get_valid_eq_len(&equations);
    println!("{}", valid_eq_sum);
}

fn get_valid_eq_len(equations: &[(u64, Vec<u64>)]) -> u64 {
    let mut valid_count: u64 = 0;

    for (test_value, numbers) in equations {
        if validate_ops(numbers, *test_value) {
            valid_count += *test_value;
        }
        println!("--------------");
    }
    valid_count
}

fn validate_ops(numbers: &[u64], test_value: u64) -> bool {
    const OPS: [char; 3] = ['+', '*', '|'];
    let operator_count = numbers.len() - 1;

    // getting all the possible combinations of operators
    let combinations = std::iter::repeat(OPS.iter().cloned())
        .take(operator_count)
        .multi_cartesian_product()
        .collect::<Vec<Vec<char>>>();

    println!("{:?}", combinations);

    for combination in combinations {
        let mut result = numbers[0];

        let nums_and_operators_zipped = numbers[1..].iter().zip(combination.iter());
        //  [(6, '+'), (16, '+'), (20, '+')], [(6, '+'), (16, '+'), (20, '*')] etc etc for every combination

        for (&num, &op) in nums_and_operators_zipped {
            //println!("Applying {} to {} and {}", op, result, num);
            match op {
                '+' => result += num,
                '*' => result *= num,
                '|' => {
                    result = result * 10u64.pow(num.to_string().len() as u32) + num;
                    // eg 12|34 = 12* 100 + 34
                }
                _ => panic!("Only + and * have been added for part 1"),
            }
            //println!("Result so far: {}", result);
        }

        //println!("result withh this combination: {}", result);

        if result > test_value {
            println!("small optimisation check - result greater than test value so just skipping");
            continue;
        }
        if result == test_value {
            println!("Equation is true: {:?}", numbers);
            return true;
        }
    }
    false
}

fn parse_input() -> Vec<(u64, Vec<u64>)> {
    let contents = include_str!("../input.txt");
    let mut equations: Vec<(u64, Vec<u64>)> = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            if let Ok(key) = parts[0].trim().parse::<u64>() {
                let values: Vec<u64> = parts[1]
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                equations.push((key, values));
            }
        }
    }
    equations
}
