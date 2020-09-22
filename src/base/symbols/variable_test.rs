#[cfg(test)]
mod base {
    use crate::base::{expression::Expression, symbols::variable::Variable};

    #[test]
    fn displays_label() {
        let y: Expression = Variable::new(String::from("y"));
        assert_eq!(format!("{}", y), String::from("y"));
    }
}

#[cfg(test)]
mod evaluable {
    use crate::{
        base::{expression::Expression, symbols::variable::Variable},
        manipulation::evaluate::Evaluable,
    };

    #[test]
    fn not_evaluable() {
        let mut x: Expression = Variable::new(String::from("x"));
        assert!(x.evaluate().is_err());
    }
}
