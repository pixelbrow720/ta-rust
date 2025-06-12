// AROONOSC - Aroon Oscillator
use crate::common::TAResult;
use crate::momentum::aroon;

/// Calculates the Aroon Oscillator.
/// 
/// # Arguments
/// * `high` - High prices
/// * `low` - Low prices
/// * `period` - Period for calculation
/// 
/// # Returns
/// Vector of Aroon Oscillator values
pub fn aroonosc(
    high: &[f64],
    low: &[f64],
    period: usize,
) -> TAResult<Vec<f64>> {
    let (up, down) = aroon(high, low, period)?;
    let len = up.len();
    let mut out = vec![f64::NAN; len];
    for i in 0..len {
        out[i] = up[i] - down[i];
    }
    Ok(out)
} 