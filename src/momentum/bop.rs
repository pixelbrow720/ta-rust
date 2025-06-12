// BOP - Balance Of Power
use crate::common::{TAError, TAResult};

/// Calculates the Balance Of Power (BOP).
/// 
/// # Arguments
/// * `open` - Open prices
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// 
/// # Returns
/// Vector of BOP values
pub fn bop(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TAResult<Vec<f64>> {
    let len = close.len();
    if open.len() != len || high.len() != len || low.len() != len {
        return Err(TAError::mismatched_inputs(format!("open: {}, high: {}, low: {}, close: {}", open.len(), high.len(), low.len(), len)));
    }
    let mut bop = vec![f64::NAN; len];
    for i in 0..len {
        let denom = high[i] - low[i];
        if denom.abs() < 1e-12 {
            bop[i] = 0.0;
        } else {
            bop[i] = (close[i] - open[i]) / denom;
        }
    }
    Ok(bop)
} 