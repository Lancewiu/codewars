mod lightstate;

#[cfg(test)]
mod tests {
    use crate::lightstate;

    #[test]
    fn it_works() {
        assert_eq!(lightstate::update_light("green"), "yellow");
        assert_eq!(lightstate::update_light("yellow"), "red");
        assert_eq!(lightstate::update_light("red"), "green");
    }
}
