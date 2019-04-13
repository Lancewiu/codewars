mod mbm;

#[cfg(test)]
mod tests {
    use crate::mbm;

    #[test]
    fn basic_tests() {
        // flip every bit
        assert_eq!(mbm::interpreter("10", "1010101"), "0101010");
        // flip every other bit
        assert_eq!(mbm::interpreter("100", "1111111111"), "0101010101");
        // donothing
        assert_eq!(mbm::interpreter("110", "00000000000"), "00000000000");
        assert_eq!(mbm::interpreter("000", "00000000000"), "00000000000");

        // flip second bit
        assert_eq!(mbm::interpreter("010", "0000000000"), "0101010101");

        assert_eq!(mbm::interpreter("0010", "0000000000"), "0010010010");

        assert_eq!(mbm::interpreter("001110111110", "0000000000"), "0011001100");
    }
}
