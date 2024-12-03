advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i64> {
    Some(input.lines().map(|line| {
        let mut last: u32 = 0;
        let mut init = true;
        let mut result = 1;
        let mut dir = 0;
        line.split_whitespace().for_each(|value| {
            if result == 0 {
                return;
            };
            if init {
                init = false;
                last = value.parse().unwrap();
            } else {
                let next: u32 = value.parse().unwrap();
                if next == last {
                    result = 0;
                    return;
                }
                if next.abs_diff(last) < 4 {
                    let new_dir = if next > last  { 2 } else { 1 };
                    if dir == 0 {
                        dir = new_dir;
                        last = next;
                        return;
                    }
                    if new_dir == dir {
                        last = next;
                        return;
                    }
                }
                result = 0;
                last = next;
                return;
            }
        });
        result
    }).sum())
}

pub fn is_ok2(line: Vec<u32>) -> bool {
    let mut last: u32 = 0;
    let mut init = true;
    let mut result = 1;
    let mut dir = 0;
    line.iter().for_each(|next| {
        let next = *next;
        if result == 0 {
            return;
        };
        if init {
            init = false;
            last = next
        } else {
            if next == last {
                result = 0;
                return;
            }
            if next.abs_diff(last) < 4 {
                let new_dir = if next > last  { 2 } else { 1 };
                if dir == 0 {
                    dir = new_dir;
                    last = next;
                    return;
                }
                if new_dir == dir {
                    last = next;
                    return;
                }
            }
            result = 0;
            last = next;
            return;
        }
    });
    result == 1
}

pub fn is_ok(line: Vec<&u32>) -> bool {
    let mut last: u32 = 0;
    let mut init = true;
    let mut result = 1;
    let mut dir = 0;
    line.iter().for_each(|next| {
        let next = **next;
        if result == 0 {
            return;
        };
        if init {
            init = false;
            last = next
        } else {
            if next == last {
                result = 0;
                return;
            }
            if next.abs_diff(last) < 4 {
                let new_dir = if next > last  { 2 } else { 1 };
                if dir == 0 {
                    dir = new_dir;
                    last = next;
                    return;
                }
                if new_dir == dir {
                    last = next;
                    return;
                }
            }
            result = 0;
            last = next;
            return;
        }
    });
    result == 1
}


pub fn part_two(input: &str) -> Option<i64> {
    Some(input.lines().map(|line| {
        let values: Vec<u32> = line.split_whitespace()
            .map(|l| l.parse().unwrap()).collect();
        let temp:Vec<&u32> = values.iter().collect();
        if is_ok(temp) {
            return 1
        }
        for skip in 0..values.len() {
            let temp: Vec<u32> = values.clone().into_iter().enumerate()
                .filter(|&(i, _)| i != skip)
                .map(|(_, v)| v)
                .collect::<Vec<_>>();
            if is_ok2(temp) {
                return 1
            }
        }
        0
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
