use std::collections::HashMap;

pub fn main() {
    let contents = include_str!("../input.txt");

    let (rules, updates) = parse_input(&contents);

    // use a hashmap for storing the rules
    let mut rule_map: HashMap<u32, Vec<u32>> = HashMap::new();

    // if same key get the value and push to vec
    for (key, value) in rules {
        let entry = rule_map.entry(key).or_default(); // default is Vec::new()
        entry.push(value);
    }
    println!("{:?}", rule_map);

    // now check the updates list

    let mut sum = 0;
    for update in updates {
        // check if update is valid
        // if yes get middle val

        if is_valid_update(&update, &rule_map) {
            let middle_page = update[update.len() / 2];
            sum += middle_page;
        }
    }
    println!("{}", sum);
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
    // Create a mapping of values to their indices in the update
    let indexes_mapping: HashMap<&u32, usize> = update
        .iter()
        .enumerate()
        .map(|(index, value)| (value, index))
        .collect();

    // Validate each element in the update
    for (index, element) in update.iter().enumerate() {
        if let Some(pages_associated_to_element) = rule_map.get(element) {
            for page in pages_associated_to_element {
                // chhecking if the page is in the update
                if let Some(&page_index) = indexes_mapping.get(page) {
                    // if the page index violates the rule, return false
                    if page_index < index {
                        println!(
                            "Rule violated brev: {:?} must come before {:?} in {:?}",
                            element, page, update
                        );
                        return false;
                    }
                }
            }
        }
    }

    true
}
