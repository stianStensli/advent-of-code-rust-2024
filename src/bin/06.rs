advent_of_code::solution!(6);

use advent_of_code::advent_util::get_value_at_safe;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u64> {
    let mut game = input
        .lines()
        .map(|line| line.chars().into_iter().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut start_points: Vec<(i32, i32, u32)> = Vec::new();
    //println!("started:");
    for i in 0..game.len() {
        let row = &game[i];
        for j in 0..row.len() {
            if row[j] == '^' {
                start_points.push((i as i32, j as i32, 3));
            } else if row[j] == '>' {
                start_points.push((i as i32, j as i32, 5));
            } else if row[j] == 'v' {
                start_points.push((i as i32, j as i32, 7));
            } else if row[j] == '<' {
                start_points.push((i as i32, j as i32, 1));
            }
        }
    }
    for start_point in &mut start_points {
        game[start_point.0 as usize][start_point.1 as usize] = '.'
    }
    /*
    for i in 0..game.len(){
        for j in 0..game[i].len(){
            print!("{}", game[i][j]);
        }
        print!("\n");
    }*/
    let mut visited = HashSet::new();
    let mut iters = 0;
    'main: loop {
        iters += 1;
        let mut next_start_points: Vec<(i32, i32, u32)> = Vec::new();
        for point in &start_points {
            let x: i32 = point.0;
            let y: i32 = point.1;
            visited.insert((x, y));
            //println!("{:?}",can_d_b);
            if point.2 == 1 {
                // back
                let x: i32 = point.0 as i32;
                let y: i32 = (point.1 - 1) as i32;
                match get_value_at_safe(&game, x, y) {
                    Some(char_value) => {
                        if char_value == '.' {
                            let x: i32 = point.0;
                            let y: i32 = point.1 - 1;
                            next_start_points.push((x, y, 1));
                        } else {
                            let x = point.0 - 1;
                            let y = point.1;
                            next_start_points.push((x, y, 3));
                        }
                    }
                    _ => (),
                }
            }
            if point.2 == 3 {
                let x = point.0 - 1;
                let y = point.1;
                match get_value_at_safe(&game, x, y) {
                    Some(char_value) => {
                        if char_value == '.' {
                            next_start_points.push((x, y, 3));
                        } else {
                            let x = point.0;
                            let y = point.1 + 1;
                            next_start_points.push((x, y, 5));
                        }
                    }
                    _ => (),
                }
            }
            if point.2 == 5 {
                let x = point.0;
                let y = point.1 + 1;
                match get_value_at_safe(&game, x, y) {
                    Some(char_value) => {
                        if char_value == '.' {
                            next_start_points.push((x, y, 5));
                        } else {
                            let x = point.0 + 1;
                            let y = point.1;
                            next_start_points.push((x, y, 7));
                        }
                    }
                    _ => (),
                }
            }
            if point.2 == 7 {
                let x = point.0 + 1;
                let y = point.1;
                match get_value_at_safe(&game, x, y) {
                    Some(char_value) => {
                        if char_value == '.' {
                            next_start_points.push((x, y, 7));
                        } else {
                            let x: i32 = point.0;
                            let y: i32 = point.1 - 1;
                            next_start_points.push((x, y, 1));
                        }
                    }
                    _ => (),
                }
            }
        }
        if next_start_points.is_empty() || iters == 50000 {
            break 'main;
        } else {
            /* println!("iter: {}", iters);
            for i in 0..game.len(){
                for j in 0..game[i].len(){
                    if visited.get(&(i as i32, j as i32)).is_some() {
                        print!("X");
                    } else {
                        print!("{}", game[i][j]);
                    }
                }
                print!("\n");
            }
            */
            start_points = next_start_points.clone();
        }
    }

    Some(visited.len() as u64)
}

pub fn is_loop(game: &Vec<Vec<char>>, strart_point: (i32, i32, u32)) -> bool {
    let mut visited: HashSet<(i32, i32, u32)> = HashSet::new();
    let mut point = strart_point;
    'main: loop {
        if visited.get(&point).is_some() {
            /*println!("iter: x",);
            for i in 0..game.len(){
                for j in 0..game[i].len(){
                    if i == strart_point.0 as usize && j == strart_point.1 as usize {
                        if strart_point.2 == 1 {
                            print!("<");
                        } else if strart_point.2 == 3 {
                            print!("^");
                        }else if strart_point.2 == 5{
                            print!(">");
                        }else if strart_point.2 == 7 {
                            print!("v");
                        }
                    }else {
                        print!("{}", game[i][j]);
                    }
                }
                print!("\n");
            }*/
            return true;
        }
        let mut next_start_points: Vec<(i32, i32, u32)> = Vec::new();
        let x: i32 = point.0;
        let y: i32 = point.1;
        visited.insert((x, y, point.2));
        //println!("{:?}",can_d_b);
        if point.2 == 1 {
            // back
            let x: i32 = point.0 as i32;
            let y: i32 = (point.1 - 1) as i32;
            match get_value_at_safe(&game, x, y) {
                Some(char_value) => {
                    if char_value == '.' {
                        let x: i32 = point.0;
                        let y: i32 = point.1 - 1;
                        next_start_points.push((x, y, 1));
                    } else {
                        let x = point.0 - 1;
                        let y = point.1;
                        next_start_points.push((x, y, 3));
                    }
                }
                _ => (),
            }
        }
        if point.2 == 3 {
            let x = point.0 - 1;
            let y = point.1;
            match get_value_at_safe(&game, x, y) {
                Some(char_value) => {
                    if char_value == '.' {
                        next_start_points.push((x, y, 3));
                    } else {
                        let x = point.0;
                        let y = point.1 + 1;
                        next_start_points.push((x, y, 5));
                    }
                }
                _ => (),
            }
        }
        if point.2 == 5 {
            let x = point.0;
            let y = point.1 + 1;
            match get_value_at_safe(&game, x, y) {
                Some(char_value) => {
                    if char_value == '.' {
                        next_start_points.push((x, y, 5));
                    } else {
                        let x = point.0 + 1;
                        let y = point.1;
                        next_start_points.push((x, y, 7));
                    }
                }
                _ => (),
            }
        }
        if point.2 == 7 {
            let x = point.0 + 1;
            let y = point.1;
            match get_value_at_safe(&game, x, y) {
                Some(char_value) => {
                    if char_value == '.' {
                        next_start_points.push((x, y, 7));
                    } else {
                        let x: i32 = point.0;
                        let y: i32 = point.1 - 1;
                        next_start_points.push((x, y, 1));
                    }
                }
                _ => (),
            }
        }

        if next_start_points.is_empty() {
            break 'main;
        } else {
            point = next_start_points[0];
        }
    }
    false
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
    let mut obstic = HashSet::new();
    let mut iters = 0;
    let mut visited = HashSet::new();
    obstic.insert((point.0, point.1));
   // println!("start: {}, {}", point.0, point.1);
    'main: loop {
        iters += 1;
        visited.insert((point.0, point.1));
        let mut next_start_points: Vec<(i32, i32, u32)> = Vec::new();
            if point.2 == 1 {
                // back
                let x: i32 = point.0 as i32;
                let y: i32 = (point.1 - 1) as i32;
                match get_value_at_safe(&game, x, y) {
                    Some(char_value) => {
                        if char_value == '.' {
                            let x: i32 = point.0;
                            let y: i32 = point.1 - 1;
                            next_start_points.push((x, y, 1));
                            let mut loop_game = game.clone();
                            loop_game[x as usize][y as usize] = '#';
                            if is_loop(&loop_game, start_point) {
                                obstic.insert((x,y));
                                //println!("obstic: {}, {}", x,y);
                            }
                        } else {
                            let x = point.0 - 1;
                            let y = point.1;
                            next_start_points.push((x, y, 3));
                        }
                    }
                    _ => (),
                }
            }
            if point.2 == 3 {
                let x = point.0 - 1;
                let y = point.1;
                match get_value_at_safe(&game, x, y) {
                    Some(char_value) => {
                        if char_value == '.' {
                            next_start_points.push((x, y, 3));
                            let mut loop_game = game.clone();
                            loop_game[x as usize][y as usize] = '#';
                            if is_loop(&loop_game, start_point) {
                                obstic.insert((x,y));
                                //println!("obstic: {}, {}", x,y);
                            }
                        } else {
                            let x = point.0;
                            let y = point.1 + 1;
                            next_start_points.push((x, y, 5));
                        }
                    }
                    _ => (),
                }
            }
            if point.2 == 5 {
                let x = point.0;
                let y = point.1 + 1;
                match get_value_at_safe(&game, x, y) {
                    Some(char_value) => {
                        if char_value == '.' {
                            next_start_points.push((x, y, 5));
                            let mut loop_game = game.clone();
                            loop_game[x as usize][y as usize] = '#';
                            if is_loop(&loop_game, start_point) {
                                obstic.insert((x,y));
                                //println!("obstic: {}, {}", x,y);
                            }
                        } else {
                            let x = point.0 + 1;
                            let y = point.1;
                            next_start_points.push((x, y, 7));
                        }
                    }
                    _ => (),
                }
            }
            if point.2 == 7 {
                let x = point.0 + 1;
                let y = point.1;
                match get_value_at_safe(&game, x, y) {
                    Some(char_value) => {
                        if char_value == '.' {
                            next_start_points.push((x, y, 7));
                            let mut loop_game = game.clone();
                            loop_game[x as usize][y as usize] = '#';
                            if is_loop(&loop_game, start_point) {
                                obstic.insert((x,y));
                                //println!("obstic: {}, {}", x,y);
                            }
                        } else {
                            let x: i32 = point.0;
                            let y: i32 = point.1 - 1;
                            next_start_points.push((x, y, 1));
                        }
                    }
                    _ => (),
                }
            }
        if next_start_points.is_empty() || iters == 1000000 {
            break 'main;
        } else {
            /*println!("iter: {}", iters);
            for i in 0..game.len(){
                for j in 0..game[i].len(){
                    if visited.get(&(i as i32, j as i32)).is_some() {
                        print!("X");
                    } else {
                        print!("{}", game[i][j]);
                    }
                }
                print!("\n");
            }*/

            point = next_start_points[0];
        }
    }
    Some(obstic.len() as u64 - 1)
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
