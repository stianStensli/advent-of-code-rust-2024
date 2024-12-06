use advent_of_code::advent_util::get_value_at;

advent_of_code::solution!(4);

fn get_point(game: &Vec<Vec<char>>, x: usize, y: usize, dir: u32, next_char: &char)
             -> Option<(usize, usize, u32)> {
    let next_char = *next_char;
    let value = get_value_at(game, x, y);

    match value {
        Some(value) => {
            if value == next_char {
                Some((x, y, dir))
            } else {
                None
            }
        }
        _ => None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let game = input.lines()
        .map(|line| line.chars().into_iter().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut start_points:Vec<(usize, usize, u32)> = Vec::new();

    for i in 0..game.len() {
        let row = &game[i];
        for j in 0..row.len() {
            if row[j] == 'X' {
                start_points.push((i, j, 0));
            }
        }
    }

    for next_char in ['M', 'A', 'S'] {
        let mut next_point:Vec<(usize, usize, u32)> = Vec::new();
        for point in &start_points {
            let can_back = point.1 > 0 &&
                (point.2 == 0 || point.2 == 1);
            let can_back_up = point.0 > 0 && point.1 > 0  &&
                (point.2 == 0 || point.2 == 2);
            let can_up = point.0 > 0 &&
                (point.2 == 0 || point.2 == 3);
            let can_f_up = point.0 > 0 &&
                (point.2 == 0 || point.2 == 4);
            let can_f = point.2 == 0 || point.2 == 5;
            let can_f_d = point.2 == 0 || point.2 == 6;
            let can_d = point.2 == 0 || point.2 == 7;
            let can_d_b = point.1 > 0 &&
                (point.2 == 0 || point.2 == 8);
            //println!("{:?}",can_d_b);
            if can_back {
                let x = point.0;
                let y = point.1 - 1;
                match get_point(&game, x, y, 1, &next_char) {
                    Some(point) => {next_point.push(point);},
                    _ => ()
                }

            }
            if can_back_up {
                let x = point.0 - 1;
                let y = point.1 - 1;
                match get_point(&game, x, y, 2, &next_char) {
                    Some(point) => {next_point.push(point);},
                    _ => ()
                }
            }
            if can_up {
                let x = point.0 - 1;
                let y = point.1;
                match get_point(&game, x, y, 3, &next_char){
                    Some(point) => {next_point.push(point);},
                    _ => ()
                }
            }
            if can_f_up {
                let x = point.0 - 1;
                let y = point.1 + 1;
                match get_point(&game, x, y, 4, &next_char){
                    Some(point) => {next_point.push(point);},
                    _ => ()
                }
            }
            if can_f {
                let x = point.0;
                let y = point.1 + 1;
                match get_point(&game, x, y, 5, &next_char){
                    Some(point) => {next_point.push(point);},
                    _ => ()
                }
            }
            if can_f_d {
                let x = point.0 + 1;
                let y = point.1 + 1;
                match get_point(&game, x, y, 6, &next_char){
                    Some(point) => {next_point.push(point);},
                    _ => ()
                }
            }
            if can_d {
                let x = point.0 + 1;
                let y = point.1;
                match get_point(&game, x, y, 7, &next_char){
                    Some(point) => {next_point.push(point);},
                    _ => ()
                }
            }
            if can_d_b {
                let x = point.0 + 1;
                let y = point.1 - 1;
                match get_point(&game, x, y, 8, &next_char){
                    Some(point) => {next_point.push(point);},
                    _ => ()
                }
            }
        }
        start_points = next_point;
    }

    Some(start_points.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let game = input.lines()
        .map(|line| line.chars().into_iter().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut start_points:Vec<(usize, usize)> = Vec::new();

    for i in 0..game.len() {
        let row = &game[i];
        for j in 0..row.len() {
            if row[j] == 'A' {
                start_points.push((i, j));
            }
        }
    }

    let mut xmas= 0;
    for point in &start_points {
        let can_back_up = point.0 > 0 && point.1 > 0;
        let can_f_up = point.0 > 0;
        let can_d_b = point.1 > 0;
        if !can_back_up || !can_f_up || !can_d_b {
            continue
        }
        let back_up = get_value_at(&game, point.0 - 1, point.1 - 1).unwrap_or('.');
        let f_d = get_value_at(&game, point.0 + 1, point.1 + 1).unwrap_or('.');
        if !((back_up == 'M' && f_d == 'S') ||
            (back_up == 'S' && f_d == 'M')) {
            continue
        }
        let f_up = get_value_at(&game, point.0 - 1, point.1 + 1).unwrap_or('.');
        let f_d_b = get_value_at(&game, point.0 + 1, point.1 - 1).unwrap_or('.');
        if !((f_up == 'M' && f_d_b == 'S') ||
            (f_up == 'S' && f_d_b == 'M')) {
            continue
        }
        xmas +=1;
    }
    Some(xmas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {

        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(9));
    }
}
