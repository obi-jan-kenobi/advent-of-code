fn ring(val: u32) -> u32 {
    let mut sum = 1;
    let mut ring: u32 = 0;
    while sum < val {
        ring = ring + 1;
        sum = sum + ring * 8;
    }
    ring
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
    #[test]
    fn ring_works() {
        assert_eq!(0, ring(1));
        assert_eq!(1, ring(2));
        assert_eq!(1, ring(5));
        assert_eq!(1, ring(9));
        assert_eq!(2, ring(10));
        assert_eq!(2, ring(12));
        assert_eq!(2, ring(25));
        assert_eq!(3, ring(26));
    }
}
