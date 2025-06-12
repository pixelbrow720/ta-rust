// CCI - Commodity Channel Index
use crate::common::{TAError, TAResult};
use crate::price_transform::typprice;
use crate::overlap::sma;

/// Calculates the Commodity Channel Index (CCI).
/// 
/// # Arguments
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `period` - Period for calculation
/// 
/// # Returns
/// Vector of CCI values
pub fn cci(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
) -> TAResult<Vec<f64>> {
    let len = close.len();
    if high.len() != len || low.len() != len {
        return Err(TAError::mismatched_inputs(format!("high: {}, low: {}, close: {}", high.len(), low.len(), len)));
    }
    if len < period {
        return Err(TAError::insufficient_data(period, len));
    }
    let tp = typprice(high, low, close)?;
    let sma_tp = sma(&tp, period)?;
    let mut mad = vec![f64::NAN; len];
    for i in (period - 1)..len {
        let mean = sma_tp[i];
        let sum_abs: f64 = tp[i + 1 - period..=i].iter().map(|&v| (v - mean).abs()).sum();
        mad[i] = sum_abs / period as f64;
    }
    let mut cci = vec![f64::NAN; len];
    for i in (period - 1)..len {
        cci[i] = (tp[i] - sma_tp[i]) / (0.015 * mad[i]);
    }
    Ok(cci)
} 