use set::Set;
pub type MembershipFunction = Fn(f32) -> f32;
pub type DefuzzFunc = Fn(&Set) -> f32;

pub struct MembershipFactory;

impl MembershipFactory {
    pub fn triangular(a: f32, b: f32, c: f32) -> Box<MembershipFunction> {
        Box::new(move |x: f32| {
            if a <= x && x <= b {
                1.0 - (b - x) / (b - a)
            } else if b <= x && x <= c {
                1.0 - (x - b) / (c - b)
            } else {
                0.0
            }
        })
    }

    // TODO implement this
    pub fn trapezoidal(a: f32, b: f32, c: f32, d: f32) -> Box<MembershipFunction> {
        unimplemented!();
    }

    pub fn sigmoidal(a: f32, c: f32) -> Box<MembershipFunction> {
        Box::new(move |x: f32| 1.0 / (1.0 + (-1.0 * a * (x - c)).exp()))
    }
}

pub struct DefuzzFactory;

impl DefuzzFactory {
    pub fn center_of_mass() -> Box<DefuzzFunc> {
        Box::new(|s: &Set| {
            let sum = s.cache.iter().fold(0.0, |acc, (&k, &v)| acc + v);
            let prod_sum = s.cache.iter().fold(0.0, |acc, (&k, &v)| acc + k.into_inner() * v);
            prod_sum / sum
        })
    }
}

#[cfg(test)]
mod test {
    use std::f32;
    use super::*;

    #[test]
    fn sigmoidal() {
        let steepness = 2.0;
        for i in -100..100 {
            let midpoint = i as f32;
            let f = MembershipFactory::sigmoidal(steepness, midpoint);
            let mut diff = (0.5 - f(midpoint)).abs();
            assert!(diff <= f32::EPSILON);
        }
    }

    #[test]
    // TODO make tests
    fn triangular() {
        let f = MembershipFactory::triangular(1.0, 2.0, 3.0);
    }

    #[test]
    // TODO make tests
    fn trapezoidal() {
        let f = MembershipFactory::trapezoidal(1.0, 2.0, 3.0, 4.0);

    }
}
