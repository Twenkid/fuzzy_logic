use super::mf;
use std::collections::HashMap;

pub struct SigmoidMF {
    a: f32,
    c: f32,
    name: &'static str,
}

impl SigmoidMF {
    pub fn new(a: f32, c: f32) -> Self {
        SigmoidMF {
            a: a,
            c: c,
            name: "Sigmoid",
        }
    }
}

impl mf::MembershipFunction for SigmoidMF {
    fn call(&self, x: &f32) -> f32 {
        1.0 / (1.0 + (-1.0 * self.a * (x - self.c)).exp())
    }
}

impl mf::StorableMF for SigmoidMF {
    fn get_params(&self) -> HashMap<String, f32> {
        let mut result = HashMap::new();
        result.insert("a".to_string(), self.a);
        result.insert("c".to_string(), self.c);
        result
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }
}
