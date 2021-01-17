

#[cfg(test)]
mod tests {

    #[test]
    fn test_pass() {
       assert_eq!(3, 3)
    }

    #[test]
    fn test_fail() {
        assert_eq!(3, -1)
     }
}

