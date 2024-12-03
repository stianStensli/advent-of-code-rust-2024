use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");

    Some(input.lines()
             .map(|line| {
                  mul_regex.captures_iter(line).map(|m| {
                      //println!("{}",m.get(0).unwrap().as_str());
                     let first = m.get(1).unwrap().as_str().parse::<u64>().unwrap();
                     let second = m.get(2).unwrap().as_str().parse::<u64>().unwrap();
                     first*second
                 }).sum::<u64>()
             })
        .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines()
        .map(|line| {
            let mut is_dont = false;
            let mut is_dont_index = 0;
            line.split("don't()").map(|dt| {
                if is_dont_index != 0 {
                    is_dont = true;
                }
                is_dont_index += 1;
                let mut is_do_index = 0;
                dt.split("do()").map(|d| {
                    if is_do_index != 0 {
                        is_dont = false;
                    }
                    is_do_index += 1;
                    //println!("str: {:?}, dont: {:}, do: {:}", d, is_dont_index, is_do_index);
                    if is_dont {
                       //println!("return");
                        return 0;
                    }
                    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
                    let res = mul_regex.captures_iter(d).map(|m| {
                        //println!("{}",m.get(0).unwrap().as_str());
                        let first = m.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        let second = m.get(2).unwrap().as_str().parse::<u64>().unwrap();
                        first*second
                    }).sum::<u64>();
                    //println!("{}", res);
                    res
                }).sum::<u64>()
            }).sum::<u64>()
        })
        .sum())
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
