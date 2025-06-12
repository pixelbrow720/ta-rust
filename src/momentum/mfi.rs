// MFI - Money Flow Index
use crate::common::{TAError, TAResult};
use crate::price_transform::typprice;

/// Calculates the Money Flow Index (MFI).
/// 
/// # Arguments
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `volume` - Volume data
/// * `period` - Period for calculation
/// 
/// # Returns
/// Vector of MFI values
pub fn mfi(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    period: usize,
) -> TAResult<Vec<f64>> {
    let len = close.len();
    if high.len() != len || low.len() != len || volume.len() != len {
        return Err(TAError::mismatched_inputs(format!("high: {}, low: {}, close: {}, volume: {}", high.len(), low.len(), len, volume.len())));
    }
    if len < period + 1 {
        return Err(TAError::insufficient_data(period + 1, len));
    }
    let tp = typprice(high, low, close)?;
    let mut pos_mf = vec![0.0; len];
    let mut neg_mf = vec![0.0; len];
    for i in 1..len {
        let mf = tp[i] * volume[i];
        if tp[i] > tp[i - 1] {
            pos_mf[i] = mf;
        } else if tp[i] < tp[i - 1] {
            neg_mf[i] = mf;
        }
    }
    let mut mfi = vec![f64::NAN; len];
    for i in period..len {
        let pos_sum: f64 = pos_mf[i + 1 - period..=i].iter().sum();
        let neg_sum: f64 = neg_mf[i + 1 - period..=i].iter().sum();
        if neg_sum.abs() < 1e-12 {
            mfi[i] = 100.0;
        } else {
            let mfr = pos_sum / neg_sum;
            mfi[i] = 100.0 - (100.0 / (1.0 + mfr));
        }
    }
    Ok(mfi)
} 