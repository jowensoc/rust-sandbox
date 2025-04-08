
#[cfg(test)]
mod dummy_test {

    #[test]
    fn add_should_pass() {
        assert_eq!(2, 1 + 1);
    }

    #[test]
    fn add_should_fail() {
        assert_ne!(2, 1 + 2);
    }

}