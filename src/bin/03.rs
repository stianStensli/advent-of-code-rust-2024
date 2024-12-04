use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");

    Some(
        mul_regex
            .captures_iter(input)
            .map(|m| {
                let first = m.get(1).unwrap().as_str().parse::<u64>().unwrap();
                let second = m.get(2).unwrap().as_str().parse::<u64>().unwrap();
                first * second
            })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mul_regex =
        Regex::new(r"(don't\(\))|(do\(\))|(mul\((\d+),(\d+)\))").expect("Invalid regex");

    let mut is_a_go = true;
    Some(
        mul_regex
            .captures_iter(input)
            .map(|c| {
                if c.get(1).is_some() {
                    is_a_go = false;
                } else if c.get(2).is_some() {
                    is_a_go = true;
                } else if c.get(3).is_some() {
                    if is_a_go {
                        return c.get(4).unwrap().as_str().parse::<u64>().unwrap()
                            * c.get(5).unwrap().as_str().parse::<u64>().unwrap();
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
