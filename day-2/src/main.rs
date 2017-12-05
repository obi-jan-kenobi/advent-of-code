fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works_for_rows() {
        assert_eq!(8, row_diff("5 1 9 5"));
        assert_eq!(4, row_diff("7 5 3"));
        assert_eq!(6, row_diff("2 4 6 8"));
    }
    fn works_for_example() {
        assert_eq!(18, "5 1 9 5\n7 5 3\n2 4 6 8");
    }
}
