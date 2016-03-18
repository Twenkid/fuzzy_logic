use set::Set;
use ops::{LogicOps, SetOps};
use std::collections::HashMap;

pub type DefuzzFunc = Fn(&Set) -> f32;

pub struct InferenceOptions {
    pub logic_ops: Box<LogicOps>,
    pub set_ops: Box<SetOps>,
    pub defuzz_func: Box<DefuzzFunc>,
}

pub struct InferenceContext<'a> {
    pub values: HashMap<String, f32>,
    pub sets: &'a mut HashMap<String, Set>,
    pub options: InferenceOptions,
}
