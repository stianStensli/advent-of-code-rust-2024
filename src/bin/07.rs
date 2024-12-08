advent_of_code::solution!(7);

fn add_or_multi(to_check: u64, values: &Vec<u64>, use_concat: bool, sum: u64, index: usize) -> u64 {
    if index == values.len() || sum > to_check {
        return if to_check == sum { to_check } else { 0 };
    }
    let first = sum;
    let second = values[index];

    if use_concat {
        let concat = first * i64::from(10).pow(second.ilog(10) + 1) as u64 + second;
        let res = add_or_multi(to_check, values, use_concat, concat, index + 1);
        if res > 0 {
            return res;
        }
    }
    if add_or_multi(to_check, values, use_concat, sum + second, index + 1) > 0 {
        return to_check;
    }
    add_or_multi(to_check, values, use_concat, first * second, index + 1)
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
                factors.iter().for_each(|f| {
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
                });

                if max < value && min > value {
                    0
                } else if max == value || min == value {
                    value
                } else {
                    add_or_multi(value, &factors, false, factors[0], 1)
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

                add_or_multi(value, &factors, true, factors[0], 1)
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
