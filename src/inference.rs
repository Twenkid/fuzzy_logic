//! This module contains implementation of the fuzzy logic inference mechanism.
//!
//! `InferenceOptions` contains specified implementations of functions and operations.
//! `InferenceContext` also contains `&HashMap<String, f32>` to get values of input variables.
//! Fuzzy logic mechanism is implemented in `InferenceMachine`.
//! User can modify input variables with `update` method and get inference result with `compute` method.

use crate::functions::DefuzzFunc;
use crate::ops::{LogicOps, SetOps};
use crate::rules::RuleSet;
use crate::set::UniversalSet;
use std::collections::HashMap;
use std::fmt::Error; //...
use std::fmt::Formatter;
use std::fmt::Debug;

/// Structure which contains the implementation of fuzzy logic operations.
//#[derive(Debug)]
pub struct InferenceOptions {
    /// Contains fuzzy logical operations.
    pub logic_ops: Box<dyn LogicOps>,
    /// Contains fuzzy set operations.
    pub set_ops: Box<dyn SetOps>,
    /// Contains defuzzification function.
    pub defuzz_func: Box<DefuzzFunc>,
}

/// Structure which contains the evaluation context. Passed to `RuleSet`.
//#[derive(Debug)]
pub struct InferenceContext<'a> {
    /// Reference to the Key-Value container, which contains input variables' values.
    pub values: &'a HashMap<String, f32>,
    /// Reference to the list of available universes.
    pub universes: &'a mut HashMap<String, UniversalSet>,
    /// Reference to the evaluation options.
    pub options: &'a InferenceOptions,
}


/// Structure which contains the implementation of the fuzzy logic inference mechanism.
//#[derive(Debug)]
pub struct InferenceMachine {
    /// List of rules to be evaluated.
    pub rules: RuleSet,
    /// HashMap of all universes. Access by name.
    pub universes: HashMap<String, UniversalSet>,
    /// Input variables' values.
    pub values: HashMap<String, f32>,
    /// Evaluation options.
    pub options: InferenceOptions,
}


impl InferenceMachine {
    /// Constructs a new `InferenceMachine`.
    ///
    /// This function moves all arguments to the structure.
    pub fn new(
        rules: RuleSet,
        universes: HashMap<String, UniversalSet>,
        options: InferenceOptions,
    ) -> Self {
        Self {
            rules,
            universes,
            values: HashMap::new(),
            options,
        }
    }

    /// Updates values in `values`.
    pub fn update(&mut self, values: HashMap<String, f32>) {
        self.values = values;
    }

    /// Computes the result of the fuzzy logic inference.
    ///
    /// Returns activated fuzzy rule's name and defuzzificated result.

    pub fn compute(&mut self) -> (String, f32) {
        println!("inference::compute ...");
        //for i in  self.values {println!("{}",i);}
        //for j in  self.universes {println!("{}",j)};
        //for k in self.options {println!("{}",k)};
        let mut context = InferenceContext {
            values: &self.values,
            universes: &mut self.universes,
            options: &self.options,
        };
        let result = self.rules.compute_all(&mut context);
        dbg!(&result);

        (result.name.clone(), (*self.options.defuzz_func)(&result))
    }
}


impl Debug for InferenceContext<'_>{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>{
		f.debug_struct("context")
         .field("values", &self.values)
         .field("universes", &self.universes)
		 .field("options", &self.options)
         .finish()
	}
}



impl Debug for InferenceOptions<>{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>{
		f.debug_struct("InferenceOptions")
         .field("defuzz_func", &self.defuzz_func)         
         .finish()
	}
}