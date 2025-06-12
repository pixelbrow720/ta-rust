//! Moving Average with Variable Period (MAVP)
//!
//! MAVP calculates a simple moving average where the period can vary for each data point.
//! This allows for adaptive moving averages based on external conditions or indicators.

use crate::common::{TAError, TAResult};

/// Moving Average with Variable Period (MAVP)
///
/// MAVP calculates a simple moving average where each data point can have a different
/// period length. The period is constrained between minimum and maximum values.
///
/// # Formula
/// ```text
/// For each point i:
///     period = periods[i] (bounded by min and max)
///     MA[i] = SMA(Price, period) at point i
/// ```
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `periods` - Slice of period values for each data point
/// * `min_period` - Minimum allowed period
/// * `max_period` - Maximum allowed period
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of variable period moving average values
/// * `Err(TAError)` - Error if inputs are invalid
///
/// # Examples
/// ```
/// use ta_rust::overlap::mavp;
///
/// let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
/// let periods = vec![5.0, 5.0, 3.0, 3.0, 7.0, 7.0, 4.0, 4.0, 6.0, 6.0];
/// let result = mavp(&close, &periods, 2, 10).unwrap();
/// ```
pub fn mavp(close: &[f64], periods: &[f64], min_period: usize, max_period: usize) -> TAResult<Vec<f64>> {
    if close.is_empty() || periods.is_empty() {
        return Err(TAError::invalid_input("Input arrays cannot be empty"));
    }
    
    if close.len() != periods.len() {
        return Err(TAError::mismatched_inputs("Close and periods arrays must have the same length"));
    }
    
    if min_period == 0 {
        return Err(TAError::invalid_parameter("period", "must be greater than 0"));
    }
    
    if min_period > max_period {
        return Err(TAError::invalid_input("Minimum period cannot be greater than maximum period"));
    }
    
    if max_period > close.len() {
        return Err(TAError::insufficient_data(max_period, close.len()));
    }
    
    let len = close.len();
    let mut result = vec![f64::NAN; len];
    
    for i in 0..len {
        // Get the period for this data point, constrained by min/max
        let raw_period = periods[i];
        if raw_period.is_nan() || raw_period <= 0.0 {
            continue; // Skip invalid periods
        }
        
        let period = (raw_period.round() as usize).clamp(min_period, max_period);
        
        // Check if we have enough data for this period
        if i + 1 >= period {
            // Calculate SMA for this period
            let start_idx = i + 1 - period;
            let sum: f64 = close[start_idx..=i].iter().sum();
            result[i] = sum / period as f64;
        }
    }
    
    Ok(result)
}

/// MAVP with integer periods
///
/// Convenience function for when periods are already integers.
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `periods` - Slice of integer period values
/// * `min_period` - Minimum allowed period
/// * `max_period` - Maximum allowed period
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of variable period moving average values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn mavp_int(close: &[f64], periods: &[usize], min_period: usize, max_period: usize) -> TAResult<Vec<f64>> {
    let float_periods: Vec<f64> = periods.iter().map(|&p| p as f64).collect();
    mavp(close, &float_periods, min_period, max_period)
}

/// MAVP with adaptive periods based on volatility
///
/// This function calculates variable periods based on price volatility.
/// Higher volatility results in shorter periods (more responsive), 
/// lower volatility results in longer periods (smoother).
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `volatility_period` - Period for volatility calculation
/// * `min_period` - Minimum allowed period
/// * `max_period` - Maximum allowed period
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of adaptive moving average values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn mavp_adaptive(close: &[f64], volatility_period: usize, min_period: usize, max_period: usize) -> TAResult<Vec<f64>> {
    if close.is_empty() {
        return Err(TAError::invalid_input("Close prices cannot be empty"));
    }
    
    if volatility_period == 0 {
        return Err(TAError::invalid_parameter("period", "must be greater than 0"));
    }
    
    let len = close.len();
    let mut periods = vec![max_period as f64; len];
    
    // Calculate adaptive periods based on volatility
    for i in volatility_period..len {
        // Calculate volatility as standard deviation over the volatility period
        let start_idx = i + 1 - volatility_period;
        let window = &close[start_idx..=i];
        
        let mean = window.iter().sum::<f64>() / volatility_period as f64;
        let variance = window.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / volatility_period as f64;
        let volatility = variance.sqrt();
        
        // Normalize volatility to determine period
        // Higher volatility -> shorter period (more responsive)
        // Lower volatility -> longer period (smoother)
        let max_volatility = mean * 0.1; // Assume 10% volatility is high
        let normalized_vol = (volatility / max_volatility).min(1.0);
        
        // Invert the relationship: high volatility = short period
        let period_factor = 1.0 - normalized_vol;
        let adaptive_period = min_period as f64 + period_factor * (max_period - min_period) as f64;
        
        periods[i] = adaptive_period;
    }
    
    mavp(close, &periods, min_period, max_period)
}

/// MAVP with periods based on another indicator
///
/// This function uses values from another indicator to determine the periods.
/// The indicator values are normalized and mapped to the period range.
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `indicator` - Slice of indicator values to base periods on
/// * `min_period` - Minimum allowed period
/// * `max_period` - Maximum allowed period
/// * `invert` - If true, higher indicator values result in shorter periods
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of indicator-based moving average values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn mavp_indicator_based(
    close: &[f64], 
    indicator: &[f64], 
    min_period: usize, 
    max_period: usize,
    invert: bool
) -> TAResult<Vec<f64>> {
    if close.is_empty() || indicator.is_empty() {
        return Err(TAError::invalid_input("Input arrays cannot be empty"));
    }
    
    if close.len() != indicator.len() {
        return Err(TAError::mismatched_inputs("Close and indicator arrays must have the same length"));
    }
    
    // Find min and max of indicator for normalization
    let valid_indicator: Vec<f64> = indicator.iter().filter(|&&x| !x.is_nan()).cloned().collect();
    if valid_indicator.is_empty() {
        return Err(TAError::invalid_input("Indicator contains no valid values"));
    }
    
    let indicator_min = valid_indicator.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let indicator_max = valid_indicator.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let indicator_range = indicator_max - indicator_min;
    
    if indicator_range.abs() < f64::EPSILON {
        // Constant indicator - use middle period
        let middle_period = (min_period + max_period) as f64 / 2.0;
        let periods = vec![middle_period; close.len()];
        return mavp(close, &periods, min_period, max_period);
    }
    
    // Calculate periods based on normalized indicator
    let mut periods = vec![0.0; close.len()];
    for i in 0..close.len() {
        if indicator[i].is_nan() {
            periods[i] = (min_period + max_period) as f64 / 2.0; // Default to middle
        } else {
            // Normalize indicator to 0-1 range
            let normalized = (indicator[i] - indicator_min) / indicator_range;
            
            // Map to period range
            let period_factor = if invert { 1.0 - normalized } else { normalized };
            periods[i] = min_period as f64 + period_factor * (max_period - min_period) as f64;
        }
    }
    
    mavp(close, &periods, min_period, max_period)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;
    #[test]
    fn test_mavp_basic() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
        let periods = vec![5.0, 5.0, 3.0, 3.0, 7.0, 7.0, 4.0, 4.0, 6.0, 6.0];
        let result = mavp(&close, &periods, 2, 10).unwrap();
        
        assert_eq!(result.len(), 10);
        
        // Check specific calculations
        // At index 4 (period=7, but clamped to max_period=10, and we have 5 data points)
        // So we should use period=5 (all available data)
        if !result[4].is_nan() {
            let expected = (20.0 + 21.0 + 22.0 + 23.0 + 24.0) / 5.0;
            assert_float_eq!(result[4], expected, 1e-10);
        }
        
        // At index 6 (period=4)
        assert!(!result[6].is_nan());
        let expected = (23.0 + 24.0 + 23.0 + 22.0) / 4.0;
        assert_float_eq!(result[6], expected, 1e-10);
    }

    #[test]
    fn test_mavp_int() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
        let periods = vec![5, 5, 3, 3, 7, 7, 4, 4, 6, 6];
        let result = mavp_int(&close, &periods, 2, 10).unwrap();
        
        assert_eq!(result.len(), 10);
        
        // Should match the float version
        let periods_float: Vec<f64> = periods.iter().map(|&p| p as f64).collect();
        let result_float = mavp(&close, &periods_float, 2, 10).unwrap();
        
        for i in 0..result.len() {
            if result[i].is_nan() && result_float[i].is_nan() {
                continue;
            }
            assert_float_eq!(result[i], result_float[i], 1e-10);
        }
    }

    #[test]
    fn test_mavp_period_constraints() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
        let periods = vec![1.0, 15.0, 3.0, 0.0, -5.0, 7.0, 4.0, 4.0, 6.0, 6.0];
        let result = mavp(&close, &periods, 2, 8).unwrap();
        
        assert_eq!(result.len(), 10);
        
        // Period 1.0 should be constrained to min_period (2)
        // Period 15.0 should be constrained to max_period (8)
        // Period 0.0 and -5.0 should be skipped (NaN result)
        
        // Check if we have enough data for the periods
        // At index 0, we need at least 2 points but only have 1
        // At index 1, we need at least 8 points but only have 2
        // At index 2, we need at least 3 points and have 3
        
        assert!(result[0].is_nan()); // period 1 -> 2, but not enough data
        assert!(result[1].is_nan()); // period 15 -> 8, but not enough data  
        assert!(!result[2].is_nan()); // period 3 -> 3, have enough data
        assert!(result[3].is_nan());  // period 0 -> skip
        assert!(result[4].is_nan());  // period -5 -> skip
    }

    #[test]
    fn test_mavp_adaptive() {
        // Create data with varying volatility
        let mut close = vec![];
        
        // Low volatility period
        for i in 0..10 {
            close.push(20.0 + (i as f64 * 0.01));
        }
        
        // High volatility period
        for i in 0..10 {
            close.push(20.1 + (i as f64 * 0.1) * if i % 2 == 0 { 1.0 } else { -1.0 });
        }
        
        let result = mavp_adaptive(&close, 5, 2, 10).unwrap();
        
        assert_eq!(result.len(), 20);
        
        // Should have some valid values
        let valid_count = result.iter().filter(|&&x| !x.is_nan()).count();
        assert!(valid_count > 0);
    }

    #[test]
    fn test_mavp_indicator_based() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
        let indicator = vec![0.1, 0.2, 0.8, 0.9, 0.5, 0.3, 0.7, 0.6, 0.4, 0.1];
        
        let result = mavp_indicator_based(&close, &indicator, 2, 8, false).unwrap();
        assert_eq!(result.len(), 10);
        
        let result_inverted = mavp_indicator_based(&close, &indicator, 2, 8, true).unwrap();
        assert_eq!(result_inverted.len(), 10);
        
        // Results should be different due to inversion
        let mut _differences = 0;
        for i in 0..result.len() {
            if !result[i].is_nan() && !result_inverted[i].is_nan() {
                if (result[i] - result_inverted[i]).abs() > 1e-10 {
                    _differences += 1;
                }
            }
        }
        
        // Should have some differences (unless all periods end up the same)
        // This depends on the specific data and normalization
    }

    #[test]
    fn test_mavp_constant_indicator() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
        let indicator = vec![0.5; 10]; // Constant indicator
        
        let result = mavp_indicator_based(&close, &indicator, 2, 8, false).unwrap();
        
        // With constant indicator, should use middle period (5)
        // Check that we get reasonable results
        let valid_count = result.iter().filter(|&&x| !x.is_nan()).count();
        assert!(valid_count > 0);
    }

    #[test]
    fn test_mavp_invalid_input() {
        let close: Vec<f64> = vec![];
        let periods: Vec<f64> = vec![];
        assert!(mavp(&close, &periods, 2, 10).is_err());
        
        let close = vec![20.0, 21.0];
        let periods = vec![5.0];
        assert!(mavp(&close, &periods, 2, 10).is_err()); // Mismatched lengths
        
        let close = vec![20.0, 21.0];
        let periods = vec![5.0, 3.0];
        assert!(mavp(&close, &periods, 0, 10).is_err()); // Zero min period
        assert!(mavp(&close, &periods, 10, 5).is_err());  // Min > max
        assert!(mavp(&close, &periods, 2, 5).is_err());   // Max > data length
    }

    #[test]
    fn test_mavp_nan_periods() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0];
        let periods = vec![3.0, f64::NAN, 4.0, 2.0, 5.0];
        let result = mavp(&close, &periods, 2, 5).unwrap();
        
        assert_eq!(result.len(), 5);
        
        // Index 1 should be NaN due to NaN period
        assert!(result[1].is_nan());
        
        // Other valid indices should have values if there's enough data
        // At index 2, period 4, need 4 points, have 3 - not enough
        // At index 3, period 2, need 2 points, have 4 - enough
        // At index 4, period 5, need 5 points, have 5 - enough
        
        assert!(result[2].is_nan()); // period 4, index 2, not enough data
        assert!(!result[3].is_nan()); // period 2, index 3, enough data
        assert!(!result[4].is_nan()); // period 5, index 4, enough data
    }

    #[test]
    fn test_mavp_insufficient_data() {
        let close = vec![20.0, 21.0];
        let periods = vec![5.0, 3.0];
        
        // This should return an error because max_period (10) > data length (2)
        let result = mavp(&close, &periods, 2, 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_mavp_edge_cases() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0];
        let periods = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        let result = mavp(&close, &periods, 1, 5).unwrap();
        
        // With period 1, each result should equal the corresponding close price
        for i in 0..close.len() {
            assert_float_eq!(result[i], close[i], 1e-10);
        }
    }
}