//! Statistic Functions
//!
//! Statistical analysis functions for financial data including correlation,
//! linear regression, standard deviation, and other statistical measures.

pub mod beta;
pub mod correl;
pub mod linearreg;
pub mod linearreg_angle;
pub mod linearreg_intercept;
pub mod linearreg_slope;
pub mod stddev;
pub mod tsf;
pub mod var;

pub use beta::*;
pub use correl::*;
pub use linearreg::*;
pub use linearreg_angle::*;
pub use linearreg_intercept::*;
pub use linearreg_slope::*;
pub use stddev::*;
pub use tsf::*;
pub use var::*;