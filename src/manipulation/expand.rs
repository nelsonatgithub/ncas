// =================================== //
//      Recursion on Expression        //
// =================================== //
use crate::{
    base::expression::Expression,
    manipulation::expansion_rules::{
        multiplicative_distributive::MultiplicativeDistributive,
        power_distributive_addition::PowerDistributiveAddition, rule::Rule,
    },
};

impl Expression {
    pub fn expand(self) -> Expression {
        /* recursive expansion */
        match &self {
            Expression::Multiplication(factors) => {
                return MultiplicativeDistributive::apply(&Expression::multiplication(
                    factors.map(&|factor| factor.clone().expand()),
                ));
            }
            Expression::Addition(addends) => {
                return Expression::addition(addends.map(&|addend| addend.clone().expand()));
            }

            Expression::Power(power) => {
                return PowerDistributiveAddition::apply(&Expression::power(
                    power.argument().expand(),
                    power.modifier().expand(),
                ));
            }
            Expression::Logarithm(log) => {
                return Expression::logarithm(log.argument().expand(), log.modifier().expand())
            }

            _ => return self,
        }
    }
}
