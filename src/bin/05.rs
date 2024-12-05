use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;

advent_of_code::solution!(5);

fn insert_to(is_larger_map: &mut HashMap<u32, HashMap<u32, bool>>, k: &u32, v: &u32, is_larger: bool) {
    if let Some(rule_set) = is_larger_map.get_mut(k) {
        rule_set.insert(*v, is_larger);
    } else {
        let mut temp = HashMap::new();
        temp.insert(*v, is_larger);
        is_larger_map.insert(*k, temp);
    };
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut is_rules = true;
    let mut is_larger: HashMap<u32, HashMap<u32, bool>> = HashMap::new();
    Some(input.lines().map(|l|
        if l.is_empty() {
            is_rules = false;
            0
        } else if is_rules {
            let pair = l.split('|').collect::<Vec<_>>();
            let small = pair[0].parse::<u32>().unwrap();
            let large = pair[1].parse::<u32>().unwrap();

            insert_to(&mut is_larger, &small, &large, false);
            insert_to(&mut is_larger, &large, &small, true);
           0
        } else {
            let page = l
                .split(',')
                .map(|x| x.parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>();
            let mut page_sorted = page.clone();
            page_sorted.sort_by(|x, y| if is_larger[x][y] {Greater} else {Less});
            let full_match = page.iter().zip(page_sorted).all(|(x, y)| *x == y);
            if full_match {
                let i = (page.len() - 1) / 2;
                return page[i];
            }
            return 0
        }).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut is_rules = true;
    let mut is_larger: HashMap<u32, HashMap<u32, bool>> = HashMap::new();
    Some(input.lines().map(|l|
        if l.is_empty() {
            is_rules = false;
            0
        } else if is_rules {
            let pair = l.split('|').collect::<Vec<_>>();
            let small = pair[0].parse::<u32>().unwrap();
            let large = pair[1].parse::<u32>().unwrap();

            insert_to(&mut is_larger, &small, &large, false);
            insert_to(&mut is_larger, &large, &small, true);
            0
        } else {
            let page = l
                .split(',')
                .map(|x| x.parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>();
            let mut page_sorted = page.clone();
            page_sorted.sort_by(|x, y| if is_larger[x][y] {Greater} else {Less});
            let full_match = page_sorted.iter().zip(page).all(|(x, y)| *x == y);
            if full_match {
                return 0
            }
            let i = (page_sorted.len() - 1) / 2;
            return page_sorted[i]
        }).sum::<u32>())
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
