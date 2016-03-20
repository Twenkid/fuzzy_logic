use set::{Set, UniversalSet};
use ops::{LogicOps, SetOps};
use rules::{RuleSet, Expression};
use std::collections::HashMap;

pub type DefuzzFunc = Fn(&Set) -> f32;

pub struct InferenceOptions {
    pub logic_ops: Box<LogicOps>,
    pub set_ops: Box<SetOps>,
    pub defuzz_func: Box<DefuzzFunc>,
}

pub struct InferenceContext<'a> {
    pub values: &'a HashMap<String, f32>,
    pub sets: &'a mut HashMap<String, Set>,
    pub options: &'a InferenceOptions,
}

pub struct InferenceMachine<E: Expression> {
    pub rules: RuleSet<E>,
    pub universe: UniversalSet,
    pub values: HashMap<String, f32>,
    pub options: InferenceOptions,
}

impl<E: Expression> InferenceMachine<E> {
    // add code here
    pub fn new(rules: RuleSet<E>,
               universe: UniversalSet,
               options: InferenceOptions,
               values: HashMap<String, f32>)
               -> InferenceMachine<E> {
        InferenceMachine {
            rules: rules,
            universe: universe,
            values: values,
            options: options,
        }
    }

    pub fn update(&mut self, values: &HashMap<String, f32>) {
        self.values = values.clone();
    }

    pub fn compute(&mut self) -> f32 {
        let mut context = InferenceContext {
            values: &self.values,
            sets: &mut self.universe.sets,
            options: &self.options,
        };
        let result = self.rules.compute_all(&mut context);
        (*self.options.defuzz_func)(&result)
    }
}
