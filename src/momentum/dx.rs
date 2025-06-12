// DX - Directional Movement Index
use crate::common::{TAError, TAResult};
use crate::momentum::{plus_di, minus_di};

/// Calculates the Directional Movement Index.
/// 
/// # Arguments
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `period` - Period for calculation
/// 
/// # Returns
/// Vector of DX values
pub fn dx(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
) -> TAResult<Vec<f64>> {
    let len = close.len();
    if high.len() != len || low.len() != len {
        return Err(TAError::mismatched_inputs(format!("high: {}, low: {}, close: {}", high.len(), low.len(), len)));
    }
    let plusdi = plus_di(high, low, close, period)?;
    let minusdi = minus_di(high, low, close, period)?;
    let mut out = vec![f64::NAN; len];
    for i in 0..len {
        let denom = plusdi[i].abs() + minusdi[i].abs();
        if denom < 1e-12 {
            out[i] = 0.0;
        } else {
            out[i] = 100.0 * (plusdi[i] - minusdi[i]).abs() / denom;
        }
    }
    Ok(out)
} 