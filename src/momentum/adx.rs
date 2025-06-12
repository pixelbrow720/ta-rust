// ADX - Average Directional Movement Index
use crate::common::TAResult;
use crate::momentum::dx;
use crate::overlap::ema;

/// Calculates the Average Directional Movement Index.
/// 
/// # Arguments
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `period` - Period for calculation
/// 
/// # Returns
/// Vector of ADX values
pub fn adx(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
) -> TAResult<Vec<f64>> {
    let dx_vec = dx(high, low, close, period)?;
    let adx_vec = ema(&dx_vec, period)?;
    Ok(adx_vec)
} 