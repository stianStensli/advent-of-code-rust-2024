use std::collections::HashMap;
use std::ops::AddAssign;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ids: Vec<u32> = Vec::new();
    let mut leng: Vec<u32> = Vec::new();
    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        let f = parts.next().unwrap().parse().unwrap();
        let s = parts.next().unwrap().parse().unwrap();
        ids.push(f);
        leng.push(s);
    });
    ids.sort();
    leng.sort();

    Some(
        ids.iter()
            .zip(leng.iter())
            .map(|x| x.0.abs_diff(*x.1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ids: Vec<u32> = Vec::new();
    let mut sum_list: HashMap<u32, u32> = HashMap::new();
    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        let f = parts.next().unwrap().parse().unwrap();
        let s = parts.next().unwrap().parse().unwrap();

        ids.push(f);
        match sum_list.get_mut(&s) {
            Some(exist) => exist.add_assign(s),
            _ => {
                sum_list.insert(s, s);
                ()
            }
        }
    });

    Some(ids.iter().map(|v| sum_list.get(v).unwrap_or(&0)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
