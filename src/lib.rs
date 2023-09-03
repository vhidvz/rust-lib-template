pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    extern crate dotenv;

    use super::*;

    fn init() {
        dotenv::dotenv().ok();
    }

    #[test]
    fn it_works() {
        tests::init();

        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
