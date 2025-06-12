// PLUS_DM - Plus Directional Movement
use crate::common::{TAError, TAResult};

pub fn plus_dm(high: &[f64], low: &[f64]) -> TAResult<Vec<f64>> {
    let len = high.len();
    if low.len() != len {
        return Err(TAError::MismatchedInputLength);
    }
    let mut out = vec![f64::NAN; len];
    for i in 1..len {
        let up = high[i] - high[i - 1];
        let down = low[i - 1] - low[i];
        if up > 0.0 && up > down {
            out[i] = up;
        } else {
            out[i] = 0.0;
        }
    }
    Ok(out)
} 