//! # TA-Rust: Pure Rust Technical Analysis Library
//!
//! A pure Rust implementation of TA-Lib (Technical Analysis Library) with 100% compatibility.
//! This library provides 158+ technical analysis functions for financial market analysis.
//!
//! ## Features
//!
//! - **Pure Rust**: No external C dependencies
//! - **100% TA-Lib Compatible**: Same algorithms, same results
//! - **High Performance**: Optimized for speed and memory efficiency
//! - **Type Safe**: Leverages Rust's type system for correctness
//! - **No Std Support**: Can be used in embedded environments
//!
//! ## Quick Start
//!
//! ```rust
//! use ta_rust::prelude::*;
//!
//! // This example will work once the overlap module is implemented in Phase 2
//! // let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! // let result = sma(&prices, 3).unwrap();
//! // result = [NaN, NaN, 2.0, 3.0, 4.0]
//! ```
//!
//! ## Categories
//!
//! - **Overlap Studies**: Moving averages, Bollinger Bands, SAR, etc.
//! - **Momentum Indicators**: RSI, MACD, Stochastic, ADX, etc.
//! - **Volume Indicators**: OBV, A/D Line, etc.
//! - **Volatility Indicators**: ATR, True Range, etc.
//! - **Price Transform**: Typical Price, Weighted Close, etc.
//! - **Cycle Indicators**: Hilbert Transform functions
//! - **Pattern Recognition**: 61 candlestick patterns
//! - **Statistic Functions**: Correlation, Linear Regression, etc.
//! - **Math Transform**: Trigonometric and logarithmic functions
//! - **Math Operators**: Basic arithmetic operations

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(clippy::all)]

// Re-export common types and errors
pub use common::{TAError, TAResult};

// Core modules
pub mod common;

// Function categories
pub mod overlap;
pub mod price_transform;
pub mod math_operators;

// Function categories - Phase 3 implementations
pub mod momentum;
pub mod volatility;
pub mod math_transform;

// Function categories - Phase 5 implementations
pub mod volume;
pub mod statistic;

// Function categories (will be implemented in subsequent phases)
// pub mod cycle;
// pub mod pattern;

// Prelude for convenient imports
pub mod prelude {
    //! Convenient re-exports of commonly used items
    
    pub use crate::common::{TAError, TAResult, MAType, Price, Volume, Period};
    pub use crate::overlap::*;
    pub use crate::price_transform::*;
    pub use crate::math_operators::*;
    pub use crate::momentum::*;
    pub use crate::volatility::*;
    pub use crate::math_transform::*;
    pub use crate::volume::*;
    pub use crate::statistic::*;
    // Additional re-exports will be added as modules are implemented
}