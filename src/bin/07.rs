advent_of_code::solution!(7);

fn add_or_multi(to_check: u64, values: Vec<u64>, use_concat: bool) -> u64 {
    let first = values[0];
    if values.len() == 1 || first > to_check {
        return if to_check == first { to_check } else { 0 };
    }

    let second = values[1];
    let base: Vec<u64> = values.into_iter().skip(2).collect();

    if use_concat {
        let concat = &mut first.clone().to_string();
        let mut new_concat: Vec<u64> = base.clone();
        concat.push_str(&*&second.clone().to_string());
        new_concat.insert(0, concat.parse::<u64>().unwrap());
        let res = add_or_multi(to_check, new_concat, use_concat);
        if  res > 0 {
            return res
        }
    }

    let mut new: Vec<u64> = base.clone();
    new.insert(0, first + second);

    if add_or_multi(to_check, new, use_concat) > 0 {
        return to_check
    }

    let mut new: Vec<u64> = base.clone();
    new.insert(0, first * second);

    add_or_multi(to_check, new, use_concat)
}


pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut line = line.split(':');
                let value = line.next().unwrap().parse::<u64>().unwrap();
                let factors = line
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                let mut max = 0;
                let mut min = 0;
                factors.iter().for_each(|f|
                    if max == 0 {
                        max = *f;
                        min = *f;
                    } else if *f == 1 {
                        max += *f;
                        min *= *f;
                    } else {
                        max *= f;
                        min += *f;
                    }
                );

                if max < value && min > value {
                    0
                }else if max == value || min == value {
                    value
                } else {
                    add_or_multi(value, factors, false)
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut line = line.split(':');
                let value = line.next().unwrap().parse::<u64>().unwrap();
                let factors = line
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                add_or_multi(value, factors, true)
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
