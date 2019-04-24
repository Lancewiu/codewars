mod isomorphism;

#[cfg(test)]
mod tests {
    use isomorphism::*;

    #[test]
    fn sub_st_l_test() {
        assert!(sub_st_l(iso_bool())(true));
        assert!(!sub_st_l(iso_bool())(false));
        assert!(sub_st_l(iso_bool_not())(false));
    }
}
