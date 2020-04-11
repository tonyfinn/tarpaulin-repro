fn for_tests() -> u32 {
    42
}

fn main() {
    println!("Hello, world!");
    panic!("Tarpaulin should not run this");
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(for_tests(), 42);
    }
}
