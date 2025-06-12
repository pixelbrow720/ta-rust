// PPO - Percentage Price Oscillator
use crate::common::{TAError, TAResult, MAType};
use crate::overlap::ma;

pub fn ppo(
    price: &[f64],
    fast_period: usize,
    slow_period: usize,
    ma_type: MAType,
) -> TAResult<Vec<f64>> {
    if price.len() < slow_period {
        return Err(TAError::InsufficientData);
    }
    let fast = ma(price, fast_period, ma_type)?;
    let slow = ma(price, slow_period, ma_type)?;
    let mut ppo = vec![f64::NAN; price.len()];
    for i in 0..price.len() {
        if i < slow_period - 1 {
            continue;
        }
        if slow[i].abs() < 1e-12 {
            ppo[i] = 0.0;
        } else {
            ppo[i] = 100.0 * (fast[i] - slow[i]) / slow[i];
        }
    }
    Ok(ppo)
} 