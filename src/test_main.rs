#[cfg(test)]
mod tests {
    use self::calculate_median;

    #[test]
    fn test_calculate_median_odd() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let median = calculate_median(&values);
        assert_eq!(median, 3.0);
    }

    #[test]
    fn test_calculate_median_even() {
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let median = calculate_median(&values);
        assert_eq!(median, 2.5);
    }
}

