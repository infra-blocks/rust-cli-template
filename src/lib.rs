pub fn big_function() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_big_function() {
        big_function();
    }
}
