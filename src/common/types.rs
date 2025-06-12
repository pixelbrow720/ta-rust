//! Core type definitions for TA-Rust

/// Price type used throughout the library
/// 
/// Uses f64 for maximum precision in financial calculations
pub type Price = f64;

/// Volume type for trading volume data
pub type Volume = f64;

/// Period type for time periods in indicators
pub type Period = usize;

/// Moving Average types supported by the library
/// 
/// These correspond to the MA types available in TA-Lib
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MAType {
    /// Simple Moving Average
    SMA = 0,
    /// Exponential Moving Average
    EMA = 1,
    /// Weighted Moving Average
    WMA = 2,
    /// Double Exponential Moving Average
    DEMA = 3,
    /// Triple Exponential Moving Average
    TEMA = 4,
    /// Triangular Moving Average
    TRIMA = 5,
    /// Kaufman Adaptive Moving Average
    KAMA = 6,
    /// MESA Adaptive Moving Average
    MAMA = 7,
    /// Triple Exponential Moving Average (T3)
    T3 = 8,
}

impl MAType {
    /// Returns the default parameters for each MA type
    pub fn default_period(self) -> Period {
        match self {
            MAType::SMA => 14,
            MAType::EMA => 14,
            MAType::WMA => 14,
            MAType::DEMA => 14,
            MAType::TEMA => 14,
            MAType::TRIMA => 14,
            MAType::KAMA => 14,
            MAType::MAMA => 14,
            MAType::T3 => 14,
        }
    }

    /// Returns the minimum required data points for the MA type
    pub fn min_period(self) -> Period {
        match self {
            MAType::SMA => 1,
            MAType::EMA => 1,
            MAType::WMA => 1,
            MAType::DEMA => 2,
            MAType::TEMA => 3,
            MAType::TRIMA => 1,
            MAType::KAMA => 2,
            MAType::MAMA => 32, // Requires significant data for Hilbert Transform
            MAType::T3 => 6,
        }
    }

    /// Returns all available MA types
    pub fn all() -> &'static [MAType] {
        &[
            MAType::SMA,
            MAType::EMA,
            MAType::WMA,
            MAType::DEMA,
            MAType::TEMA,
            MAType::TRIMA,
            MAType::KAMA,
            MAType::MAMA,
            MAType::T3,
        ]
    }
}

impl Default for MAType {
    fn default() -> Self {
        MAType::SMA
    }
}

impl core::fmt::Display for MAType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            MAType::SMA => "SMA",
            MAType::EMA => "EMA",
            MAType::WMA => "WMA",
            MAType::DEMA => "DEMA",
            MAType::TEMA => "TEMA",
            MAType::TRIMA => "TRIMA",
            MAType::KAMA => "KAMA",
            MAType::MAMA => "MAMA",
            MAType::T3 => "T3",
        };
        write!(f, "{}", name)
    }
}

/// Candlestick data structure
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OHLC {
    /// Opening price
    pub open: Price,
    /// Highest price
    pub high: Price,
    /// Lowest price
    pub low: Price,
    /// Closing price
    pub close: Price,
}

impl OHLC {
    /// Creates a new OHLC instance
    pub fn new(open: Price, high: Price, low: Price, close: Price) -> Self {
        Self { open, high, low, close }
    }

    /// Returns the typical price (HLC/3)
    pub fn typical_price(self) -> Price {
        (self.high + self.low + self.close) / 3.0
    }

    /// Returns the median price (HL/2)
    pub fn median_price(self) -> Price {
        (self.high + self.low) / 2.0
    }

    /// Returns the weighted close price (HLCC/4)
    pub fn weighted_close_price(self) -> Price {
        (self.high + self.low + 2.0 * self.close) / 4.0
    }

    /// Returns the average price (OHLC/4)
    pub fn average_price(self) -> Price {
        (self.open + self.high + self.low + self.close) / 4.0
    }

    /// Returns the true range
    pub fn true_range(self, prev_close: Option<Price>) -> Price {
        let hl = self.high - self.low;
        match prev_close {
            Some(pc) => {
                let hc = (self.high - pc).abs();
                let lc = (self.low - pc).abs();
                hl.max(hc).max(lc)
            }
            None => hl,
        }
    }

    /// Returns the body size (absolute difference between open and close)
    pub fn body_size(self) -> Price {
        (self.close - self.open).abs()
    }

    /// Returns the upper shadow size
    pub fn upper_shadow(self) -> Price {
        self.high - self.open.max(self.close)
    }

    /// Returns the lower shadow size
    pub fn lower_shadow(self) -> Price {
        self.open.min(self.close) - self.low
    }

    /// Returns true if this is a bullish candle (close > open)
    pub fn is_bullish(self) -> bool {
        self.close > self.open
    }

    /// Returns true if this is a bearish candle (close < open)
    pub fn is_bearish(self) -> bool {
        self.close < self.open
    }

    /// Returns true if this is a doji (open ≈ close)
    pub fn is_doji(self, threshold: Price) -> bool {
        (self.close - self.open).abs() <= threshold
    }
}

/// Candlestick data with volume
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OHLCV {
    /// OHLC data
    pub ohlc: OHLC,
    /// Trading volume
    pub volume: Volume,
}

impl OHLCV {
    /// Creates a new OHLCV instance
    pub fn new(open: Price, high: Price, low: Price, close: Price, volume: Volume) -> Self {
        Self {
            ohlc: OHLC::new(open, high, low, close),
            volume,
        }
    }

    /// Returns the money flow (typical price * volume)
    pub fn money_flow(self) -> Price {
        self.ohlc.typical_price() * self.volume
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ma_type_display() {
        assert_eq!(MAType::SMA.to_string(), "SMA");
        assert_eq!(MAType::EMA.to_string(), "EMA");
        assert_eq!(MAType::WMA.to_string(), "WMA");
    }

    #[test]
    fn test_ma_type_default_period() {
        assert_eq!(MAType::SMA.default_period(), 14);
        assert_eq!(MAType::EMA.default_period(), 14);
    }

    #[test]
    fn test_ma_type_min_period() {
        assert_eq!(MAType::SMA.min_period(), 1);
        assert_eq!(MAType::DEMA.min_period(), 2);
        assert_eq!(MAType::TEMA.min_period(), 3);
    }

    #[test]
    fn test_ohlc_calculations() {
        let ohlc = OHLC::new(10.0, 12.0, 9.0, 11.0);
        
        assert_eq!(ohlc.typical_price(), (12.0 + 9.0 + 11.0) / 3.0);
        assert_eq!(ohlc.median_price(), (12.0 + 9.0) / 2.0);
        assert_eq!(ohlc.weighted_close_price(), (12.0 + 9.0 + 2.0 * 11.0) / 4.0);
        assert_eq!(ohlc.average_price(), (10.0 + 12.0 + 9.0 + 11.0) / 4.0);
        
        assert_eq!(ohlc.body_size(), 1.0);
        assert_eq!(ohlc.upper_shadow(), 1.0); // 12.0 - 11.0
        assert_eq!(ohlc.lower_shadow(), 1.0); // 10.0 - 9.0
        
        assert!(ohlc.is_bullish());
        assert!(!ohlc.is_bearish());
        assert!(!ohlc.is_doji(0.1));
    }

    #[test]
    fn test_true_range() {
        let ohlc = OHLC::new(10.0, 12.0, 9.0, 11.0);
        
        // Without previous close
        assert_eq!(ohlc.true_range(None), 3.0); // 12.0 - 9.0
        
        // With previous close
        assert_eq!(ohlc.true_range(Some(8.0)), 4.0); // max(3.0, 4.0, 1.0)
        assert_eq!(ohlc.true_range(Some(13.0)), 4.0); // max(3.0, 1.0, 4.0)
    }
}