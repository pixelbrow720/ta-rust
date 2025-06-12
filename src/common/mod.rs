//! Common types, utilities, and error handling for TA-Rust
//!
//! This module provides the foundational components used throughout the library:
//! - Type definitions for prices, volumes, and periods
//! - Error handling types
//! - Moving average type enumeration
//! - Utility functions for validation and calculations
//! - Constants used in pattern recognition

pub mod types;
pub mod errors;
pub mod utils;
pub mod constants;

#[cfg(test)]
pub mod test_helpers;

// Re-export commonly used items
pub use types::{Price, Volume, Period, MAType, OHLC, OHLCV};
pub use errors::{TAError, TAResult};
pub use utils::*;
pub use constants::*;

#[cfg(test)]
pub use test_helpers::*;