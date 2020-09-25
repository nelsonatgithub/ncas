#[cfg(test)]
mod as_expression {
    use crate::{
        arithmetics::primitives_interface::AsExpression,
        base::symbols::{number::Number, variable::Variable},
    };

    #[test]
    fn sample_1() {
        assert_eq!(1.as_expression(), Number::new(1.0));
        assert_eq!((-1).as_expression(), Number::new(-1.0));
        assert_eq!(-1.as_expression(), Number::new(-1.0) * Number::new(1.0));
        assert_eq!(1.01.as_expression(), Number::new(1.01));
        assert_eq!("a".as_expression(), Variable::new(String::from("a")));
        assert_eq!(
            String::from("a").as_expression(),
            Variable::new(String::from("a"))
        );
    }
}

#[cfg(test)]
mod negation {
    use crate::{
        base::symbols::{constant::Constant, number::Number},
        manipulation::numeric_evaluation::NumericEvaluable,
    };

    #[test]
    fn negate_number_sign() {
        assert_eq!(-Number::new(1.0), Number::new(-1.0) * Number::new(1.0));
    }

    #[test]
    fn negate_number_sign_numerical_evaluation() {
        assert_eq!((-Number::new(1.0)).into_num(), Ok(-1.0));
    }

    #[test]
    fn negate_constant() {
        assert_eq!(
            -Constant::new(String::from("one"), 1.0),
            Number::new(-1.0) * Constant::new(String::from("one"), 1.0)
        );
    }

    #[test]
    fn negate_constant_numerical_evaluation() {
        assert_eq!(
            (-Constant::new(String::from("one"), 1.0)).into_num(),
            Ok(-1.0)
        );
    }
}

#[cfg(test)]
mod arithmetics {
    use crate::base::symbols::{number::Number, variable::Variable};

    #[test]
    fn addition() {
        /* numeric */
        assert_eq!(1 + Number::new(1.0), Number::new(1.0) + Number::new(1.0));
        assert_eq!(1.0 + Number::new(1.0), Number::new(1.0) + Number::new(1.0));
        assert_eq!(Number::new(1.0) + 1, Number::new(1.0) + Number::new(1.0));
        assert_eq!(Number::new(1.0) + 1.0, Number::new(1.0) + Number::new(1.0));

        /* algebraic */
        assert_eq!(
            "a" + Number::new(1.0),
            Variable::new(String::from("a")) + Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) + "a",
            Number::new(1.0) + Variable::new(String::from("a"))
        );
        assert_eq!(
            String::from("a") + Number::new(1.0),
            Variable::new(String::from("a")) + Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) + String::from("a"),
            Number::new(1.0) + Variable::new(String::from("a"))
        );
    }

    #[test]
    fn subtraction() {
        /* numeric */
        assert_eq!(1 - Number::new(1.0), Number::new(1.0) - Number::new(1.0));
        assert_eq!(1.0 - Number::new(1.0), Number::new(1.0) - Number::new(1.0));
        assert_eq!(Number::new(1.0) - 1, Number::new(1.0) - Number::new(1.0));
        assert_eq!(Number::new(1.0) - 1.0, Number::new(1.0) - Number::new(1.0));

        /* algebraic */
        assert_eq!(
            "a" - Number::new(1.0),
            Variable::new(String::from("a")) - Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) - "a",
            Number::new(1.0) - Variable::new(String::from("a"))
        );
        assert_eq!(
            String::from("a") - Number::new(1.0),
            Variable::new(String::from("a")) - Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) - String::from("a"),
            Number::new(1.0) - Variable::new(String::from("a"))
        );
    }

    #[test]
    fn multiplication() {
        /* numeric */
        assert_eq!(1 * Number::new(1.0), Number::new(1.0) * Number::new(1.0));
        assert_eq!(1.0 * Number::new(1.0), Number::new(1.0) * Number::new(1.0));
        assert_eq!(Number::new(1.0) * 1, Number::new(1.0) * Number::new(1.0));
        assert_eq!(Number::new(1.0) * 1.0, Number::new(1.0) * Number::new(1.0));

        /* algebraic */
        assert_eq!(
            "a" * Number::new(1.0),
            Variable::new(String::from("a")) * Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) * "a",
            Number::new(1.0) * Variable::new(String::from("a"))
        );
        assert_eq!(
            String::from("a") * Number::new(1.0),
            Variable::new(String::from("a")) * Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) * String::from("a"),
            Number::new(1.0) * Variable::new(String::from("a"))
        );
    }

    #[test]
    fn division() {
        /* numeric */
        assert_eq!(1 / Number::new(1.0), Number::new(1.0) / Number::new(1.0));
        assert_eq!(1.0 / Number::new(1.0), Number::new(1.0) / Number::new(1.0));
        assert_eq!(Number::new(1.0) / 1, Number::new(1.0) / Number::new(1.0));
        assert_eq!(Number::new(1.0) / 1.0, Number::new(1.0) / Number::new(1.0));

        /* algebraic */
        assert_eq!(
            "a" / Number::new(1.0),
            Variable::new(String::from("a")) / Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) / "a",
            Number::new(1.0) / Variable::new(String::from("a"))
        );
        assert_eq!(
            String::from("a") / Number::new(1.0),
            Variable::new(String::from("a")) / Number::new(1.0)
        );
        assert_eq!(
            Number::new(1.0) / String::from("a"),
            Number::new(1.0) / Variable::new(String::from("a"))
        );
    }
}
