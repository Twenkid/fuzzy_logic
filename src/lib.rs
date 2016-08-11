#![warn(missing_docs)]
//! The implementation of the fuzzy logic inference.
//!
//! Provides structs, which defines fuzzy sets and rules.
//! Rules are constructed with logical operations
//! but implementation of operations is chosen by user.
//!
//! User is available to implement his own functions and operations.

// pub mod functions;
pub mod membership;
pub mod set;
pub mod ops;
pub mod rules;
pub mod inference;
pub mod defuzz;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {}
}
