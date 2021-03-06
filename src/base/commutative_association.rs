use crate::base::expression::Expression;
use std::collections::BinaryHeap;

/**
 *  Associations between several Expressions where elements satisfy:
 *      - associativity
 *      - commutativity
 */
#[derive(Debug, Clone)]
pub struct CommutativeAssociation {
    items: BinaryHeap<Expression>,
}

impl CommutativeAssociation {
    /**
     * Constructor
     */
    pub fn new(items: Vec<Expression>) -> Self {
        Self {
            items: items.iter().cloned().collect(),
        }
    }

    /**
     * Getter for item list
     */
    pub fn items(&self) -> Vec<Expression> {
        self.items.clone().into_sorted_vec()
    }
}

use std::hash::{Hash, Hasher};
impl Hash for CommutativeAssociation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in self.items().iter() {
            item.hash(state);
        }
    }
}

/**
 *  Getter implementation for specific symbols
 */
impl CommutativeAssociation {
    pub fn get(&self, f: &dyn Fn(&Expression) -> bool) -> Vec<Expression> {
        self.items().iter().cloned().filter(f).collect()
    }

    pub fn get_one(&self, f: &dyn Fn(&Expression) -> bool) -> Option<Expression> {
        self.items().iter().cloned().find(f)
    }

    pub fn map(&self, f: &dyn Fn(&Expression) -> Expression) -> Vec<Expression> {
        self.items().iter().map(f).collect()
    }

    /**
     * Get real part of commucative association
     */
    pub fn get_reals(&self) -> Vec<Expression> {
        self.items()
            .iter()
            .cloned()
            .filter(|factor| match factor {
                Expression::Real(_) => true,
                _ => false,
            })
            .collect()
    }

    /**
     * Get integer numerators
     *  - only one integer is expected if the addition of multiplication is simplified at construction
     */
    pub fn get_rational_numerators(&self) -> Vec<Expression> {
        self.items()
            .iter()
            .cloned()
            .filter(|factor| match factor {
                Expression::Integer(_) => true,
                _ => false,
            })
            .collect()
    }

    /**
     * Get integer denominators
     *  - addition may have serveral fractional parts
     */
    pub fn get_rational_denominators(&self) -> Vec<Expression> {
        self.items()
            .iter()
            .cloned()
            .filter(|factor| match factor {
                Expression::Power(power) => match (power.argument(), power.modifier()) {
                    (Expression::Integer(_), Expression::Integer(_)) => true,
                    _ => false,
                },
                _ => false,
            })
            .map(|factor| match factor {
                Expression::Power(power) => return power.argument(),
                _ => panic!(),
            })
            .collect()
    }

    /**
     * Get integer divided by another integer
     */
    pub fn get_rationals(&self) -> Vec<Expression> {
        self.items()
            .iter()
            .cloned()
            .filter(|factor| match factor {
                Expression::Integer(_) => true,
                Expression::Power(power) => match (power.argument(), power.modifier()) {
                    (Expression::Integer(_), Expression::Integer(_)) => true,
                    _ => false,
                },
                Expression::Multiplication(factors) => {
                    let possible_numerator: Vec<Expression> = factors.get_rational_numerators();
                    let possible_denominator: Vec<Expression> = factors.get_rational_denominators();

                    return factors.items().len()
                        == possible_numerator.len() + possible_denominator.len();
                }
                _ => false,
            })
            .collect()
    }

    /**
     * Get all non rational, non integer, non inverted integer
     */
    pub fn get_non_rationals(&self) -> Vec<Expression> {
        self.items()
            .iter()
            .cloned()
            .filter(|factor| match factor {
                Expression::Integer(_) => false,
                Expression::Power(power) => match (power.argument(), power.modifier()) {
                    (Expression::Integer(_), Expression::Integer(_)) => false,
                    _ => true,
                },
                Expression::Multiplication(factors) => {
                    let possible_numerator: Vec<Expression> = factors.get_rational_numerators();
                    let possible_denominator: Vec<Expression> = factors.get_rational_denominators();

                    return factors.items().len()
                        != possible_numerator.len() + possible_denominator.len();
                }
                _ => true,
            })
            .collect()
    }

    /**
     * Get integer denominators
     *  - addition may have serveral fractional parts
     */
    pub fn get_symbolics(&self) -> Vec<Expression> {
        self.items()
            .iter()
            .cloned()
            .filter(|factor| match factor {
                Expression::Integer(_) => false,
                Expression::Real(_) => false,
                Expression::Power(power) => match (power.argument(), power.modifier()) {
                    (Expression::Integer(_), Expression::Integer(_)) => false,
                    _ => true,
                },
                _ => true,
            })
            .collect()
    }
} /* end - commutative association */

#[cfg(test)]
mod getters {
    use super::*;
    use crate::base::symbol::Symbol;

    #[test]
    fn reals() {
        let items = CommutativeAssociation::new(vec![
            Symbol::integer(1).expr(),
            Symbol::integer(2).expr(),
            Symbol::integer(3).expr(),
            Symbol::real(1.0).expr(),
            Symbol::real(2.0).expr(),
            Symbol::real(3.0).expr(),
        ]);

        assert_eq!(items.get_reals().len(), 3);
        assert!(items.get_reals().contains(&Symbol::real(1.0).expr()));
        assert!(items.get_reals().contains(&Symbol::real(2.0).expr()));
        assert!(items.get_reals().contains(&Symbol::real(3.0).expr()));
    }

    #[test]
    fn numerators() {
        let items = CommutativeAssociation::new(vec![
            Symbol::integer(1).expr(),
            Symbol::integer(2).expr(),
            Symbol::integer(3).expr(),
            Symbol::real(1.0).expr(),
            Symbol::real(2.0).expr(),
            Symbol::real(3.0).expr(),
        ]);

        assert_eq!(items.get_rational_numerators().len(), 3);
        assert!(items
            .get_rational_numerators()
            .contains(&Symbol::integer(1).expr()));
        assert!(items
            .get_rational_numerators()
            .contains(&Symbol::integer(2).expr()));
        assert!(items
            .get_rational_numerators()
            .contains(&Symbol::integer(3).expr()));
    }

    #[test]
    fn denominators() {
        let items = CommutativeAssociation::new(vec![
            Symbol::integer(1).expr(),
            Symbol::integer(2).expr(),
            Symbol::integer(3).expr(),
            Expression::power(Symbol::integer(5).expr(), Symbol::integer(-1).expr()),
            Expression::power(Symbol::integer(2).expr(), Symbol::integer(-1).expr()),
            Expression::power(Symbol::integer(3).expr(), Symbol::integer(-1).expr()),
        ]);

        assert_eq!(items.get_rational_denominators().len(), 3);
        assert!(items
            .get_rational_denominators()
            .contains(&Symbol::integer(2).expr()));
        assert!(items
            .get_rational_denominators()
            .contains(&Symbol::integer(3).expr()));
        assert!(items
            .get_rational_denominators()
            .contains(&Symbol::integer(5).expr()));
    }

    #[test]
    fn rationals() {
        let items = CommutativeAssociation::new(vec![
            Symbol::integer(1).expr(),
            Symbol::variable("a").expr(),
            Symbol::integer(1).expr() / Symbol::integer(7).expr(),
            Symbol::integer(2).expr() / Symbol::integer(7).expr(),
            Symbol::integer(3).expr() / Symbol::integer(7).expr(),
            (Symbol::integer(3).expr() / Symbol::integer(7).expr()) / Symbol::integer(7).expr(),
        ]);

        assert_eq!(items.get_rationals().len(), 5);
        assert!(items.get_rationals().contains(&(Symbol::integer(1).expr())));
        assert!(items
            .get_rationals()
            .contains(&(Symbol::integer(1).expr() / Symbol::integer(7).expr())));
        assert!(items
            .get_rationals()
            .contains(&(Symbol::integer(2).expr() / Symbol::integer(7).expr())));
        assert!(items
            .get_rationals()
            .contains(&(Symbol::integer(3).expr() / Symbol::integer(7).expr())));
        assert!(items.get_rationals().contains(
            &((Symbol::integer(3).expr() / Symbol::integer(7).expr()) / Symbol::integer(7).expr())
        ));

        assert_eq!(items.get_non_rationals().len(), 1);
        assert!(items
            .get_non_rationals()
            .contains(&Symbol::variable("a").expr()));
    }

    #[test]
    fn non_rationals() {
        let items = CommutativeAssociation::new(vec![
            Symbol::integer(1).expr(),
            Symbol::variable("a").expr(),
            Symbol::integer(5).expr() / Symbol::integer(7).expr(),
            Symbol::variable("a").expr() / Symbol::integer(7).expr(),
            Symbol::integer(3).expr() / Symbol::integer(7).expr(),
            (Symbol::integer(3).expr() / Symbol::integer(7).expr()) / Symbol::integer(7).expr(),
        ]);

        assert_eq!(items.get_non_rationals().len(), 2);
        assert!(items
            .get_non_rationals()
            .contains(&Symbol::variable("a").expr()));
        assert!(items
            .get_non_rationals()
            .contains(&(Symbol::variable("a").expr() / Symbol::integer(7).expr())));
    }

    #[test]
    fn symbolics() {
        let items = CommutativeAssociation::new(vec![
            Symbol::integer(1).expr(),
            Symbol::integer(2).expr(),
            Symbol::integer(3).expr(),
            Symbol::variable("a").expr(),
            Symbol::variable("b").expr(),
            Symbol::variable("c").expr(),
        ]);

        assert_eq!(items.get_symbolics().len(), 3);
        assert!(items
            .get_symbolics()
            .contains(&Symbol::variable("a").expr()));
        assert!(items
            .get_symbolics()
            .contains(&Symbol::variable("b").expr()));
        assert!(items
            .get_symbolics()
            .contains(&Symbol::variable("c").expr()));
    }

    #[test]
    fn get() {
        let items = CommutativeAssociation::new(vec![
            Symbol::integer(1).expr(),
            Symbol::integer(2).expr(),
            Symbol::integer(3).expr(),
            Symbol::variable("a").expr(),
            Symbol::variable("b").expr(),
            Symbol::variable("c").expr(),
        ]);

        let filtered = items.get(&|item| match item {
            Expression::Integer(_) => true,
            _ => false,
        });

        assert_eq!(filtered.len(), 3);
        assert!(filtered.contains(&Symbol::integer(1).expr()));
        assert!(filtered.contains(&Symbol::integer(2).expr()));
        assert!(filtered.contains(&Symbol::integer(3).expr()));
    }

    #[test]
    fn map() {
        let items = CommutativeAssociation::new(vec![
            Symbol::integer(1).expr(),
            Symbol::integer(2).expr(),
            Symbol::integer(4).expr(),
        ]);

        let mapped = items.map(&|item| item * Symbol::integer(3).expr());

        assert_eq!(mapped.len(), 3);
        assert!(mapped.contains(&Symbol::integer(3).expr()));
        assert!(mapped.contains(&Symbol::integer(6).expr()));
        assert!(mapped.contains(&Symbol::integer(12).expr()));
    }
}
