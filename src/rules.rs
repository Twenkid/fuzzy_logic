extern crate ordered_float;
use self::ordered_float::OrderedFloat;

use inference::InferenceContext;
use set::Set;

use std::fmt;
use std::collections::HashMap;

pub trait Expression {
    fn eval(&self, context: &mut InferenceContext) -> f32;
    fn to_string(&self) -> String;
}

// TODO implement `is`
pub struct Is {
    variable: String,
    set: String,
}

impl Is {
    pub fn new(variable: String, set: String) -> Is {
        Is {
            variable: variable,
            set: set,
        }
    }
}

impl Expression for Is {
    fn eval(&self, context: &mut InferenceContext) -> f32 {
        let value = context.values[&self.variable];
        let mut set = context.sets.get_mut(&self.set).unwrap();
        set.check(value)
    }
    fn to_string(&self) -> String {
        format!("(is {} {})", self.variable, self.set)
    }
}

pub struct And<L, R>
    where L: Expression,
          R: Expression
{
    left: L,
    right: R,
}

impl<L: Expression, R: Expression> And<L, R> {
    pub fn new(left: L, right: R) -> And<L, R> {
        And {
            left: left,
            right: right,
        }
    }
}

impl<L: Expression, R: Expression> Expression for And<L, R> {
    fn eval(&self, context: &mut InferenceContext) -> f32 {
        let left_result = self.left.eval(context);
        let right_result = self.right.eval(context);
        (*context.options.logic_ops).and(left_result, right_result)
    }
    fn to_string(&self) -> String {
        format!("(and {} {})", self.left.to_string(), self.right.to_string())
    }
}

pub struct Or<L, R>
    where L: Expression,
          R: Expression
{
    left: L,
    right: R,
}

impl<L: Expression, R: Expression> Or<L, R> {
    pub fn new(left: L, right: R) -> Or<L, R> {
        Or {
            left: left,
            right: right,
        }
    }
}

impl<L: Expression, R: Expression> Expression for Or<L, R> {
    fn eval(&self, context: &mut InferenceContext) -> f32 {
        let left_result = self.left.eval(context);
        let right_result = self.right.eval(context);
        (*context.options.logic_ops).or(left_result, right_result)
    }
    fn to_string(&self) -> String {
        format!("(or {} {})", self.left.to_string(), self.right.to_string())
    }
}

pub struct Not {
    expression: Box<Expression>,
}

impl Not {
    fn new(expression: Box<Expression>) -> Not {
        Not { expression: expression }
    }
}

impl Expression for Not {
    fn eval(&self, context: &mut InferenceContext) -> f32 {
        let value = (*self.expression).eval(context);
        (*context.options.logic_ops).not(value)
    }
    fn to_string(&self) -> String {
        format!("(not {})", (*self.expression).to_string())
    }
}

pub struct Rule<E: Expression> {
    name: String,
    condition: E,
    result: String,
}

impl<E: Expression> Rule<E> {
    pub fn new(name: String, condition: E, result: String) -> Rule<E> {
        Rule {
            name: name,
            condition: condition,
            result: result,
        }
    }
    pub fn compute(&self, context: &mut InferenceContext) -> Set {
        let expression_result = self.condition.eval(context);
        let result_set = context.sets.get_mut(&self.result).unwrap();
        let result_values = result_set.cache
                                      .iter()
                                      .filter_map(|(&key, &value)| {
                                          if value <= expression_result {
                                              Some((key, value))
                                          } else {
                                              None
                                          }
                                      })
                                      .collect::<HashMap<_, f32>>();
        Set::new_with_domain(format!("{} rule result", self.name), result_values)
    }
}

impl<E: Expression> fmt::Display for Rule<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "(Rule:\"{}\" if:{} then:{})",
               self.name,
               self.condition.to_string(),
               self.result)
    }
}

pub struct RuleSet<E: Expression> {
    rules: Vec<Rule<E>>,
}

impl<E: Expression> RuleSet<E> {
    pub fn new(rules: Vec<Rule<E>>) -> RuleSet<E> {
        RuleSet { rules: rules }
    }
    pub fn compute_all(&self, context: &mut InferenceContext) -> Set {
        let mut result_set = Set::new_empty();
        for rule in &self.rules {
            let mut result = rule.compute(context);
            result_set = (*context.options.set_ops).union(&mut result_set, &mut result);
        }
        result_set
    }
}

impl<E: Expression> fmt::Display for RuleSet<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for rule in &self.rules {
            s = s + &format!("\t{}\n", rule);
        }
        write!(f, "(RuleSet\n{})", s)
    }
}
