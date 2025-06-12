//! Overlap Studies - Moving averages and trend-following indicators
//!
//! This module contains functions that typically overlay the price chart,
//! including various types of moving averages and trend indicators.

pub mod sma;
pub mod ema;
pub mod wma;
pub mod dema;
pub mod tema;
pub mod trima;
pub mod ma;
pub mod midpoint;
pub mod midprice;

// Re-export all functions for convenient access
pub use sma::{sma, sma_rolling};
pub use ema::{ema, ema_from_first, ema_custom};
pub use wma::{wma, wma_custom, wma_rolling};
pub use dema::{dema, dema_direct};
pub use tema::{tema, tema_direct};
pub use trima::{trima, trima_direct, trima_custom_peak};
pub use ma::{ma, ma_multiple, ma_auto};
pub use midpoint::{midpoint, midpoint_rolling, midpoint_custom};
pub use midprice::{midprice, midprice_ohlc, midprice_percentile, midprice_adaptive};