use super::mf;
use std::collections::HashMap;

pub struct GaussianMF {
    a: f32,
    b: f32,
    c: f32,
    name: &'static str,
}

impl GaussianMF {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        GaussianMF {
            a: a,
            b: b,
            c: c,
            name: "Gaussian",
        }
    }
}

impl mf::MembershipFunction for GaussianMF {
    fn call(&self, x: &f32) -> f32 {
        self.a * (-1.0 * ((x - self.b).powi(2) / (2.0 * self.c.powi(2)))).exp()
    }
}

impl mf::StorableMF for GaussianMF {
    fn get_params(&self) -> HashMap<String, f32> {
        let mut result = HashMap::new();
        result.insert("a".to_string(), self.a);
        result.insert("b".to_string(), self.b);
        result.insert("c".to_string(), self.c);
        result
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }
}
