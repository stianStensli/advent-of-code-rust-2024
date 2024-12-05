use std::cmp::min;
use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut is_rules = true;
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    Some(
        input
            .lines()
            .map(|l| {
                if l.is_empty() {
                    is_rules = false;
                    return 0;
                }
                if is_rules {
                    let pair = l.split('|').collect::<Vec<_>>();
                    let key = pair[0].parse::<u32>().unwrap();
                    let value = pair[1].parse::<u32>().unwrap();
                    let rule_set = rules.get_mut(&key);
                    if let Some(rule_set) = rule_set {
                        rule_set.push(value);
                    } else {
                        rules.insert(key, vec![value]);
                    }
                    return 0;
                } else {
                    let page = l
                        .split(',')
                        .map(|x| x.parse::<u32>().unwrap_or(0))
                        .collect::<Vec<u32>>();
                    let mut prev_values = Vec::new();
                    for i in 0..page.len() {
                        let value = page[i];
                        let rule_set = rules.get(&value);
                        if let Some(rule_set) = rule_set {
                            for j in 0..rule_set.len() {
                                let r = rule_set[j];
                                if prev_values.contains(&r) {
                                    return 0;
                                }
                            }
                        }
                        prev_values.push(value)
                    }

                    let i = (page.len() - 1) / 2;
                    return page[i];
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut is_rules = true;
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    Some(input.lines().map(|l| {
        return if l.is_empty() {
            is_rules = false;
            0
        } else if is_rules {
            let pair = l.split('|').collect::<Vec<_>>();
            let key = pair[0].parse::<u32>().unwrap();
            let value = pair[1].parse::<u32>().unwrap();
            let rule_set = rules.get_mut(&key);
            if let Some(rule_set) = rule_set {
                rule_set.push(value);
            } else {
                rules.insert(key, vec![value]);
            }
            0
        } else {
            let page = l
                .split(',')
                .map(|x| x.parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>();
            let mut prev_values = Vec::new();
            let mut error_values = Vec::new();
            'outer: for i in 0..page.len() {
                let value = page[i];
                let rule_set = rules.get(&value);
                if let Some(rule_set) = rule_set {
                    for j in 0..rule_set.len() {
                        let r = rule_set[j];
                        if prev_values.contains(&r) {
                            error_values.push(value);
                            continue 'outer;
                        }
                    }
                }
                prev_values.push(value)
            }

            if error_values.len() > 0 {
                let mut isplaced = Vec::new();
                loop {
                    for error_index in 0..error_values.len() {
                        if isplaced.contains(&error_index) {
                            continue;
                        }
                        let error_value = error_values[error_index];
                        let rule_set = rules.get(&error_value);
                        if let Some(rule_set) = rule_set {
                            let mut lowest_index = 99999999;
                            let mut all_rules_match = true;
                            for rule in rule_set {
                                if !error_values.contains(rule) && !prev_values.contains(rule) {
                                    continue;
                                }
                                let mut rule_match = false;
                                for prev_values_index in 0..prev_values.len() {
                                    if prev_values[prev_values_index] == *rule {
                                        lowest_index = min(lowest_index, prev_values_index);
                                        rule_match = true;
                                        break;
                                    }
                                }
                                all_rules_match = rule_match && all_rules_match
                            }
                            if all_rules_match {
                                prev_values.insert(lowest_index, error_value);
                                isplaced.push(error_index);
                            }
                        } else {panic!("not in ruleset")}
                    }
                    if error_values.len() == isplaced.len() {
                        break;
                    }
                }
                let i = (prev_values.len() - 1) / 2;
                prev_values[i]
            } else {
                0
            }
        }
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
