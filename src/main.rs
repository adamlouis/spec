fn main() {
    println!("Hello, world!");
}

// a simple no-op unit test
#[cfg(test)]
mod tests {

    #[test]
    fn example_test() {
        assert_eq!(2 + 2, 4);
    }
}
