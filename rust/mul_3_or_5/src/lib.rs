mod sum_mul_three_or_five;

#[cfg(test)]
mod tests {
    use crate::sum_mul_three_or_five::solution;
    #[test]
    fn provided() {
        assert_eq!(solution(10), 23);
        assert_eq!(solution(11), 33);
        assert_eq!(solution(6), 8);
    }

    #[test]
    fn lower_edge() {
        assert_eq!(solution(2), 0);
    }

    #[test]
    fn invalid_arg() {
        assert_eq!(solution(-1), 0);
    }
}
