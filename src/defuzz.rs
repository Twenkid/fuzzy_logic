//! This module defines types and structures for fuzzy logic functions.
//!
//! Module contains implementation of membership functions and defuzzification functions.
//! Also contains factory methods to create most used functions.

use set::Set;
use std::collections::HashMap;

/// Used to defuzzificate the fuzzy logic inference result.
/// All defuzzification functions must be this type.
pub type DefuzzFunc = Fn(&Set) -> f32;

/// Defines methods to create most used defuzzification functions.
///
/// #Usage
/// Create function which calculates center of mass:
///
/// ```rust
/// use fuzzy_logic::functions::{DefuzzFactory, MembershipFactory};
/// use fuzzy_logic::set::Set;
///
/// let mem = MembershipFactory::triangular(-15.0, -15.0, 22.0);
/// let df = DefuzzFactory::center_of_mass();
/// let set = Set::new_with_mem("Test".to_string(), mem);
/// df(&set);
/// ```
pub struct DefuzzFactory;

impl DefuzzFactory {
    /// Creates function which calculates center of mass.
    pub fn center_of_mass() -> Box<DefuzzFunc> {
        unimplemented!()
    }
}
