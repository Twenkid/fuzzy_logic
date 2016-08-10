use std::collections::HashMap;

pub trait MembershipFunction {
    fn call(&self, x: &f32) -> f32;
}

pub trait StorableMF: MembershipFunction {
    fn get_params(&self) -> HashMap<String, f32>;
    fn get_name(&self) -> String;
}
