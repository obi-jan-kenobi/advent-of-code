fn main() {
    println!("Hello, world!");
}

struct Mem {
    value: u32,
    x: i32,
    y: i32,
}

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn create_grid(size: u32) -> Vec<Mem> {
    let dir = Direction::RIGHT;
}

fn manhatten(value: u32) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works_for_examples() {
        assert_eq!(0, manhatten(1));
        assert_eq!(3, manhatten(12));
        assert_eq!(2, manhatten(23));
        assert_eq!(31, manhatten(1024));
    }
}
