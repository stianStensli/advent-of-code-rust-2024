advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ids: Vec<u32> = Vec::new();
    let mut leng: Vec<u32> = Vec::new();
    input.lines().for_each(|line| {
            let mut parts = line.split_whitespace();
            //let f: i32 = parts[0]..parse();
            //et mut isinit = true;
            let f = parts.next().unwrap();
            let f1:u32 = f.parse().unwrap();
            ids.push(f1);
            let s = parts.next().unwrap();
            let s1:u32 = s.parse().unwrap();
            leng.push(s1);
        });
    ids.sort();
    leng.sort();
    
    let it = ids.iter().zip(leng.iter());
    Some(it.map(|x| {
        x.0.abs_diff(*x.1)
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ids: Vec<u32> = Vec::new();
    let mut leng: Vec<u32> = Vec::new();
    input.lines().for_each(|line| {
            let mut parts = line.split_whitespace();
            //let f: i32 = parts[0]..parse();
            //et mut isinit = true;
            let f = parts.next().unwrap();
            let f1:u32 = f.parse().unwrap();
            ids.push(f1);
            let s = parts.next().unwrap();
            let s1:u32 = s.parse().unwrap();
            leng.push(s1);
        });
    

    Some(ids.iter().map(|v| {
        let o: u32 = leng.iter().filter(|f| *f == v).count() as u32;
        *v * o
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(31));
    }
}
