use std::collections::HashMap;

advent_of_code::solution!(8);

fn double_dist_calc(p1: (usize, usize), p2: (usize, usize), times: i32) -> ((i32, i32), (i32, i32)) {
    let diff_x = p1.0.abs_diff(p2.0) as i32 * times;
    let diff_y = p1.1.abs_diff(p2.1) as i32 * times;
    let f_x;
    let s_x;
    if p1.0 < p2.0 {
        f_x = p1.0 as i32 - diff_x;
        s_x = p2.0 as i32 + diff_x;
    } else {
        f_x = p1.0 as i32 + diff_x;
        s_x = p2.0 as i32 - diff_x;
    }
    let f_y;
    let s_y;
    if p1.1 < p2.1 {
        f_y = p1.1 as i32 - diff_y;
        s_y = p2.1 as i32 + diff_y;
    } else {
        f_y = p1.1 as i32 + diff_y;
        s_y = p2.1 as i32 - diff_y;
    };
    ((f_x, f_y), (s_x, s_y))
}

fn fill_if_ok(point: (i32, i32), stupid: &mut Vec<Vec<char>>) -> bool {
    if point.0 >= 0 && point.1 >= 0 {
        let x = point.0 as usize;
        let y = point.1 as usize;
        if stupid.get(x).is_some() && stupid.get(x).unwrap().get(y).is_some() {
            stupid[point.0 as usize][point.1 as usize] = '#';
            return true;
        }
    }
    false
}

fn get_harmonic_points(
    cmp: (usize, usize),
    locs: Vec<(usize, usize)>,
    stupid: &mut Vec<Vec<char>>,
) {
    if locs.is_empty() {
        return;
    }
    let first = locs[0];
    let mut i = 0;
    loop {
        let res = double_dist_calc(cmp, first, i);
        let r1 = fill_if_ok(res.0, stupid);
        let r2 = fill_if_ok(res.1, stupid);
        if !r1 && !r2 {
            break;
        }
        i += 1;
    }
    let next_list = locs.into_iter().skip(1).collect::<Vec<(usize, usize)>>();
    get_harmonic_points(cmp, next_list, stupid)
}
fn get_points(cmp: (usize, usize), locs: Vec<(usize, usize)>, stupid: &mut Vec<Vec<char>>) {
    if locs.is_empty() {
        return;
    }
    let first = locs[0];
    let opt_points = double_dist_calc(cmp, first, 1);

    fill_if_ok(opt_points.0, stupid);
    fill_if_ok(opt_points.1, stupid);

    let next_list = locs.into_iter().skip(1).collect::<Vec<(usize, usize)>>();
    get_points(cmp, next_list, stupid)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut antenna_loc: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut stupid: Vec<Vec<char>> = Vec::new();
    input.lines().for_each(|l| {
        let mut line = Vec::new();
        l.chars().for_each(|c| {
            line.push('.');
            if c == '.' {
                return;
            }
            antenna_loc.entry(c).or_insert(vec![]).push((stupid.len(), line.len() -1));
        });
        stupid.push(line);
    });
    for (_, val) in antenna_loc {
        for i in 0..val.len() {
            let first = val[i];
            let list = val.clone();
            let new_list = list.into_iter().skip(i + 1).collect();
            get_points(first, new_list, &mut stupid);
        }
    }
    Some(
        stupid
            .iter()
            .map(|l| l.iter().map(|v| if *v == '#' { 1 } else { 0 }).sum::<u64>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut antenna_loc: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut stupid: Vec<Vec<char>> = Vec::new();
    input.lines().for_each(|l| {
        let mut line = Vec::new();
        l.chars().for_each(|c| {
            line.push('.');
            if c == '.' {
                return;
            }
            antenna_loc
                .entry(c)
                .or_insert(vec![])
                .push((stupid.len(), line.len() - 1));
        });
        stupid.push(line);
    });
    for (_, val) in antenna_loc {
        for i in 0..val.len() {
            let first = val[i];
            let list = val.clone();
            let new_list = list.into_iter().skip(i + 1).collect();
            get_harmonic_points(first, new_list, &mut stupid);
        }
    }

    Some(
        stupid
            .iter()
            .map(|l| l.iter().map(|v| if *v == '#' { 1 } else { 0 }).sum::<u64>())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        /*let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));*/

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(9));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
