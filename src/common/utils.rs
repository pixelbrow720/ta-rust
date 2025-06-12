//! Utility functions for TA-Rust

use crate::common::{TAError, TAResult, Price, Period};

/// Validates that input data is not empty
pub fn validate_not_empty<T>(data: &[T], name: &str) -> TAResult<()> {
    if data.is_empty() {
        Err(TAError::invalid_input(format!("{} cannot be empty", name)))
    } else {
        Ok(())
    }
}

/// Validates that there is sufficient data for the given period
pub fn validate_sufficient_data<T>(data: &[T], period: Period, _name: &str) -> TAResult<()> {
    if data.len() < period {
        Err(TAError::insufficient_data(period, data.len()))
    } else {
        Ok(())
    }
}

/// Validates that a period is positive
pub fn validate_period(period: Period, name: &str) -> TAResult<()> {
    if period == 0 {
        Err(TAError::invalid_parameter(
            name,
            "period must be greater than 0",
        ))
    } else {
        Ok(())
    }
}

/// Validates that multiple input arrays have the same length
pub fn validate_same_length<T, U>(data1: &[T], data2: &[U], name1: &str, name2: &str) -> TAResult<()> {
    if data1.len() != data2.len() {
        Err(TAError::mismatched_inputs(format!(
            "{} length ({}) != {} length ({})",
            name1,
            data1.len(),
            name2,
            data2.len()
        )))
    } else {
        Ok(())
    }
}

/// Validates that input prices are valid (not NaN or infinite)
pub fn validate_prices(prices: &[Price], name: &str) -> TAResult<()> {
    for (i, &price) in prices.iter().enumerate() {
        if !price.is_finite() {
            return Err(TAError::invalid_input(format!(
                "{} contains invalid value at index {}: {}",
                name, i, price
            )));
        }
    }
    Ok(())
}

/// Validates OHLC data consistency (High >= Low, High >= Open, High >= Close, Low <= Open, Low <= Close)
pub fn validate_ohlc(
    open: &[Price],
    high: &[Price],
    low: &[Price],
    close: &[Price],
) -> TAResult<()> {
    // Check all arrays have same length
    let len = open.len();
    if high.len() != len || low.len() != len || close.len() != len {
        return Err(TAError::mismatched_inputs(
            "OHLC arrays must have the same length".to_string(),
        ));
    }

    // Validate each OHLC bar
    for i in 0..len {
        let (o, h, l, c) = (open[i], high[i], low[i], close[i]);

        // Check for invalid values
        if !o.is_finite() || !h.is_finite() || !l.is_finite() || !c.is_finite() {
            return Err(TAError::invalid_input(format!(
                "Invalid OHLC values at index {}: O={}, H={}, L={}, C={}",
                i, o, h, l, c
            )));
        }

        // Check OHLC constraints
        if h < l {
            return Err(TAError::invalid_input(format!(
                "High ({}) < Low ({}) at index {}",
                h, l, i
            )));
        }
        if h < o || h < c {
            return Err(TAError::invalid_input(format!(
                "High ({}) is not the highest value at index {} (O={}, C={})",
                h, i, o, c
            )));
        }
        if l > o || l > c {
            return Err(TAError::invalid_input(format!(
                "Low ({}) is not the lowest value at index {} (O={}, C={})",
                l, i, o, c
            )));
        }
    }

    Ok(())
}

/// Allocates and initializes an output vector with NaN values
pub fn allocate_output(size: usize) -> Vec<Price> {
    vec![Price::NAN; size]
}

/// Allocates and initializes an output vector with a specific value
pub fn allocate_output_with_value(size: usize, value: Price) -> Vec<Price> {
    vec![value; size]
}

/// Calculates the exponential moving average multiplier
pub fn ema_multiplier(period: Period) -> Price {
    2.0 / (period as Price + 1.0)
}

/// Calculates the Wilder's smoothing multiplier (used in RSI, ATR, etc.)
pub fn wilders_multiplier(period: Period) -> Price {
    1.0 / period as Price
}

/// Finds the highest value in a slice
pub fn highest(data: &[Price]) -> Price {
    data.iter().fold(Price::NEG_INFINITY, |acc, &x| acc.max(x))
}

/// Finds the lowest value in a slice
pub fn lowest(data: &[Price]) -> Price {
    data.iter().fold(Price::INFINITY, |acc, &x| acc.min(x))
}

/// Finds the highest value in a slice over a specific period
pub fn highest_in_period(data: &[Price], start: usize, period: Period) -> Price {
    let end = (start + period).min(data.len());
    highest(&data[start..end])
}

/// Finds the lowest value in a slice over a specific period
pub fn lowest_in_period(data: &[Price], start: usize, period: Period) -> Price {
    let end = (start + period).min(data.len());
    lowest(&data[start..end])
}

/// Finds the index of the highest value in a slice
pub fn highest_index(data: &[Price]) -> usize {
    data.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0)
}

/// Finds the index of the lowest value in a slice
pub fn lowest_index(data: &[Price]) -> usize {
    data.iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0)
}

/// Calculates the sum of a slice
pub fn sum(data: &[Price]) -> Price {
    data.iter().sum()
}

/// Calculates the mean of a slice
pub fn mean(data: &[Price]) -> Price {
    if data.is_empty() {
        Price::NAN
    } else {
        sum(data) / data.len() as Price
    }
}

/// Calculates the variance of a slice
pub fn variance(data: &[Price]) -> Price {
    if data.len() < 2 {
        return Price::NAN;
    }

    let mean_val = mean(data);
    let sum_sq_diff: Price = data.iter().map(|&x| (x - mean_val).powi(2)).sum();
    sum_sq_diff / (data.len() - 1) as Price
}

/// Calculates the standard deviation of a slice
pub fn std_dev(data: &[Price]) -> Price {
    variance(data).sqrt()
}

/// Calculates the mean absolute deviation
pub fn mean_absolute_deviation(data: &[Price], mean_val: Price) -> Price {
    if data.is_empty() {
        return Price::NAN;
    }

    let sum_abs_diff: Price = data.iter().map(|&x| (x - mean_val).abs()).sum();
    sum_abs_diff / data.len() as Price
}

/// Checks if a value is approximately equal to another within a tolerance
pub fn approx_equal(a: Price, b: Price, tolerance: Price) -> bool {
    (a - b).abs() <= tolerance
}

/// Rounds a value to a specific number of decimal places
pub fn round_to_decimals(value: Price, decimals: u32) -> Price {
    let multiplier = 10.0_f64.powi(decimals as i32);
    (value * multiplier).round() / multiplier
}

/// Clamps a value between min and max
pub fn clamp(value: Price, min: Price, max: Price) -> Price {
    value.max(min).min(max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_validate_not_empty() {
        assert!(validate_not_empty(&[1, 2, 3], "data").is_ok());
        assert!(validate_not_empty(&[] as &[i32], "data").is_err());
    }

    #[test]
    fn test_validate_sufficient_data() {
        assert!(validate_sufficient_data(&[1, 2, 3, 4, 5], 3, "data").is_ok());
        assert!(validate_sufficient_data(&[1, 2], 3, "data").is_err());
    }

    #[test]
    fn test_validate_period() {
        assert!(validate_period(5, "period").is_ok());
        assert!(validate_period(0, "period").is_err());
    }

    #[test]
    fn test_validate_same_length() {
        assert!(validate_same_length(&[1, 2, 3], &[4, 5, 6], "a", "b").is_ok());
        assert!(validate_same_length(&[1, 2], &[4, 5, 6], "a", "b").is_err());
    }

    #[test]
    fn test_validate_prices() {
        assert!(validate_prices(&[1.0, 2.0, 3.0], "prices").is_ok());
        assert!(validate_prices(&[1.0, Price::NAN, 3.0], "prices").is_err());
        assert!(validate_prices(&[1.0, Price::INFINITY, 3.0], "prices").is_err());
    }

    #[test]
    fn test_validate_ohlc() {
        let open = vec![10.0, 11.0, 12.0];
        let high = vec![12.0, 13.0, 14.0];
        let low = vec![9.0, 10.0, 11.0];
        let close = vec![11.0, 12.0, 13.0];

        assert!(validate_ohlc(&open, &high, &low, &close).is_ok());

        // Test invalid OHLC (high < low)
        let invalid_high = vec![8.0, 13.0, 14.0];
        assert!(validate_ohlc(&open, &invalid_high, &low, &close).is_err());
    }

    #[test]
    fn test_multipliers() {
        assert_relative_eq!(ema_multiplier(10), 2.0 / 11.0, epsilon = 1e-10);
        assert_relative_eq!(wilders_multiplier(14), 1.0 / 14.0, epsilon = 1e-10);
    }

    #[test]
    fn test_highest_lowest() {
        let data = vec![1.0, 5.0, 3.0, 9.0, 2.0];
        assert_eq!(highest(&data), 9.0);
        assert_eq!(lowest(&data), 1.0);
        assert_eq!(highest_index(&data), 3);
        assert_eq!(lowest_index(&data), 0);
    }

    #[test]
    fn test_statistical_functions() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(sum(&data), 15.0);
        assert_eq!(mean(&data), 3.0);
        assert_relative_eq!(std_dev(&data), (2.5_f64).sqrt(), epsilon = 1e-10);
    }

    #[test]
    fn test_utility_functions() {
        assert!(approx_equal(1.0, 1.001, 0.01));
        assert!(!approx_equal(1.0, 1.1, 0.01));

        assert_eq!(round_to_decimals(3.14159, 2), 3.14);
        assert_eq!(clamp(5.0, 1.0, 3.0), 3.0);
        assert_eq!(clamp(-1.0, 1.0, 3.0), 1.0);
        assert_eq!(clamp(2.0, 1.0, 3.0), 2.0);
    }
}