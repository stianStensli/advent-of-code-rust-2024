pub fn get_value_at<T>(game: &Vec<Vec<T>>, x: usize, y: usize) -> Option<T>
where T: Copy
{
    match game.get(x) {
        Some(exist) =>  match exist.get(y) {
            Some(exist) => {
                Some(exist.to_owned())},
            _ => None
        },
        _ => None
    }
}

pub fn get_value_at_safe<T>(game: &Vec<Vec<T>>, x: i32, y: i32) -> Option<T>
where T: Copy
{
    if x < 0 || y < 0 {
        return None;
    }
    get_value_at(game, x as usize, y as usize)
}
pub fn main() {}
