//! Constants used throughout TA-Rust

/// Pattern recognition output values
pub mod pattern {
    /// Bullish pattern detected
    pub const BULLISH: i32 = 100;
    
    /// Bearish pattern detected
    pub const BEARISH: i32 = -100;
    
    /// No pattern detected
    pub const NONE: i32 = 0;
}

/// Default parameters for various indicators
pub mod defaults {
    use crate::common::Period;

    /// Default period for most indicators
    pub const PERIOD: Period = 14;
    
    /// Default fast period for MACD
    pub const MACD_FAST: Period = 12;
    
    /// Default slow period for MACD
    pub const MACD_SLOW: Period = 26;
    
    /// Default signal period for MACD
    pub const MACD_SIGNAL: Period = 9;
    
    /// Default period for Bollinger Bands
    pub const BBANDS_PERIOD: Period = 20;
    
    /// Default standard deviation multiplier for Bollinger Bands
    pub const BBANDS_STDDEV: f64 = 2.0;
    
    /// Default acceleration factor for Parabolic SAR
    pub const SAR_AF: f64 = 0.02;
    
    /// Default maximum acceleration factor for Parabolic SAR
    pub const SAR_MAX_AF: f64 = 0.20;
    
    /// Default fast limit for MAMA
    pub const MAMA_FAST_LIMIT: f64 = 0.5;
    
    /// Default slow limit for MAMA
    pub const MAMA_SLOW_LIMIT: f64 = 0.05;
    
    /// Default volume factor for T3
    pub const T3_VOLUME_FACTOR: f64 = 0.7;
    
    /// Default period 1 for Ultimate Oscillator
    pub const ULTOSC_PERIOD1: Period = 7;
    
    /// Default period 2 for Ultimate Oscillator
    pub const ULTOSC_PERIOD2: Period = 14;
    
    /// Default period 3 for Ultimate Oscillator
    pub const ULTOSC_PERIOD3: Period = 28;
    
    /// Default fast period for Stochastic
    pub const STOCH_FASTK: Period = 5;
    
    /// Default slow K period for Stochastic
    pub const STOCH_SLOWK: Period = 3;
    
    /// Default slow D period for Stochastic
    pub const STOCH_SLOWD: Period = 3;
}

/// Mathematical constants
pub mod math {
    use crate::common::Price;

    /// Pi constant
    pub const PI: Price = core::f64::consts::PI;
    
    /// 2 * Pi
    pub const TWO_PI: Price = 2.0 * PI;
    
    /// Pi / 2
    pub const PI_2: Price = PI / 2.0;
    
    /// Pi / 4
    pub const PI_4: Price = PI / 4.0;
    
    /// Square root of 2
    pub const SQRT_2: Price = core::f64::consts::SQRT_2;
    
    /// Natural logarithm of 2
    pub const LN_2: Price = core::f64::consts::LN_2;
    
    /// Natural logarithm of 10
    pub const LN_10: Price = core::f64::consts::LN_10;
    
    /// Euler's number
    pub const E: Price = core::f64::consts::E;
}

/// Thresholds and tolerances
pub mod thresholds {
    use crate::common::Price;

    /// Default tolerance for floating point comparisons
    pub const EPSILON: Price = 1e-10;
    
    /// Tolerance for doji detection (body size relative to range)
    pub const DOJI_THRESHOLD: Price = 0.1;
    
    /// Threshold for long candle detection (body size relative to average)
    pub const LONG_CANDLE_THRESHOLD: Price = 1.5;
    
    /// Threshold for short candle detection (body size relative to average)
    pub const SHORT_CANDLE_THRESHOLD: Price = 0.5;
    
    /// Minimum shadow to body ratio for hammer/hanging man
    pub const HAMMER_SHADOW_RATIO: Price = 2.0;
    
    /// Maximum upper shadow for hammer (relative to body)
    pub const HAMMER_UPPER_SHADOW_MAX: Price = 0.1;
    
    /// Minimum engulfing ratio (how much the second candle engulfs the first)
    pub const ENGULFING_RATIO: Price = 1.0;
    
    /// CCI constant (0.015)
    pub const CCI_CONSTANT: Price = 0.015;
    
    /// Minimum correlation coefficient for valid correlation
    pub const MIN_CORRELATION: Price = -1.0;
    
    /// Maximum correlation coefficient for valid correlation
    pub const MAX_CORRELATION: Price = 1.0;
}

/// Unstable periods for various indicators
/// These represent the number of periods needed for an indicator to stabilize
pub mod unstable_periods {
    use crate::common::Period;

    /// EMA unstable period calculation: 2 * period - 1
    pub fn ema(period: Period) -> Period {
        2 * period - 1
    }
    
    /// RSI unstable period (using Wilder's smoothing)
    pub fn rsi(period: Period) -> Period {
        period + 100 // Conservative estimate for Wilder's smoothing
    }
    
    /// ATR unstable period (using Wilder's smoothing)
    pub fn atr(period: Period) -> Period {
        period + 100 // Conservative estimate for Wilder's smoothing
    }
    
    /// ADX unstable period
    pub fn adx(period: Period) -> Period {
        2 * period + 100 // DX smoothing + ADX smoothing
    }
    
    /// MACD unstable period
    pub fn macd(slow_period: Period, signal_period: Period) -> Period {
        slow_period + signal_period - 1
    }
    
    /// Stochastic unstable period
    pub fn stochastic(fastk_period: Period, slowk_period: Period, slowd_period: Period) -> Period {
        fastk_period + slowk_period + slowd_period - 2
    }
    
    /// Bollinger Bands unstable period
    pub fn bbands(period: Period) -> Period {
        period - 1
    }
    
    /// KAMA unstable period
    pub fn kama(period: Period) -> Period {
        period + 32 // Conservative estimate for adaptive period
    }
    
    /// Hilbert Transform unstable period
    pub const HILBERT_TRANSFORM: Period = 63;
    
    /// MAMA unstable period
    pub const MAMA: Period = 32;
    
    /// T3 unstable period
    pub fn t3(period: Period) -> Period {
        6 * period // Six EMA calculations
    }
}

/// Validation limits
pub mod limits {
    use crate::common::{Price, Period};

    /// Maximum reasonable period for most indicators
    pub const MAX_PERIOD: Period = 100000;
    
    /// Minimum period for most indicators
    pub const MIN_PERIOD: Period = 1;
    
    /// Maximum price value (to prevent overflow)
    pub const MAX_PRICE: Price = 1e15;
    
    /// Minimum price value (to prevent underflow)
    pub const MIN_PRICE: Price = -1e15;
    
    /// Maximum volume value
    pub const MAX_VOLUME: Price = 1e15;
    
    /// Minimum volume value
    pub const MIN_VOLUME: Price = 0.0;
    
    /// Maximum standard deviation multiplier for Bollinger Bands
    pub const MAX_BBANDS_STDDEV: Price = 10.0;
    
    /// Minimum standard deviation multiplier for Bollinger Bands
    pub const MIN_BBANDS_STDDEV: Price = 0.1;
    
    /// Maximum acceleration factor for Parabolic SAR
    pub const MAX_SAR_AF: Price = 1.0;
    
    /// Minimum acceleration factor for Parabolic SAR
    pub const MIN_SAR_AF: Price = 0.001;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_constants() {
        assert_eq!(pattern::BULLISH, 100);
        assert_eq!(pattern::BEARISH, -100);
        assert_eq!(pattern::NONE, 0);
    }

    #[test]
    fn test_default_constants() {
        assert_eq!(defaults::PERIOD, 14);
        assert_eq!(defaults::MACD_FAST, 12);
        assert_eq!(defaults::MACD_SLOW, 26);
        assert_eq!(defaults::MACD_SIGNAL, 9);
    }

    #[test]
    fn test_math_constants() {
        use approx::assert_relative_eq;
        
        assert_relative_eq!(math::PI, std::f64::consts::PI, epsilon = 1e-15);
        assert_relative_eq!(math::TWO_PI, 2.0 * std::f64::consts::PI, epsilon = 1e-15);
        assert_relative_eq!(math::E, std::f64::consts::E, epsilon = 1e-15);
    }

    #[test]
    fn test_unstable_periods() {
        assert_eq!(unstable_periods::ema(14), 27);
        assert_eq!(unstable_periods::macd(26, 9), 34);
        assert_eq!(unstable_periods::HILBERT_TRANSFORM, 63);
    }

    #[test]
    fn test_limits() {
        assert!(limits::MAX_PERIOD > limits::MIN_PERIOD);
        assert!(limits::MAX_PRICE > limits::MIN_PRICE);
        assert!(limits::MAX_VOLUME >= limits::MIN_VOLUME);
    }
}