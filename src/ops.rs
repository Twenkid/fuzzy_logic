//! This module contains implementation of the fuzzy logic operations.
//!
//! Fuzzy set operations and fuzzy logic operations are defined here.
//!
//! User can implement his own operations by implementing `LogicOps` or `SetOps` traits.
use set::Set;
use membership::MembershipFactory;

/// Abstraction over set operations. Doesn't contain default implementation.
pub trait SetOps {
    /// Union of fuzzy sets.
    fn union(&self, left: &mut Set, right: &mut Set) -> Set;
    /// Intersection of fuzzy sets.
    fn intersect(& self, left: & mut Set, right: & mut Set) -> Set;
}

/// Implementation of commonly used minimax set operations.
pub struct MinMaxOps;

impl SetOps for MinMaxOps {
    /// Union of fuzzy sets.
    ///
    /// Values with highest memberships are copied to the result set.
    fn union(&self, left: &mut Set, right: &mut Set) -> Set {
        let left_mf = left.get_membership();
        let right_mf = right.get_membership();
        let new_mf = Box::new(move |x: &f32| left_mf.call(x).max(right_mf.call(x)));
        let anon_mf = Box::new(MembershipFactory::anonymous(new_mf));
        Set::new(&format!("{} UNION {}", left.get_name(), right.get_name()),
                 anon_mf)
    }

    /// Intersection of fuzzy sets.
    ///
    /// Values with lowest memberships are copied to the result set.
    fn intersect(& self, left: &mut Set, right: &mut Set) -> Set {
        let left_mf = left.get_membership();
        let right_mf = right.get_membership();
        let new_mf = Box::new(move |x: &f32| left_mf.call(x).min(right_mf.call(x)));
        let anon_mf = Box::new(MembershipFactory::anonymous(new_mf));
        Set::new(&format!("{} UNION {}", left.get_name(), right.get_name()),
                 anon_mf)
    }
}

/// Abstraction over fuzzy logic operations. Doesn't contain default implementation.
pub trait LogicOps {
    /// Fuzzy logic AND operation.
    fn and(&self, left: f32, right: f32) -> f32;
    /// Fuzzy logic OR operation.
    fn or(&self, left: f32, right: f32) -> f32;
    /// Fuzzy logic NOT operation.
    fn not(&self, value: f32) -> f32;
}

/// Implementation of commonly used Zadeh fuzzy logic operations.
pub struct ZadehOps;

impl LogicOps for ZadehOps {
    /// Fuzzy logic AND operation.
    ///
    /// Returns minimum of arguments.
    ///
    /// # Usage
    /// The classical AND operator's truth table:
    ///
    /// ```rust
    /// use fuzzy_logic::ops::{LogicOps, ZadehOps};
    /// let ops = ZadehOps{};
    /// ops.and(0.0, 0.0); //-> 0.0
    /// ops.and(0.0, 1.0); //-> 0.0
    /// ops.and(1.0, 0.0); //-> 0.0
    /// ops.and(1.0, 1.0); //-> 1.0
    /// ```
    fn and(&self, left: f32, right: f32) -> f32 {
        left.min(right)
    }

    /// Fuzzy logic AND operation.
    ///
    /// Returns maximum of arguments.
    ///
    /// # Usage
    /// The classical OR operator's truth table:
    ///
    /// ```rust
    /// use fuzzy_logic::ops::{LogicOps, ZadehOps};
    /// let ops = ZadehOps{};
    /// ops.or(0.0, 0.0); //-> 0.0
    /// ops.or(0.0, 1.0); //-> 1.0
    /// ops.or(1.0, 0.0); //-> 1.0
    /// ops.or(1.0, 1.0); //-> 1.0
    /// ```
    fn or(&self, left: f32, right: f32) -> f32 {
        left.max(right)
    }

    /// Fuzzy logic AND operation.
    ///
    /// Returns inversed logical value.
    /// # Usage
    /// The classical NOT operator's truth table:
    ///
    /// ```rust
    /// use fuzzy_logic::ops::{LogicOps, ZadehOps};
    /// let ops = ZadehOps{};
    /// ops.not(0.0); //-> 1.0
    /// ops.not(1.0); //-> 0.0
    /// ```
    fn not(&self, value: f32) -> f32 {
        1.0 - value
    }
}
