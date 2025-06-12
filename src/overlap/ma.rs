//! Generic Moving Average (MA)

use crate::common::{TAResult, Price, Period, MAType};
use crate::common::utils::{validate_not_empty, validate_period, validate_sufficient_data};
use crate::overlap::{sma, ema, wma, dema, tema, trima};

/// Calculates a Moving Average using the specified type
///
/// This is a generic function that can calculate different types of moving averages
/// based on the MAType parameter. It serves as a unified interface for all
/// moving average calculations.
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods for the moving average
/// - `ma_type`: Type of moving average to calculate
///
/// # Returns
/// Vector of MA values. The number of leading NaN values depends on the MA type.
///
/// # Errors
/// - `EmptyInput` if data is empty
/// - `InvalidParameter` if period is 0
/// - `InsufficientData` if data length is insufficient for the MA type
/// - `UnsupportedOperation` if MA type is not yet implemented
///
/// # Example
/// ```rust
/// use ta_rust::overlap::ma;
/// use ta_rust::common::MAType;
///
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// 
/// // Calculate SMA
/// let sma_result = ma(&prices, 3, MAType::SMA).unwrap();
/// 
/// // Calculate EMA
/// let ema_result = ma(&prices, 3, MAType::EMA).unwrap();
/// 
/// // Calculate WMA
/// let wma_result = ma(&prices, 3, MAType::WMA).unwrap();
/// ```
pub fn ma(data: &[Price], period: Period, ma_type: MAType) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    
    // Check minimum data requirements based on MA type
    let min_required = match ma_type {
        MAType::SMA | MAType::EMA | MAType::WMA | MAType::TRIMA => period,
        MAType::DEMA => 2 * period - 1,
        MAType::TEMA => 3 * period - 2,
        MAType::KAMA => period + 1, // KAMA needs extra data for efficiency ratio
        MAType::MAMA => 32, // MAMA requires significant data for Hilbert Transform
        MAType::T3 => 6 * period, // T3 requires multiple EMA calculations
    };
    
    validate_sufficient_data(data, min_required, "data")?;

    // Dispatch to appropriate MA function
    match ma_type {
        MAType::SMA => sma::sma(data, period),
        MAType::EMA => ema::ema(data, period),
        MAType::WMA => wma::wma(data, period),
        MAType::DEMA => dema::dema(data, period),
        MAType::TEMA => tema::tema(data, period),
        MAType::TRIMA => trima::trima(data, period),
        MAType::KAMA => {
            // KAMA will be implemented in a later phase
            Err(crate::common::TAError::unsupported_operation(
                "KAMA not yet implemented - will be available in Phase 5"
            ))
        },
        MAType::MAMA => {
            // MAMA will be implemented in Phase 6 (Hilbert Transform)
            Err(crate::common::TAError::unsupported_operation(
                "MAMA not yet implemented - will be available in Phase 6"
            ))
        },
        MAType::T3 => {
            // T3 will be implemented in Phase 5
            Err(crate::common::TAError::unsupported_operation(
                "T3 not yet implemented - will be available in Phase 5"
            ))
        },
    }
}

/// Calculates multiple moving averages at once
///
/// This function calculates several different types of moving averages
/// for the same data and period, which can be useful for comparison.
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods for the moving averages
/// - `ma_types`: Slice of MA types to calculate
///
/// # Returns
/// Vector of tuples containing (MAType, Result<Vec<Price>>)
///
/// # Example
/// ```rust
/// use ta_rust::overlap::ma_multiple;
/// use ta_rust::common::MAType;
///
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let ma_types = vec![MAType::SMA, MAType::EMA, MAType::WMA];
/// let results = ma_multiple(&prices, 5, &ma_types);
/// 
/// for (ma_type, result) in results {
///     match result {
///         Ok(values) => println!("{}: {:?}", ma_type, values),
///         Err(e) => println!("{}: Error - {}", ma_type, e),
///     }
/// }
/// ```
pub fn ma_multiple(
    data: &[Price], 
    period: Period, 
    ma_types: &[MAType]
) -> Vec<(MAType, TAResult<Vec<Price>>)> {
    ma_types.iter()
        .map(|&ma_type| (ma_type, ma(data, period, ma_type)))
        .collect()
}

/// Calculates a moving average with automatic type selection based on data characteristics
///
/// This function analyzes the input data and selects the most appropriate
/// moving average type based on volatility and trend characteristics.
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods for the moving average
///
/// # Returns
/// Tuple containing (selected MAType, calculated values)
///
/// # Selection Logic
/// - High volatility data: WMA (more responsive)
/// - Trending data: EMA (good trend following)
/// - Stable data: SMA (smooth)
/// - Very noisy data: TRIMA (maximum smoothing)
pub fn ma_auto(data: &[Price], period: Period) -> TAResult<(MAType, Vec<Price>)> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period * 2, "data")?; // Need extra data for analysis
    
    // Analyze data characteristics
    let volatility = calculate_volatility(data);
    let trend_strength = calculate_trend_strength(data);
    
    // Select MA type based on characteristics
    let selected_type = if volatility > 0.05 && trend_strength > 0.3 {
        MAType::WMA // High volatility + trending: use WMA
    } else if trend_strength > 0.2 {
        MAType::EMA // Trending: use EMA
    } else if volatility < 0.02 {
        MAType::SMA // Low volatility: use SMA
    } else {
        MAType::TRIMA // Noisy data: use TRIMA for smoothing
    };
    
    let result = ma(data, period, selected_type)?;
    Ok((selected_type, result))
}

/// Calculates data volatility for MA type selection
fn calculate_volatility(data: &[Price]) -> Price {
    if data.len() < 2 {
        return 0.0;
    }
    
    let returns: Vec<Price> = data.windows(2)
        .map(|w| (w[1] - w[0]) / w[0])
        .collect();
    
    crate::common::utils::std_dev(&returns)
}

/// Calculates trend strength for MA type selection
fn calculate_trend_strength(data: &[Price]) -> Price {
    if data.len() < 3 {
        return 0.0;
    }
    
    let first_third = data.len() / 3;
    let last_third = data.len() - first_third;
    
    let early_avg = crate::common::utils::mean(&data[0..first_third]);
    let late_avg = crate::common::utils::mean(&data[last_third..]);
    
    if early_avg == 0.0 {
        return 0.0;
    }
    
    ((late_avg - early_avg) / early_avg).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};

    #[test]
    fn test_ma_sma() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = ma(&data, 3, MAType::SMA).unwrap();
        let expected = sma::sma(&data, 3).unwrap();
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_ma_ema() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = ma(&data, 3, MAType::EMA).unwrap();
        let expected = ema::ema(&data, 3).unwrap();
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_ma_wma() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = ma(&data, 3, MAType::WMA).unwrap();
        let expected = wma::wma(&data, 3).unwrap();
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_ma_dema() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let result = ma(&data, 3, MAType::DEMA).unwrap();
        let expected = dema::dema(&data, 3).unwrap();
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_ma_tema() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = ma(&data, 3, MAType::TEMA).unwrap();
        let expected = tema::tema(&data, 3).unwrap();
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_ma_trima() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let result = ma(&data, 5, MAType::TRIMA).unwrap();
        let expected = trima::trima(&data, 5).unwrap();
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_ma_unsupported_types() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
        // These should return errors as they're not implemented yet
        assert!(ma(&data, 3, MAType::KAMA).is_err());
        assert!(ma(&data, 3, MAType::MAMA).is_err());
        assert!(ma(&data, 3, MAType::T3).is_err());
    }

    #[test]
    fn test_ma_multiple() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let ma_types = vec![MAType::SMA, MAType::EMA, MAType::WMA];
        let results = ma_multiple(&data, 3, &ma_types);
        
        assert_eq!(results.len(), 3);
        
        for (ma_type, result) in results {
            match ma_type {
                MAType::SMA | MAType::EMA | MAType::WMA => {
                    assert!(result.is_ok());
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_ma_auto_selection() {
        // Test with trending data
        let trending_data: Vec<Price> = (1..=20).map(|x| x as Price).collect();
        let (selected_type, _) = ma_auto(&trending_data, 5).unwrap();
        
        // Should select EMA or WMA for trending data
        assert!(matches!(selected_type, MAType::EMA | MAType::WMA));
        
        // Test with stable data
        let stable_data = vec![10.0; 20];
        let (selected_type, _) = ma_auto(&stable_data, 5).unwrap();
        
        // Should select SMA for stable data
        assert_eq!(selected_type, MAType::SMA);
    }

    #[test]
    fn test_volatility_calculation() {
        // Low volatility data
        let stable_data = vec![10.0, 10.1, 9.9, 10.05, 9.95];
        let volatility = calculate_volatility(&stable_data);
        assert!(volatility < 0.1);
        
        // High volatility data
        let volatile_data = vec![10.0, 15.0, 8.0, 12.0, 6.0];
        let volatility = calculate_volatility(&volatile_data);
        assert!(volatility > 0.1);
    }

    #[test]
    fn test_trend_strength_calculation() {
        // Strong uptrend
        let uptrend_data: Vec<Price> = (1..=20).map(|x| x as Price).collect();
        let trend_strength = calculate_trend_strength(&uptrend_data);
        assert!(trend_strength > 0.5);
        
        // No trend (flat)
        let flat_data = vec![10.0; 20];
        let trend_strength = calculate_trend_strength(&flat_data);
        assert!(trend_strength < 0.1);
    }

    #[test]
    fn test_ma_insufficient_data() {
        let data = vec![1.0, 2.0];
        assert!(ma(&data, 5, MAType::SMA).is_err());
        assert!(ma(&data, 3, MAType::DEMA).is_err()); // Needs 2*3-1 = 5 points
    }

    #[test]
    fn test_ma_empty_data() {
        let data = vec![];
        assert!(ma(&data, 3, MAType::SMA).is_err());
    }

    #[test]
    fn test_ma_zero_period() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(ma(&data, 0, MAType::SMA).is_err());
    }

    #[test]
    fn test_ma_type_display() {
        // Test that MAType implements Display (from types.rs)
        assert_eq!(MAType::SMA.to_string(), "SMA");
        assert_eq!(MAType::EMA.to_string(), "EMA");
        assert_eq!(MAType::WMA.to_string(), "WMA");
    }

    #[test]
    fn test_ma_all_types() {
        // Test that all MA types are covered
        let all_types = MAType::all();
        assert_eq!(all_types.len(), 9);
        assert!(all_types.contains(&MAType::SMA));
        assert!(all_types.contains(&MAType::EMA));
        assert!(all_types.contains(&MAType::WMA));
        assert!(all_types.contains(&MAType::DEMA));
        assert!(all_types.contains(&MAType::TEMA));
        assert!(all_types.contains(&MAType::TRIMA));
        assert!(all_types.contains(&MAType::KAMA));
        assert!(all_types.contains(&MAType::MAMA));
        assert!(all_types.contains(&MAType::T3));
    }
}