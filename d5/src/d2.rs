use std::collections::HashMap;

pub fn main() {
    let contents = include_str!("../input.txt");

    let (rules, updates) = parse_input(&contents);

    let mut rule_map: HashMap<u32, Vec<u32>> = HashMap::new();

    for (key, value) in rules {
        let entry = rule_map.entry(key).or_default(); // default is Vec::new()
        entry.push(value);
    }

    let mut corrected_sum = 0;

    // Process each update and correct it if needed
    for update in updates {
        if !is_valid_update(&update, &rule_map) {
            // Reorder the update if it's not valid
            let mut corrected = update.clone();
            reorder_update(&mut corrected, &rule_map);

            // Find the middle page after reordering
            let middle_page = corrected[corrected.len() / 2];
            corrected_sum += middle_page;
        }
    }

    println!("{}", corrected_sum);
}
fn parse_input(contents: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let parts: Vec<&str> = contents.split("\n\n").collect();

    let rules = parts[0]
        .lines()
        .map(parse_rule)
        .collect::<Vec<(u32, u32)>>();

    let updates = parts[1]
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    (rules, updates)
}

fn parse_rule(rule: &str) -> (u32, u32) {
    let parts: Vec<u32> = rule.split('|').map(|x| x.parse::<u32>().unwrap()).collect();
    (parts[0], parts[1])
}

fn is_valid_update(update: &[u32], rule_map: &HashMap<u32, Vec<u32>>) -> bool {
    let indexes_mapping: HashMap<&u32, usize> = update
        .iter()
        .enumerate()
        .map(|(index, value)| (value, index))
        .collect::<HashMap<&u32, usize>>();

    // Check if the update is valid by comparing element indices with the rule
    for (index, element) in update.iter().enumerate() {
        if let Some(pages_associated_to_element) = rule_map.get(element) {
            for page in pages_associated_to_element {
                if let Some(&page_index) = indexes_mapping.get(page) {
                    if page_index < index {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn reorder_update(update: &mut [u32], rule_map: &HashMap<u32, Vec<u32>>) {
    loop {
        let mut swapped = false;

        for i in 0..update.len() - 1 {
            if should_swap(update[i], update[i + 1], rule_map) {
                update.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

fn should_swap(current: u32, next: u32, rule_map: &HashMap<u32, Vec<u32>>) -> bool {
    if let Some(associated_pages) = rule_map.get(&current) {
        !associated_pages.contains(&next)
    } else {
        false
    }
}
