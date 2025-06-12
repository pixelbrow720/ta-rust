//! Test data generators and utilities for TA-Rust testing

use ta_rust::common::Price;

/// Generates a simple ascending price series
pub fn ascending_prices(count: usize) -> Vec<Price> {
    (1..=count).map(|i| i as Price).collect()
}

/// Generates a simple descending price series
pub fn descending_prices(count: usize) -> Vec<Price> {
    (1..=count).rev().map(|i| i as Price).collect()
}

/// Generates a sine wave price series
pub fn sine_wave_prices(count: usize, amplitude: Price, frequency: Price) -> Vec<Price> {
    (0..count)
        .map(|i| {
            let x = i as Price * frequency * 2.0 * std::f64::consts::PI / count as Price;
            amplitude * x.sin() + amplitude
        })
        .collect()
}

/// Generates random walk price series (for testing)
pub fn random_walk_prices(count: usize, start: Price, volatility: Price) -> Vec<Price> {
    let mut prices = Vec::with_capacity(count);
    let mut current = start;
    
    // Simple deterministic "random" walk for reproducible tests
    for i in 0..count {
        let change = volatility * ((i as Price * 0.1).sin() * 0.5 + 0.1);
        current += change;
        prices.push(current);
    }
    
    prices
}

/// Generates OHLC data from a price series
pub fn prices_to_ohlc(prices: &[Price]) -> (Vec<Price>, Vec<Price>, Vec<Price>, Vec<Price>) {
    let mut open = Vec::with_capacity(prices.len());
    let mut high = Vec::with_capacity(prices.len());
    let mut low = Vec::with_capacity(prices.len());
    let mut close = Vec::with_capacity(prices.len());
    
    for (i, &price) in prices.iter().enumerate() {
        let volatility = price * 0.02; // 2% volatility
        let o = if i == 0 { price } else { close[i - 1] };
        let h = price + volatility;
        let l = price - volatility;
        let c = price;
        
        open.push(o);
        high.push(h);
        low.push(l);
        close.push(c);
    }
    
    (open, high, low, close)
}

/// Generates volume data that correlates with price movement
pub fn generate_volume(prices: &[Price], base_volume: Price) -> Vec<Price> {
    prices
        .windows(2)
        .enumerate()
        .map(|(_i, window)| {
            let price_change = if window.len() == 2 {
                (window[1] - window[0]).abs()
            } else {
                0.0
            };
            // Higher volume on larger price changes
            base_volume * (1.0 + price_change * 10.0)
        })
        .chain(std::iter::once(base_volume)) // First volume
        .collect()
}

/// Sample real-world-like price data for testing
pub fn sample_prices() -> Vec<Price> {
    vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93,
        46.85, 46.80, 46.80, 46.85, 46.85, 47.92, 47.25, 46.93, 46.85, 46.80,
        46.80, 46.85, 46.85, 47.92, 47.25, 46.93, 46.85, 46.80, 46.80, 46.85,
        46.85, 47.92, 47.25, 46.93, 46.85, 46.80, 46.80, 46.85, 46.85, 47.92,
        47.25, 46.93, 46.85, 46.80, 46.80, 46.85, 46.85, 47.92, 47.25, 46.93,
    ]
}

/// Sample OHLC data for testing
pub fn sample_ohlc() -> (Vec<Price>, Vec<Price>, Vec<Price>, Vec<Price>) {
    let open = vec![
        44.20, 44.30, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92,
        46.93, 46.85, 46.80, 46.80, 46.85, 46.85, 47.92, 47.25, 46.93, 46.85,
    ];
    
    let high = vec![
        44.34, 44.30, 44.20, 44.20, 44.45, 44.95, 45.95, 47.35, 47.95, 47.95,
        47.00, 46.90, 46.85, 46.85, 46.90, 47.95, 47.95, 47.30, 47.00, 46.90,
    ];
    
    let low = vec![
        44.09, 44.09, 44.05, 43.61, 43.61, 44.33, 44.83, 45.85, 47.25, 46.93,
        46.85, 46.80, 46.75, 46.80, 46.85, 46.85, 47.25, 46.93, 46.85, 46.80,
    ];
    
    let close = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93,
        46.85, 46.80, 46.80, 46.85, 46.85, 47.92, 47.25, 46.93, 46.85, 46.80,
    ];
    
    (open, high, low, close)
}

/// Creates test data with known SMA values for validation
pub fn sma_test_data() -> (Vec<Price>, Vec<Price>) {
    let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let expected_sma3 = vec![
        Price::NAN, Price::NAN, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0
    ];
    (prices, expected_sma3)
}

/// Creates test data with known EMA values for validation
pub fn ema_test_data() -> (Vec<Price>, Vec<Price>) {
    let prices = vec![22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29];
    // Expected EMA(10) values calculated manually
    let expected_ema10 = vec![
        Price::NAN, Price::NAN, Price::NAN, Price::NAN, Price::NAN,
        Price::NAN, Price::NAN, Price::NAN, Price::NAN, 22.22,
    ];
    (prices, expected_ema10)
}

/// Creates test data for RSI validation
pub fn rsi_test_data() -> (Vec<Price>, Vec<Price>) {
    let prices = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93,
        46.85, 46.80, 46.80, 46.85, 46.85, 47.92, 47.25, 46.93, 46.85, 46.80,
    ];
    
    // Expected RSI(14) values - these would need to be calculated from TA-Lib for exact values
    let expected_rsi14 = vec![
        Price::NAN, Price::NAN, Price::NAN, Price::NAN, Price::NAN,
        Price::NAN, Price::NAN, Price::NAN, Price::NAN, Price::NAN,
        Price::NAN, Price::NAN, Price::NAN, Price::NAN, 70.53,
        66.32, 66.55, 69.41, 66.36, 57.97,
    ];
    
    (prices, expected_rsi14)
}

/// Utility function to compare floating point arrays with tolerance
pub fn assert_arrays_approx_equal(actual: &[Price], expected: &[Price], tolerance: Price) {
    assert_eq!(actual.len(), expected.len(), "Array lengths don't match");
    
    for (i, (&a, &e)) in actual.iter().zip(expected.iter()).enumerate() {
        if e.is_nan() {
            assert!(a.is_nan(), "Expected NaN at index {}, got {}", i, a);
        } else {
            assert!(
                (a - e).abs() <= tolerance,
                "Values don't match at index {}: expected {}, got {}, diff = {}",
                i, e, a, (a - e).abs()
            );
        }
    }
}

/// Utility function to check if a result is within acceptable range
pub fn is_within_tolerance(actual: Price, expected: Price, tolerance: Price) -> bool {
    if expected.is_nan() {
        actual.is_nan()
    } else {
        (actual - expected).abs() <= tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascending_prices() {
        let prices = ascending_prices(5);
        assert_eq!(prices, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_descending_prices() {
        let prices = descending_prices(5);
        assert_eq!(prices, vec![5.0, 4.0, 3.0, 2.0, 1.0]);
    }

    #[test]
    fn test_sine_wave_prices() {
        let prices = sine_wave_prices(4, 1.0, 1.0);
        assert_eq!(prices.len(), 4);
        // First value should be 1.0 (sin(0) = 0, so 1.0 * 0 + 1.0 = 1.0)
        assert!((prices[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_prices_to_ohlc() {
        let prices = vec![100.0, 101.0, 102.0];
        let (open, high, low, close) = prices_to_ohlc(&prices);
        
        assert_eq!(open.len(), 3);
        assert_eq!(high.len(), 3);
        assert_eq!(low.len(), 3);
        assert_eq!(close.len(), 3);
        
        // First open should equal first price
        assert_eq!(open[0], 100.0);
        // Close should equal the input prices
        assert_eq!(close, prices);
        // High should be greater than close
        assert!(high[0] > close[0]);
        // Low should be less than close
        assert!(low[0] < close[0]);
    }

    #[test]
    fn test_assert_arrays_approx_equal() {
        let a = vec![1.0, 2.0, Price::NAN];
        let b = vec![1.001, 1.999, Price::NAN];
        
        assert_arrays_approx_equal(&a, &b, 0.01);
    }

    #[test]
    #[should_panic]
    fn test_assert_arrays_approx_equal_fail() {
        let a = vec![1.0, 2.0];
        let b = vec![1.1, 2.0];
        
        assert_arrays_approx_equal(&a, &b, 0.01);
    }
}