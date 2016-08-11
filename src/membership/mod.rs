#![warn(missing_docs)]

//! Defines methods to create most used membership functions.
//!
//! #Usage
//! Create triangular function:
//!
//! ```rust
//! use fuzzy_logic::functions::MembershipFactory;
//! let mem = MembershipFactory::triangular(-15.0, -15.0, 22.0);
//! mem(-15.0); // -> 1.0
//! ```
pub mod mf;
pub mod anon_mf;
pub mod gaussian;
pub mod sigmoid;

pub struct MembershipFactory;
impl MembershipFactory {
    pub fn gaussian(a: f32, b: f32, c: f32) -> gaussian::GaussianMF {
        gaussian::GaussianMF::new(a, b, c)
    }

    pub fn sigmoid(a: f32, c: f32) -> sigmoid::SigmoidMF {
        sigmoid::SigmoidMF::new(a, c)
    }

    pub fn anonymous(func: Box<Fn(&f32) -> f32>) -> anon_mf::AnonymousMF {
        anon_mf::AnonymousMF::new(func)
    }
}

#[cfg(test)]
pub mod tests;
