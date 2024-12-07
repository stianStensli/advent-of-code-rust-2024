advent_of_code::solution!(6);

use advent_of_code::advent_util::get_value_at_safe;
use std::collections::HashSet;
const BACK:u32 = 1;
const UP:u32 = 3;
const FORWARD:u32 = 5;
const DOWN:u32 = 7;

pub fn part_one(input: &str) -> Option<u64> {
    let mut game = input
        .lines()
        .map(|line| line.chars().into_iter().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut start_point: (i32, i32, u32) = (0,0,0);
    for i in 0..game.len() {
        let row = &game[i];
        for j in 0..row.len() {
            if row[j] == '^' {
                start_point = (i as i32, j as i32, 3);
            } else if row[j] == '>' {
                start_point = (i as i32, j as i32, 5);
            } else if row[j] == 'v' {
                start_point = (i as i32, j as i32, 7);
            } else if row[j] == '<' {
                start_point = (i as i32, j as i32, 1);
            }
        }
    }
        game[start_point.0 as usize][start_point.1 as usize] = '.';

    let res = get_nr_visit(&game, start_point);
    Some(res.unwrap_or(0) as u64)
}
pub fn get_nr_visit(game: &Vec<Vec<char>>, strart_point: (i32, i32, u32)) -> Option<u32> {
    let mut visited = HashSet::new();
    let mut point = strart_point.clone();
    let mut visited_dir: HashSet<(i32, i32, u32)> = HashSet::new();
    'main: loop {
        if visited_dir.contains(&point.clone()) {
            return None
        }
        visited_dir.insert(point.clone());
        let mut next_start_points: Vec<(i32, i32, u32)> = Vec::new();
            let x: i32 = point.0;
            let y: i32 = point.1;
            visited.insert((x, y));
            //println!("{:?}",can_d_b);
            if point.2 == BACK {
                // back
                let x: i32 = point.0 as i32;
                let y: i32 = (point.1 - 1) as i32;
                match get_value_at_safe(&game, x, y) {
                    Some(char_value) => {
                        if char_value == '.' {
                            next_start_points.push((x, y, BACK));
                        } else {
                            next_start_points.push((point.0, point.1, UP));
                        }
                    }
                    _ => (),
                }
            }
            if point.2 == UP {
                let x = point.0 - 1;
                let y = point.1;
                match get_value_at_safe(&game, x, y) {
                    Some(char_value) => {
                        if char_value == '.' {
                            next_start_points.push((x, y, UP));
                        } else {
                            next_start_points.push((point.0, point.1, FORWARD));
                        }
                    }
                    _ => (),
                }
            }
            if point.2 == FORWARD {
                let x = point.0;
                let y = point.1 + 1;
                match get_value_at_safe(&game, x, y) {
                    Some(char_value) => {
                        if char_value == '.' {
                            next_start_points.push((x, y, FORWARD));
                        } else {
                            next_start_points.push((point.0, point.1, DOWN));
                        }
                    }
                    _ => (),
                }
            }
            if point.2 == DOWN {
                let x = point.0 + 1;
                let y = point.1;
                match get_value_at_safe(&game, x, y) {
                    Some(char_value) => {
                        if char_value == '.' {
                            next_start_points.push((x, y, DOWN));
                        } else {
                            next_start_points.push((point.0, point.1, BACK));
                        }
                    }
                    _ => (),
                }
            }
        if next_start_points.is_empty() {
            break 'main;
        } else {
            point = next_start_points[0].clone();
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut game = input
        .lines()
        .map(|line| line.chars().into_iter().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut point: (i32, i32, u32) = (0, 0, 0);
    for i in 0..game.len() {
        let row = &game[i];
        for j in 0..row.len() {
            if row[j] == '^' {
                point = (i as i32, j as i32, 3);
            } else if row[j] == '>' {
                point = (i as i32, j as i32, 5);
            } else if row[j] == 'v' {
                point = (i as i32, j as i32, 7);
            } else if row[j] == '<' {
                point = (i as i32, j as i32, 1);
            }
        }
    }
    let start_point = point.clone();
    game[point.0 as usize][point.1 as usize] = '.';

    let mut point = start_point.clone();
    let mut obstic: HashSet<(i32, i32)> = HashSet::new();
    let mut is_checked: HashSet<(i32, i32)> = HashSet::new();
    obstic.insert((start_point.0, start_point.1));
    'main: loop {
        let mut next_start_points: Vec<(i32, i32, u32)> = Vec::new();
        let x: i32 = point.0;
        let y: i32 = point.1;
        let mut temp = game.clone();
        temp[x as usize][y as usize] = '#';
        if !is_checked.contains(&(x, y)) &&
            get_nr_visit(&temp, point.clone()).is_none() {
            obstic.insert((x, y));
        }
        is_checked.insert((x, y));

        //println!("{:?}",can_d_b);
        if point.2 == BACK {
            // back
            let x: i32 = point.0 as i32;
            let y: i32 = (point.1 - 1) as i32;
            match get_value_at_safe(&game, x, y) {
                Some(char_value) => {
                    if char_value == '.' {
                        next_start_points.push((x, y, BACK))
                    } else {
                        next_start_points.push((point.0, point.1, UP))
                    }
                }
                _ => (),
            }
        };
        if point.2 == UP {
            let x = point.0 - 1;
            let y = point.1;
            match get_value_at_safe(&game, x, y) {
                Some(char_value) => {
                    if char_value == '.' {
                        next_start_points.push((x, y, UP));
                    } else {
                        next_start_points.push((point.0, point.1, FORWARD));
                    }
                }
                _ => (),
            }
        }
        if point.2 == FORWARD {
            let x = point.0;
            let y = point.1 + 1;
            match get_value_at_safe(&game, x, y) {
                Some(char_value) => {
                    if char_value == '.' {
                        next_start_points.push((x, y, FORWARD));
                    } else {
                        next_start_points.push((point.0, point.1, DOWN));
                    }
                }
                _ => (),
            }
        }
        if point.2 == DOWN {
            let x = point.0 + 1;
            let y = point.1;
            match get_value_at_safe(&game, x, y) {
                Some(char_value) => {
                    if char_value == '.' {
                        next_start_points.push((x, y, DOWN));
                    } else {
                        next_start_points.push((point.0, point.1, BACK));
                    }
                }
                _ => (),
            }
        }
        if next_start_points.is_empty() {
            break 'main;
        } else {
            point = next_start_points[0].clone();
        }
    }
    Some(obstic.len() as u64 -1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
