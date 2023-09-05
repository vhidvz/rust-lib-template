extern crate log;

pub fn add(left: usize, right: usize) -> usize {
    log::info!("add {}, {}", left, right);
    left + right
}

#[cfg(test)]
mod tests {
    extern crate dotenv;
    extern crate env_logger;

    use super::*;
    use std::env;

    fn init() {
        dotenv::dotenv().ok();

        let dev = env::var("RUST_ENV")
            .unwrap_or("dev".to_string())
            .to_lowercase()
            .matches("dev")
            .count()
            > 0;

        let _ = env_logger::builder().is_test(dev).try_init();
    }

    #[test]
    fn test_add() {
        tests::init();

        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
