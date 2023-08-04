#[cfg(test)]
mod test {
    use crate::parser::{parse, Optimize};

    fn setup_single(expression: &str) -> String {
        parse(expression).unwrap().optimize_node().to_string()
    }

    fn setup_multi(expression: &str) -> String {
        parse(expression).unwrap().optimize_expression().to_string()
    }

    #[test]
    fn can_optimize_double_unary() {
        assert_eq!("25", setup_single("-(-25)"));
    }

    #[test]
    fn can_optimize_double_unary_in_expression() {
        assert_eq!("((3*5)+25)", setup_single("3*5+(-(-25))"));
    }

    #[test]
    fn can_optimize_zero_addition() {
        assert_eq!("645", setup_single("0+645"));
        assert_eq!("645", setup_single("645+0"));
    }

    #[test]
    fn can_optimize_zero_addition_in_expression() {
        assert_eq!("(55*645)", setup_single("55*(0+645)"));
        assert_eq!("(24*645)", setup_single("24*645+0"));
    }

    #[test]
    fn can_optimize_zero_subtraction() {
        assert_eq!("645", setup_single("0-645"));
        assert_eq!("645", setup_single("645-0"));
    }

    #[test]
    fn can_optimize_zero_subtraction_in_expression() {
        assert_eq!("(55*645)", setup_single("55*(0-645)"));
        assert_eq!("(24*645)", setup_single("24*645-0"));
    }

    #[test]
    fn can_optimize_double_subtraction() {
        assert_eq!("0", setup_single("112-112"));
    }

    #[test]
    fn can_optimize_double_subtraction_in_expression() {
        assert_eq!("0", setup_single("(32894/132)-(32894/132)"));
    }

    #[test]
    fn can_optimize_one_multiplication() {
        assert_eq!("645", setup_single("1*645"));
        assert_eq!("645", setup_single("645*1"));
    }

    #[test]
    fn can_optimize_one_multiplication_in_expression() {
        assert_eq!("(55*645)", setup_single("55*1*645"));
        assert_eq!("(24*645)", setup_single("24*645*1"));
    }

    #[test]
    fn can_optimize_one_division() {
        assert_eq!("645", setup_single("645/1"));
    }

    #[test]
    fn can_optimize_one_division_in_expression() {
        assert_eq!("(24*645)", setup_single("24*645/1"));
    }

    #[test]
    fn can_optimize_double_division() {
        assert_eq!("1", setup_single("112/112"));
    }

    #[test]
    fn can_optimize_double_division_in_expression() {
        assert_eq!("1", setup_single("(32894-132)/(32894-132)"));
    }

    #[test]
    fn can_optimize_double_powers() {
        assert_eq!("(3^(5+10))", setup_single("3^5*3^10"));
    }

    #[test]
    fn can_optimize_double_powers_in_expression() {
        assert_eq!(
            "(3^((3213*2)+(421*23)))",
            setup_single("3^(3213*2)*3^(421*23)")
        );
    }

    #[test]
    fn can_optimize_power_of_one() {
        assert_eq!("3", setup_single("3^1"));
    }

    #[test]
    fn can_optimize_power_of_one_in_expression() {
        assert_eq!("(3213*2)", setup_single("(3213*2)^1"));
    }

    #[test]
    fn can_optimize_power_of_negative_one() {
        assert_eq!("(1/(3^1))", setup_single("3^(-1)"));
    }

    #[test]
    fn can_optimize_power_of_negative_one_in_expression() {
        assert_eq!("(1/((3213*2)^1))", setup_single("(3213*2)^(-1)"));
    }

    #[test]
    fn can_optimize_multiple_layers() {
        assert_eq!("(1/(3213*2))", setup_multi("(3213*2)^(-1)"));
        assert_eq!("(1/0)", setup_multi("(53*88*(52-52))^(-(125/125))"));
    }

    #[test]
    fn can_optimize_monomial_plus() {
        assert_eq!("8X^(8)", setup_single("2X^8+6X^8"));
        assert_eq!("2X^(1)", setup_single("X+X"));
    }

    #[test]
    fn can_optimize_monomial_multiply() {
        assert_eq!("12X^(10)", setup_single("2X^8*6X^2"));
        assert_eq!("1X^(2)", setup_single("X*X"));
    }
}
