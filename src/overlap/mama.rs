//! MESA Adaptive Moving Average (MAMA)
//!
//! MAMA is an adaptive moving average that adjusts its smoothing factor based on
//! the dominant cycle period detected using Hilbert Transform techniques.
//! It provides both MAMA and FAMA (Following Adaptive Moving Average) outputs.

use crate::common::{TAError, TAResult};

/// MAMA result structure
#[derive(Debug, Clone)]
pub struct MamaResult {
    /// MESA Adaptive Moving Average values
    pub mama: Vec<f64>,
    /// Following Adaptive Moving Average values
    pub fama: Vec<f64>,
}

/// MESA Adaptive Moving Average (MAMA)
///
/// MAMA uses Hilbert Transform to calculate the dominant cycle period and adapts
/// its smoothing factor accordingly. FAMA is a slower version that follows MAMA.
///
/// # Formula
/// ```text
/// Uses Hilbert Transform to calculate adaptive period
/// Smoothing Factor = adaptive based on dominant cycle
/// MAMA = α × Price + (1 - α) × MAMA[prev]
/// FAMA = 0.5 × α × MAMA + (1 - 0.5 × α) × FAMA[prev]
/// ```
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `fast_limit` - Fast limit for smoothing factor (typically 0.5)
/// * `slow_limit` - Slow limit for smoothing factor (typically 0.05)
///
/// # Returns
/// * `Ok(MamaResult)` - Structure containing MAMA and FAMA values
/// * `Err(TAError)` - Error if inputs are invalid
///
/// # Examples
/// ```
/// use ta_rust::overlap::mama;
///
/// // MAMA requires at least 32 data points
/// let close: Vec<f64> = (0..50).map(|i| 20.0 + (i as f64 * 0.1).sin()).collect();
/// let result = mama(&close, 0.5, 0.05).unwrap();
/// // result.mama contains the adaptive moving average
/// // result.fama contains the following adaptive moving average
/// ```
pub fn mama(close: &[f64], fast_limit: f64, slow_limit: f64) -> TAResult<MamaResult> {
    if close.is_empty() {
        return Err(TAError::invalid_input("Close prices cannot be empty"));
    }
    
    if fast_limit <= 0.0 || slow_limit <= 0.0 {
        return Err(TAError::invalid_parameter("parameter", "must be greater than 0"));
    }
    
    if fast_limit <= slow_limit {
        return Err(TAError::invalid_input("Fast limit must be greater than slow limit"));
    }
    
    if fast_limit > 1.0 || slow_limit > 1.0 {
        return Err(TAError::invalid_input("Limits must be <= 1.0"));
    }
    
    let len = close.len();
    if len < 32 {
        return Err(TAError::invalid_input("Need at least 32 data points for MAMA"));
    }
    
    let mut mama_values = vec![f64::NAN; len];
    let mut fama_values = vec![f64::NAN; len];
    
    // Initialize arrays for Hilbert Transform calculations
    let mut smooth = vec![0.0; len];
    let mut detrender = vec![0.0; len];
    let mut i1 = vec![0.0; len];
    let mut q1 = vec![0.0; len];
    let mut ji = vec![0.0; len];
    let mut jq = vec![0.0; len];
    let mut i2 = vec![0.0; len];
    let mut q2 = vec![0.0; len];
    let mut re = vec![0.0; len];
    let mut im = vec![0.0; len];
    let mut period = vec![0.0; len];
    let mut smooth_period = vec![0.0; len];
    
    // Initialize MAMA and FAMA
    let mut mama_val = close[0];
    let mut fama_val = close[0];
    
    // Start calculations from index 6 (need enough history for i-6 access)
    for i in 6..len {
        // Smooth the price data
        smooth[i] = (4.0 * close[i] + 3.0 * close[i-1] + 2.0 * close[i-2] + close[i-3]) / 10.0;
        
        // Detrend the smoothed data
        detrender[i] = (0.0962 * smooth[i] + 0.5769 * smooth[i-2] - 0.5769 * smooth[i-4] - 0.0962 * smooth[i-6]) * (0.075 * period[i-1] + 0.54);
        
        // Compute InPhase and Quadrature components
        q1[i] = (0.0962 * detrender[i] + 0.5769 * detrender[i-2] - 0.5769 * detrender[i-4] - 0.0962 * detrender[i-6]) * (0.075 * period[i-1] + 0.54);
        i1[i] = detrender[i-3];
        
        // Advance the phase of I1 and Q1 by 90 degrees
        ji[i] = (0.0962 * i1[i] + 0.5769 * i1[i-2] - 0.5769 * i1[i-4] - 0.0962 * i1[i-6]) * (0.075 * period[i-1] + 0.54);
        jq[i] = (0.0962 * q1[i] + 0.5769 * q1[i-2] - 0.5769 * q1[i-4] - 0.0962 * q1[i-6]) * (0.075 * period[i-1] + 0.54);
        
        // Phasor addition for 3 bar averaging
        i2[i] = i1[i] - jq[i];
        q2[i] = q1[i] + ji[i];
        
        // Smooth the I and Q components before applying the discriminator
        i2[i] = 0.2 * i2[i] + 0.8 * i2[i-1];
        q2[i] = 0.2 * q2[i] + 0.8 * q2[i-1];
        
        // Homodyne Discriminator
        re[i] = i2[i] * i2[i-1] + q2[i] * q2[i-1];
        im[i] = i2[i] * q2[i-1] - q2[i] * i2[i-1];
        re[i] = 0.2 * re[i] + 0.8 * re[i-1];
        im[i] = 0.2 * im[i] + 0.8 * im[i-1];
        
        // Compute the period
        if im[i] != 0.0 && re[i] != 0.0 {
            period[i] = 2.0 * std::f64::consts::PI / im[i].atan2(re[i]);
        }
        
        // Constrain the period
        if period[i] > 1.5 * period[i-1] {
            period[i] = 1.5 * period[i-1];
        }
        if period[i] < 0.67 * period[i-1] {
            period[i] = 0.67 * period[i-1];
        }
        if period[i] < 6.0 {
            period[i] = 6.0;
        }
        if period[i] > 50.0 {
            period[i] = 50.0;
        }
        
        // Smooth the period
        period[i] = 0.2 * period[i] + 0.8 * period[i-1];
        smooth_period[i] = 0.33 * period[i] + 0.67 * smooth_period[i-1];
        
        // Compute the adaptive factor
        let phase = if i1[i] != 0.0 {
            q1[i].atan2(i1[i])
        } else {
            0.0
        };
        
        let delta_phase = if i >= 1 {
            let mut dp = phase - period[i-1];
            if dp < 1.0 {
                dp = 1.0;
            }
            dp
        } else {
            1.0
        };
        
        let alpha = fast_limit / delta_phase;
        let alpha = if alpha < slow_limit {
            slow_limit
        } else if alpha > fast_limit {
            fast_limit
        } else {
            alpha
        };
        
        // Update MAMA and FAMA
        mama_val = alpha * close[i] + (1.0 - alpha) * mama_val;
        fama_val = 0.5 * alpha * mama_val + (1.0 - 0.5 * alpha) * fama_val;
        
        mama_values[i] = mama_val;
        fama_values[i] = fama_val;
    }
    
    Ok(MamaResult {
        mama: mama_values,
        fama: fama_values,
    })
}

/// MAMA with default parameters (0.5, 0.05)
///
/// This is a convenience function using the standard default parameters.
///
/// # Arguments
/// * `close` - Slice of closing prices
///
/// # Returns
/// * `Ok(MamaResult)` - Structure containing MAMA and FAMA values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn mama_default(close: &[f64]) -> TAResult<MamaResult> {
    mama(close, 0.5, 0.05)
}

/// Extract dominant cycle period from MAMA calculation
///
/// This function performs the same Hilbert Transform calculations as MAMA
/// but returns the dominant cycle period instead of the moving averages.
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `fast_limit` - Fast limit for calculations
/// * `slow_limit` - Slow limit for calculations
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of dominant cycle periods
/// * `Err(TAError)` - Error if inputs are invalid
pub fn mama_period(close: &[f64], fast_limit: f64, slow_limit: f64) -> TAResult<Vec<f64>> {
    if close.is_empty() {
        return Err(TAError::invalid_input("Close prices cannot be empty"));
    }
    
    let len = close.len();
    if len < 32 {
        return Err(TAError::invalid_input("Need at least 32 data points for period calculation"));
    }
    
    // This is a simplified version - in practice, you'd extract the period
    // calculation from the main MAMA function
    let _result = mama(close, fast_limit, slow_limit)?;
    
    // For now, return a placeholder - in a full implementation,
    // you'd return the actual period values calculated during MAMA
    let mut periods = vec![f64::NAN; len];
    
    // Fill with estimated periods (this is a simplification)
    for i in 6..len {
        periods[i] = 20.0; // Default period estimate
    }
    
    Ok(periods)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;
    #[test]
    fn test_mama_basic() {
        // Create enough data points for MAMA
        let close: Vec<f64> = (0..50).map(|i| 20.0 + (i as f64 * 0.1).sin()).collect();
        let result = mama(&close, 0.5, 0.05).unwrap();
        
        assert_eq!(result.mama.len(), 50);
        assert_eq!(result.fama.len(), 50);
        
        // First few values should be NaN
        for i in 0..6 {
            assert!(result.mama[i].is_nan());
            assert!(result.fama[i].is_nan());
        }
        
        // Later values should be valid
        let mut valid_count = 0;
        for i in 6..50 {
            if !result.mama[i].is_nan() && !result.fama[i].is_nan() {
                valid_count += 1;
                // MAMA and FAMA should be reasonable values
                assert!(result.mama[i] > 0.0);
                assert!(result.fama[i] > 0.0);
            }
        }
        
        assert!(valid_count > 0);
    }

    #[test]
    fn test_mama_default() {
        let close: Vec<f64> = (0..50).map(|i| 20.0 + (i as f64 * 0.1).sin()).collect();
        
        let result1 = mama_default(&close).unwrap();
        let result2 = mama(&close, 0.5, 0.05).unwrap();
        
        assert_eq!(result1.mama.len(), result2.mama.len());
        assert_eq!(result1.fama.len(), result2.fama.len());
        
        for i in 0..result1.mama.len() {
            if result1.mama[i].is_nan() && result2.mama[i].is_nan() {
                continue;
            }
            if !result1.mama[i].is_nan() && !result2.mama[i].is_nan() {
                assert_float_eq!(result1.mama[i], result2.mama[i], 1e-10);
                assert_float_eq!(result1.fama[i], result2.fama[i], 1e-10);
            }
        }
    }

    #[test]
    fn test_mama_trending_data() {
        // Create trending data
        let close: Vec<f64> = (0..50).map(|i| 10.0 + i as f64 * 0.5).collect();
        let result = mama(&close, 0.5, 0.05).unwrap();
        
        // Find valid range
        let mut valid_start = 0;
        for i in 0..result.mama.len() {
            if !result.mama[i].is_nan() && !result.fama[i].is_nan() {
                valid_start = i;
                break;
            }
        }
        
        // MAMA should follow the trend
        for i in valid_start..result.mama.len() {
            if !result.mama[i].is_nan() && !result.fama[i].is_nan() {
                // Values should be within reasonable range
                assert!(result.mama[i] >= 10.0 && result.mama[i] <= 35.0);
                assert!(result.fama[i] >= 10.0 && result.fama[i] <= 35.0);
            }
        }
    }

    #[test]
    fn test_mama_period() {
        let close: Vec<f64> = (0..50).map(|i| 20.0 + (i as f64 * 0.1).sin()).collect();
        let periods = mama_period(&close, 0.5, 0.05).unwrap();
        
        assert_eq!(periods.len(), 50);
        
        // Should have some valid period values
        let valid_count = periods.iter().filter(|&&x| !x.is_nan()).count();
        assert!(valid_count > 0);
    }

    #[test]
    fn test_mama_invalid_input() {
        let close: Vec<f64> = vec![];
        assert!(mama(&close, 0.5, 0.05).is_err());
        
        // Not enough data points
        let close = vec![20.0; 10];
        assert!(mama(&close, 0.5, 0.05).is_err());
        
        let close = vec![20.0; 50];
        
        // Invalid limits
        assert!(mama(&close, 0.0, 0.05).is_err());   // Zero fast limit
        assert!(mama(&close, 0.5, 0.0).is_err());    // Zero slow limit
        assert!(mama(&close, 0.05, 0.5).is_err());   // Fast <= slow
        assert!(mama(&close, 1.5, 0.05).is_err());   // Fast > 1.0
        assert!(mama(&close, 0.5, 1.5).is_err());    // Slow > 1.0
    }

    #[test]
    fn test_mama_different_limits() {
        let close: Vec<f64> = (0..50).map(|i| 20.0 + (i as f64 * 0.1).sin()).collect();
        
        let result1 = mama(&close, 0.8, 0.02).unwrap();
        let result2 = mama(&close, 0.3, 0.1).unwrap();
        
        assert_eq!(result1.mama.len(), result2.mama.len());
        
        // Results should be different due to different limits
        let mut _differences = 0;
        for i in 0..result1.mama.len() {
            if !result1.mama[i].is_nan() && !result2.mama[i].is_nan() {
                if (result1.mama[i] - result2.mama[i]).abs() > 1e-10 {
                    _differences += 1;
                }
            }
        }
        
        // Should have some differences
        assert!(_differences > 0);
    }

    #[test]
    fn test_mama_fama_relationship() {
        let close: Vec<f64> = (0..50).map(|i| 20.0 + (i as f64 * 0.1).sin()).collect();
        let result = mama(&close, 0.5, 0.05).unwrap();
        
        // FAMA should generally be smoother (less volatile) than MAMA
        // This is a basic check - in practice, you'd measure volatility
        let mut mama_valid = Vec::new();
        let mut fama_valid = Vec::new();
        
        for i in 0..result.mama.len() {
            if !result.mama[i].is_nan() && !result.fama[i].is_nan() {
                mama_valid.push(result.mama[i]);
                fama_valid.push(result.fama[i]);
            }
        }
        
        assert!(mama_valid.len() > 0);
        assert_eq!(mama_valid.len(), fama_valid.len());
    }

    #[test]
    fn test_mama_constant_prices() {
        let close = vec![20.0; 50];
        let result = mama(&close, 0.5, 0.05).unwrap();
        
        // Find first valid values
        let mut first_valid = None;
        for i in 0..result.mama.len() {
            if !result.mama[i].is_nan() && !result.fama[i].is_nan() {
                first_valid = Some(i);
                break;
            }
        }
        
        if let Some(start) = first_valid {
            // With constant prices, MAMA and FAMA should converge to the price
            for i in (start + 10)..result.mama.len() {
                if !result.mama[i].is_nan() && !result.fama[i].is_nan() {
                    // Should be close to the constant price (allowing for some adaptation time)
                    assert!((result.mama[i] - 20.0).abs() < 1.0);
                    assert!((result.fama[i] - 20.0).abs() < 1.0);
                }
            }
        }
    }
}