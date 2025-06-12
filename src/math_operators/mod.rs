//! Math Operator Functions
//!
//! This module contains basic mathematical operations that can be applied
//! to price data, including arithmetic operations and statistical functions.

pub mod add;
pub mod sub;
pub mod mult;
pub mod div;
pub mod max;
pub mod min;
pub mod sum;

// Re-export all functions for convenient access
pub use add::{add, add_scalar};
pub use sub::{sub, sub_scalar};
pub use mult::{mult, mult_scalar};
pub use div::{div, div_scalar};
pub use max::{max, maxindex};
pub use min::{min, minindex, minmax, minmaxindex};
pub use sum::{sum, sum_rolling};