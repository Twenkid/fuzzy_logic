use super::mf::MembershipFunction;

pub struct AnonymousMF {
    func: Box<Fn(&f32) -> f32>,
}

impl AnonymousMF {
    pub fn new(func: Box<Fn(&f32) -> f32>) -> Self {
        AnonymousMF { func: func }
    }
}

impl MembershipFunction for AnonymousMF {
    fn call(&self, x: &f32) -> f32 {
        (*self.func)(x)
    }
}
