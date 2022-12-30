#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    fn fail_fn() {
        assert_eq!(1, 2)
    }
}
