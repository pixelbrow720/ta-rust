# Phase 5: Volume Indicators & Advanced Overlays

## Overview

Phase 5 implements volume indicators, advanced overlap studies, and statistical functions. This phase adds sophisticated analytical capabilities including Bollinger Bands, Parabolic SAR, adaptive moving averages, and comprehensive statistical analysis functions.

## Implemented Functions

### Volume Indicators (3 functions)

#### OBV - On Balance Volume ⭐
- **Purpose**: Momentum indicator using volume flow to predict price changes
- **Formula**: Adds volume on up days, subtracts on down days
- **Usage**: Trend confirmation and divergence analysis
- **Implementation**: `ta_rust::volume::obv`

#### AD - Chaikin A/D Line
- **Purpose**: Measures cumulative flow of money into/out of security
- **Formula**: Uses Close Location Value (CLV) with volume
- **Usage**: Accumulation/distribution analysis
- **Implementation**: `ta_rust::volume::ad`

#### ADOSC - Chaikin A/D Oscillator
- **Purpose**: Momentum of A/D Line using fast/slow EMA difference
- **Formula**: EMA(AD, fast) - EMA(AD, slow)
- **Usage**: A/D Line momentum and signal generation
- **Implementation**: `ta_rust::volume::adosc`

### Advanced Overlap Studies (8 functions)

#### BBANDS - Bollinger Bands ⭐
- **Purpose**: Volatility bands around moving average
- **Formula**: Middle ± (StdDev × Multiplier)
- **Usage**: Volatility measurement, overbought/oversold conditions
- **Implementation**: `ta_rust::overlap::bbands`
- **Features**: 
  - %B calculation
  - Band width measurement
  - Configurable periods and multipliers

#### SAR - Parabolic SAR ⭐
- **Purpose**: Trend-following indicator providing reversal points
- **Formula**: SAR = SAR_prev + AF × (EP - SAR_prev)
- **Usage**: Stop-loss levels, trend direction
- **Implementation**: `ta_rust::overlap::sar`
- **Features**:
  - Trend direction detection
  - Configurable acceleration factors

#### SAREXT - Parabolic SAR Extended
- **Purpose**: Enhanced SAR with additional parameters
- **Features**:
  - Custom start values
  - Offset on reverse
  - Different AF for long/short positions
- **Implementation**: `ta_rust::overlap::sarext`

#### KAMA - Kaufman Adaptive Moving Average
- **Purpose**: Adaptive MA based on market efficiency
- **Formula**: Uses Efficiency Ratio to adjust smoothing
- **Usage**: Trend following with noise reduction
- **Implementation**: `ta_rust::overlap::kama`
- **Features**:
  - Efficiency ratio calculation
  - Adaptive smoothing

#### T3 - Triple Exponential Moving Average
- **Purpose**: Smooth MA with reduced lag
- **Formula**: Uses 6 EMAs with volume factor weighting
- **Usage**: Trend following with minimal lag
- **Implementation**: `ta_rust::overlap::t3`

#### MAMA - MESA Adaptive Moving Average
- **Purpose**: Hilbert Transform-based adaptive MA
- **Features**:
  - MAMA and FAMA outputs
  - Dominant cycle detection
  - Adaptive period calculation
- **Implementation**: `ta_rust::overlap::mama`

#### MAVP - Moving Average with Variable Period
- **Purpose**: MA where period varies per data point
- **Features**:
  - Period constraints (min/max)
  - Adaptive period based on indicators
  - Volatility-based adaptation
- **Implementation**: `ta_rust::overlap::mavp`

#### TRIX - 1-day Rate-Of-Change of Triple Smooth EMA
- **Purpose**: Momentum oscillator with triple smoothing
- **Formula**: ROC of triple-smoothed EMA
- **Usage**: Trend filtering, momentum analysis
- **Implementation**: `ta_rust::overlap::trix`
- **Features**:
  - Signal line calculation
  - Histogram (TRIX - Signal)

### Statistic Functions (9 functions)

#### BETA - Beta Coefficient
- **Purpose**: Measures systematic risk vs market
- **Formula**: Covariance(Security, Market) / Variance(Market)
- **Usage**: Risk analysis, portfolio management
- **Implementation**: `ta_rust::statistic::beta`

#### CORREL - Pearson's Correlation Coefficient
- **Purpose**: Linear relationship strength between series
- **Formula**: Normalized covariance
- **Range**: -1 to +1
- **Implementation**: `ta_rust::statistic::correl`
- **Features**:
  - Correlation matrix calculation
  - Strength categorization

#### LINEARREG - Linear Regression
- **Purpose**: Fits straight line to data points
- **Usage**: Trend analysis, forecasting
- **Implementation**: `ta_rust::statistic::linearreg`

#### LINEARREG_ANGLE - Linear Regression Angle
- **Purpose**: Angle of regression line in degrees
- **Usage**: Trend strength measurement
- **Implementation**: `ta_rust::statistic::linearreg_angle`

#### LINEARREG_INTERCEPT - Linear Regression Intercept
- **Purpose**: Y-intercept of regression line
- **Usage**: Regression analysis
- **Implementation**: `ta_rust::statistic::linearreg_intercept`

#### LINEARREG_SLOPE - Linear Regression Slope
- **Purpose**: Slope of regression line
- **Usage**: Trend rate measurement
- **Implementation**: `ta_rust::statistic::linearreg_slope`

#### STDDEV - Standard Deviation
- **Purpose**: Measures data dispersion
- **Features**:
  - Population and sample variants
  - Coefficient of variation
  - Z-score calculation
- **Implementation**: `ta_rust::statistic::stddev`

#### TSF - Time Series Forecast
- **Purpose**: Linear regression forecast one period ahead
- **Usage**: Price prediction
- **Implementation**: `ta_rust::statistic::tsf`

#### VAR - Variance
- **Purpose**: Average squared deviation from mean
- **Usage**: Volatility measurement
- **Implementation**: `ta_rust::statistic::var`

## Key Features

### Volume Analysis
- **OBV**: Classic volume-price relationship indicator
- **A/D Line**: Money flow accumulation/distribution
- **A/D Oscillator**: Momentum of money flow

### Volatility Measurement
- **Bollinger Bands**: Dynamic volatility bands
- **Standard Deviation**: Statistical volatility
- **Variance**: Squared volatility measure

### Adaptive Indicators
- **KAMA**: Efficiency-based adaptation
- **MAMA**: Hilbert Transform adaptation
- **MAVP**: Variable period adaptation
- **T3**: Volume factor adaptation

### Trend Analysis
- **Parabolic SAR**: Trend reversal points
- **TRIX**: Triple-smoothed momentum
- **Linear Regression**: Statistical trend fitting

### Statistical Analysis
- **Correlation**: Relationship measurement
- **Beta**: Systematic risk analysis
- **Regression Suite**: Complete trend analysis

## Usage Examples

### Basic Volume Analysis
```rust
use ta_rust::prelude::*;

let close = vec![20.0, 21.0, 22.0, 23.0, 24.0];
let volume = vec![1000.0, 1500.0, 800.0, 2000.0, 1200.0];

// On Balance Volume
let obv_result = obv(&close, &volume)?;

// A/D Line
let high = vec![22.0, 23.0, 24.0, 25.0, 26.0];
let low = vec![19.0, 20.0, 21.0, 22.0, 23.0];
let ad_result = ad(&high, &low, &close, &volume)?;
```

### Bollinger Bands Analysis
```rust
use ta_rust::prelude::*;

let close = vec![/* price data */];

// Standard Bollinger Bands (20, 2.0)
let bb_result = bbands(&close, 20, 2.0)?;

// Access bands
let upper_band = &bb_result.upper;
let middle_band = &bb_result.middle;
let lower_band = &bb_result.lower;

// Calculate %B
let percent_b = bbands_percent_b(&close, &bb_result)?;

// Calculate band width
let band_width = bbands_width(&bb_result)?;
```

### Parabolic SAR
```rust
use ta_rust::prelude::*;

let high = vec![/* high prices */];
let low = vec![/* low prices */];

// Standard SAR (0.02, 0.20)
let sar_result = sar(&high, &low, 0.02, 0.20)?;

// Get trend direction
let trend = sar_trend(&high, &low, &sar_result)?;

// Extended SAR with custom parameters
let sarext_result = sarext(&high, &low, 0.0, 0.0, 
                          0.02, 0.02, 0.20,  // Long AF
                          0.03, 0.03, 0.25)?; // Short AF
```

### Adaptive Moving Averages
```rust
use ta_rust::prelude::*;

let close = vec![/* price data */];

// KAMA
let kama_result = kama(&close, 10, 2, 30)?;

// T3
let t3_result = t3(&close, 5, 0.7)?;

// MAMA
let mama_result = mama(&close, 0.5, 0.05)?;
let mama_values = &mama_result.mama;
let fama_values = &mama_result.fama;
```

### Statistical Analysis
```rust
use ta_rust::prelude::*;

let series1 = vec![/* first data series */];
let series2 = vec![/* second data series */];

// Correlation
let correlation = correl(&series1, &series2, 20)?;

// Beta coefficient
let beta_coeff = beta(&series1, &series2, 20)?;

// Standard deviation
let std_dev = stddev(&series1, 5, 1.0)?;

// Linear regression
let lin_reg = linearreg(&series1, 14)?;
let slope = linearreg_slope(&series1, 14)?;
let angle = linearreg_angle(&series1, 14)?;
```

## Performance Characteristics

### Computational Complexity
- **Volume Indicators**: O(n) - Linear time
- **Bollinger Bands**: O(n) - Single pass with rolling statistics
- **Parabolic SAR**: O(n) - State machine approach
- **KAMA**: O(n) - Efficiency ratio calculation
- **MAMA**: O(n) - Hilbert Transform implementation
- **Statistical Functions**: O(n×p) - Rolling window calculations

### Memory Usage
- **Efficient**: Most functions use O(1) additional memory
- **MAMA**: Requires additional arrays for HT calculations
- **Statistical**: Temporary arrays for window calculations

### Accuracy
- **High Precision**: All functions maintain f64 precision
- **Numerical Stability**: Careful handling of edge cases
- **TA-Lib Compatible**: Results match reference implementation

## Testing and Validation

### Accuracy Testing
- Comprehensive test suite against TA-Lib reference
- Edge case handling (zero variance, constant data)
- Numerical precision validation

### Test Coverage
- Unit tests for all functions
- Integration tests with real market data
- Performance benchmarks

### Quality Assurance
- Input validation and error handling
- Documentation with examples
- Consistent API design

## Integration Notes

### Dependencies
- Builds on Phase 1-4 foundations
- Uses existing EMA and SMA implementations
- Leverages common error handling

### API Consistency
- Follows established patterns
- Consistent parameter naming
- Standard error types

### Future Extensions
- Additional volume indicators
- More statistical functions
- Enhanced adaptive algorithms

## Conclusion

Phase 5 significantly expands TA-Rust's analytical capabilities with sophisticated volume analysis, adaptive indicators, and comprehensive statistical functions. The implementation provides professional-grade tools for advanced technical analysis while maintaining the library's focus on performance and accuracy.

Key achievements:
- ✅ Complete volume indicator suite
- ✅ Advanced overlay studies including Bollinger Bands and SAR
- ✅ Adaptive moving averages with multiple algorithms
- ✅ Comprehensive statistical analysis functions
- ✅ High accuracy and performance
- ✅ Extensive testing and validation

This phase establishes TA-Rust as a comprehensive technical analysis library suitable for professional trading applications, research, and financial analysis.