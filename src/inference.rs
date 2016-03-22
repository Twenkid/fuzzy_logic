use set::{Set, UniversalSet};
use ops::{LogicOps, SetOps};
use rules::{RuleSet, Expression};
use functions::DefuzzFunc;
use std::collections::HashMap;


pub struct InferenceOptions {
    pub logic_ops: Box<LogicOps>,
    pub set_ops: Box<SetOps>,
    pub defuzz_func: Box<DefuzzFunc>,
}

pub struct InferenceContext<'a> {
    pub values: &'a HashMap<String, f32>,
    pub universes: &'a mut HashMap<String, UniversalSet>,
    pub options: &'a InferenceOptions,
}

pub struct InferenceMachine {
    pub rules: RuleSet,
    pub universes: HashMap<String, UniversalSet>,
    pub values: HashMap<String, f32>,
    pub options: InferenceOptions,
}

impl InferenceMachine {
    pub fn new(rules: RuleSet,
               universes: HashMap<String, UniversalSet>,
               options: InferenceOptions)
               -> InferenceMachine {
        InferenceMachine {
            rules: rules,
            universes: universes,
            values: HashMap::new(),
            options: options,
        }
    }

    pub fn update(&mut self, values: &HashMap<String, f32>) {
        self.values = values.clone();
    }

    pub fn compute(&mut self) -> (String, f32) {
        let mut context = InferenceContext {
            values: &self.values,
            universes: &mut self.universes,
            options: &self.options,
        };
        let result = self.rules.compute_all(&mut context);
        (result.name.clone(), (*self.options.defuzz_func)(&result))
    }
}
