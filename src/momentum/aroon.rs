// AROON - Aroon Up/Down
use crate::common::{TAError, TAResult};

/// Calculates the Aroon Up and Aroon Down indicators.
/// 
/// # Arguments
/// * `high` - High prices
/// * `low` - Low prices
/// * `period` - Period for calculation
/// 
/// # Returns
/// Tuple of (Aroon Up, Aroon Down)
pub fn aroon(
    high: &[f64],
    low: &[f64],
    period: usize,
) -> TAResult<(Vec<f64>, Vec<f64>)> {
    let len = high.len();
    if low.len() != len {
        return Err(TAError::mismatched_inputs(format!("high: {}, low: {}", len, low.len())));
    }
    if len < period {
        return Err(TAError::insufficient_data(period, len));
    }
    let mut up = vec![f64::NAN; len];
    let mut down = vec![f64::NAN; len];
    for i in (period - 1)..len {
        let mut max_idx = 0;
        let mut min_idx = 0;
        let mut max = f64::MIN;
        let mut min = f64::MAX;
        for j in 0..period {
            let idx = i + 1 - period + j;
            if high[idx] > max {
                max = high[idx];
                max_idx = j;
            }
            if low[idx] < min {
                min = low[idx];
                min_idx = j;
            }
        }
        up[i] = 100.0 * (period as f64 - max_idx as f64) / period as f64;
        down[i] = 100.0 * (period as f64 - min_idx as f64) / period as f64;
    }
    Ok((up, down))
} 