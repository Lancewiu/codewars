pub mod solution;

type Atom = (String, usize);
type Molecule = Vec<Atom>;

#[cfg(test)]
mod tests {
    use crate::solution::*;
    use crate::{Atom, Molecule};

    fn assert_parse(mol: &str, mut expected: Molecule) {
        let res = parse_molecule(mol);
        assert!(res.is_ok());
        let mut actual = res.unwrap();
        actual.sort();
        expected.sort();
        assert!(actual == expected);
    }

    fn assert_fail(mol: &str) {
        assert!(parse_molecule(mol).is_err());
    }

    #[test]
    fn tests() {
        assert_parse("H", vec![(String::from("H"), 1)]);
        assert_parse("O2", vec![(String::from("O"), 2)]);
        assert_parse("H2O", vec![(String::from("H"), 2), (String::from("O"), 1)]);
        assert_parse(
            "Mg(OH)2",
            vec![
                (String::from("Mg"), 1),
                (String::from("O"), 2),
                (String::from("H"), 2),
            ],
        );
        assert_parse(
            "K4[ON(SO3)2]2",
            vec![
                (String::from("K"), 4),
                (String::from("O"), 14),
                (String::from("N"), 2),
                (String::from("S"), 4),
            ],
        );
        assert_parse(
            "C6H12O6",
            vec![
                (String::from("C"), 6),
                (String::from("H"), 12),
                (String::from("O"), 6),
            ],
        );
        assert_parse(
            "{[Co(NH3)4(OH)2]3Co}(SO4)3",
            vec![
                (String::from("Co"), 4),
                (String::from("H"), 42),
                (String::from("N"), 12),
                (String::from("O"), 18),
                (String::from("S"), 3),
            ],
        );
    }

    #[test]
    fn errors() {
        assert_fail("pie");
        assert_fail("Mg(OH");
        assert_fail("Mg(OH}2");
    }
}
