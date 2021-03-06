mod paintfuck;

#[cfg(test)]
mod tests {
    use crate::paintfuck;

    #[test]
    fn zeros() {
        assert_eq!(paintfuck::interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 0, 6, 9), String::from("000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should initialize all cells in the datagrid to 0");
    }

    #[test]
    fn iterations() {
        assert_eq!(paintfuck::interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 7, 6, 9), String::from("111100\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should adhere to the number of iterations specified");
    }

    #[test]
    fn traversal() {
        assert_eq!(paintfuck::interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 19, 6, 9), String::from("111100\r\n000010\r\n000001\r\n000010\r\n000100\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should traverse the 2D datagrid correctly");
        assert_eq!(paintfuck::interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 42, 6, 9), String::from("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should traverse the 2D datagrid correctly for all of the \"n\", \"e\", \"s\" and \"w\" commands");
    }

    #[test]
    fn termination() {
        assert_eq!(paintfuck::interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 100, 6, 9), String::from("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should terminate normally and return a representation of the final state of the 2D datagrid when all commands have been considered from left to right even if the number of iterations specified have not been fully performed");
    }

    #[test]
    fn exit() {
        assert_eq!(paintfuck::interpreter("eeese[e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s]*", 100, 6, 9), String::from("000000\r\n000010\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Interpreter should skip if the point below is not flipped");
        assert_eq!(paintfuck::interpreter("eeese[e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s]*", 7, 6, 9), String::from("000000\r\n000010\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Interpreter should count jumps as using one life only");
    }

    #[test]
    fn jump() {
        assert_eq!(paintfuck::interpreter("eees*[e]*", 100, 6, 9), String::from("000000\r\n000110\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Interpreter should not jump if the point is flipped");
    }

    #[test]
    fn toroidal() {
        assert_eq!(paintfuck::interpreter("w*n*se*", 100, 6, 9), String::from("100001\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000001"), "Interpreter should loop at the edges of the board");
        assert_eq!(
            paintfuck::interpreter("*[es*]", 37, 5, 6),
            String::from("11000\r\n01100\r\n00110\r\n00011\r\n00001\r\n10000"),
            "[codewars test] Interpreter should exhibit Toroidal behavior"
        );
    }

    #[test]
    fn comments() {
        assert_eq!(paintfuck::interpreter("wOO*NWn_*se*00", 100, 6, 9), String::from("100001\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000001"), "Interpreter ignores comments");
    }
}
