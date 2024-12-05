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
                        println!("v: {}, rs: {:?}, prev: {:?}", value, rule_set, prev_values);
                        if let Some(rule_set) = rule_set {
                            for j in 0..rule_set.len() {
                                let r = rule_set[j];
                                if prev_values.contains(&r) {
                                    println!("no go");
                                    return 0;
                                }
                            }
                        }
                        println!("go");
                        prev_values.push(value)
                    }

                    println!("go full!!");
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
    let mut error_list = Vec::new();
    input.lines().for_each(|l| {
        if l.is_empty() {
            is_rules = false;
            return;
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
            return;
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
                            error_list.push(page);
                            return;
                        }
                    }
                }
                prev_values.push(value)
            }
            return;
        }
    });
    println!("{:?}", error_list);
    Some(
        error_list
            .iter()
            .map(|page| {
                let mut prev_values = Vec::new();
                for i in 0..page.len() {
                    let value = page[i];
                    let rule_set = rules.get(&value);
                    if let Some(rule_set) = rule_set {
                        for j in 0..rule_set.len() {
                            let r = rule_set[j];
                            if prev_values.contains(&r) {
                                return;
                            }
                        }
                    }
                }
                0
            })
            .sum(),
    )
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
