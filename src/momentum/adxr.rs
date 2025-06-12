// ADXR - Average Directional Movement Index Rating
use crate::common::TAResult;
use crate::momentum::adx;

/// Calculates the Average Directional Movement Index Rating.
/// 
/// # Arguments
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `period` - Period for calculation
/// 
/// # Returns
/// Vector of ADXR values
pub fn adxr(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
) -> TAResult<Vec<f64>> {
    let adx_vec = adx(high, low, close, period)?;
    let len = adx_vec.len();
    let mut out = vec![f64::NAN; len];
    for i in period..len {
        out[i] = (adx_vec[i] + adx_vec[i - period]) / 2.0;
    }
    Ok(out)
} 