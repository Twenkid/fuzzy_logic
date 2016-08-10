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
pub mod gaussian;
pub mod sigmoid;

#[cfg(test)]
pub mod tests;
