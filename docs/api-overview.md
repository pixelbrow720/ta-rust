# API Overview

This document provides a comprehensive overview of the TA-Rust API, including all available functions, types, and usage patterns.

## 📚 Table of Contents

- [Core Types](#core-types)
- [Error Handling](#error-handling)
- [Function Categories](#function-categories)
- [Common Patterns](#common-patterns)
- [Performance Considerations](#performance-considerations)

## 🔧 Core Types

### Basic Types
```rust
pub type Price = f64;           // Price values
pub type Volume = f64;          // Volume values  
pub type Period = usize;        // Time periods
pub type TAResult<T> = Result<T, TAError>;  // Result type
```

### OHLC Data Structures
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OHLC {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OHLCV {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}
```

### Moving Average Types
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MAType {
    SMA,    // Simple Moving Average
    EMA,    // Exponential Moving Average
    WMA,    // Weighted Moving Average
    DEMA,   // Double Exponential Moving Average
    TEMA,   // Triple Exponential Moving Average
    TRIMA,  // Triangular Moving Average
    KAMA,   // Kaufman Adaptive Moving Average (planned)
    MAMA,   // MESA Adaptive Moving Average (planned)
    T3,     // Triple Exponential Moving Average T3 (planned)
}
```

## ⚠️ Error Handling

### TAError Enum
```rust
#[derive(Error, Debug, PartialEq)]
pub enum TAError {
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
    
    #[error("Insufficient data: need {required}, got {provided}")]
    InsufficientData { required: usize, provided: usize },
    
    #[error("Invalid period {period} for {name}")]
    InvalidPeriod { period: usize, name: String },
    
    #[error("Mismatched input lengths: {message}")]
    MismatchedInputs { message: String },
    
    #[error("Calculation error: {message}")]
    CalculationError { message: String },
}
```

### Error Handling Patterns
```rust
use ta_rust::prelude::*;

// Pattern 1: Match specific errors
match sma(&prices, period) {
    Ok(result) => println!("SMA: {:?}", result),
    Err(TAError::InsufficientData { required, provided }) => {
        println!("Need {} data points, got {}", required, provided);
    }
    Err(TAError::InvalidPeriod { period, name }) => {
        println!("Invalid period {} for {}", period, name);
    }
    Err(e) => println!("Error: {}", e),
}

// Pattern 2: Use ? operator for propagation
fn analyze_data(prices: &[f64]) -> TAResult<()> {
    let sma_result = sma(prices, 20)?;
    let rsi_result = rsi(prices, 14)?;
    // Process results...
    Ok(())
}

// Pattern 3: Convert to Box<dyn Error>
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = sma(&prices, 10)?;
    Ok(())
}
```

## 📊 Function Categories

### 1. Overlap Studies (9 functions)

#### Simple Moving Average
```rust
pub fn sma(data: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn sma_rolling(data: &[f64], period: usize) -> TAResult<Vec<f64>>
```

#### Exponential Moving Average
```rust
pub fn ema(data: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn ema_custom(data: &[f64], period: usize, alpha: f64) -> TAResult<Vec<f64>>
```

#### Weighted Moving Average
```rust
pub fn wma(data: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn wma_custom(data: &[f64], weights: &[f64]) -> TAResult<Vec<f64>>
```

#### Double/Triple Exponential Moving Averages
```rust
pub fn dema(data: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn tema(data: &[f64], period: usize) -> TAResult<Vec<f64>>
```

#### Triangular Moving Average
```rust
pub fn trima(data: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn trima_custom_peak(data: &[f64], period: usize, peak_position: f64) -> TAResult<Vec<f64>>
```

#### Generic Moving Average
```rust
pub fn ma(data: &[f64], period: usize, ma_type: MAType) -> TAResult<Vec<f64>>
pub fn ma_auto(data: &[f64], period: usize) -> TAResult<(MAType, Vec<f64>)>
pub fn ma_multiple(data: &[f64], periods: &[usize], ma_type: MAType) -> TAResult<Vec<Vec<f64>>>
```

#### MidPoint and MidPrice
```rust
pub fn midpoint(data: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn midprice(high: &[f64], low: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn midprice_adaptive(high: &[f64], low: &[f64], base_period: usize, vol_period: usize) -> TAResult<Vec<f64>>
```

### 2. Momentum Indicators (30 functions)

#### RSI Family
```rust
pub fn rsi(data: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn rsi_custom(data: &[f64], period: usize, alpha: f64) -> TAResult<Vec<f64>>
pub fn rsi_levels(data: &[f64], period: usize, overbought: f64, oversold: f64) -> TAResult<(Vec<f64>, Vec<bool>, Vec<bool>)>
```

#### MACD Family
```rust
pub fn macd(data: &[f64], fast_period: usize, slow_period: usize, signal_period: usize) -> TAResult<(Vec<f64>, Vec<f64>, Vec<f64>)>
pub fn macdext(data: &[f64], fast_period: usize, fast_ma_type: MAType, slow_period: usize, slow_ma_type: MAType, signal_period: usize, signal_ma_type: MAType) -> TAResult<(Vec<f64>, Vec<f64>, Vec<f64>)>
pub fn macdfix(data: &[f64], signal_period: usize) -> TAResult<(Vec<f64>, Vec<f64>, Vec<f64>)>
```

#### Stochastic Family
```rust
pub fn stoch(high: &[f64], low: &[f64], close: &[f64], fastk_period: usize, slowk_period: usize, slowk_ma_type: MAType, slowd_period: usize, slowd_ma_type: MAType) -> TAResult<(Vec<f64>, Vec<f64>)>
pub fn stochf(high: &[f64], low: &[f64], close: &[f64], fastk_period: usize, fastd_period: usize, fastd_ma_type: MAType) -> TAResult<(Vec<f64>, Vec<f64>)>
pub fn stochrsi(data: &[f64], period: usize, fastk_period: usize, fastd_period: usize, fastd_ma_type: MAType) -> TAResult<(Vec<f64>, Vec<f64>)>
```

#### ADX System
```rust
pub fn adx(high: &[f64], low: &[f64], close: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn adxr(high: &[f64], low: &[f64], close: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn plus_di(high: &[f64], low: &[f64], close: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn minus_di(high: &[f64], low: &[f64], close: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn dx(high: &[f64], low: &[f64], close: &[f64], period: usize) -> TAResult<Vec<f64>>
```

#### Other Oscillators
```rust
pub fn willr(high: &[f64], low: &[f64], close: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn cci(high: &[f64], low: &[f64], close: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn mfi(high: &[f64], low: &[f64], close: &[f64], volume: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn bop(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TAResult<Vec<f64>>
pub fn ultosc(high: &[f64], low: &[f64], close: &[f64], period1: usize, period2: usize, period3: usize) -> TAResult<Vec<f64>>
```

#### Rate of Change Family
```rust
pub fn mom(data: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn roc(data: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn rocp(data: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn rocr(data: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn rocr100(data: &[f64], period: usize) -> TAResult<Vec<f64>>
```

### 3. Volatility Indicators (3 functions)

```rust
pub fn trange(high: &[f64], low: &[f64], close: &[f64]) -> TAResult<Vec<f64>>
pub fn atr(high: &[f64], low: &[f64], close: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn natr(high: &[f64], low: &[f64], close: &[f64], period: usize) -> TAResult<Vec<f64>>
```

### 4. Price Transform (4 functions)

```rust
pub fn avgprice(open: &[f64], high: &[f64], low: &[f64], close: &[f64]) -> TAResult<Vec<f64>>
pub fn medprice(high: &[f64], low: &[f64]) -> TAResult<Vec<f64>>
pub fn typprice(high: &[f64], low: &[f64], close: &[f64]) -> TAResult<Vec<f64>>
pub fn wclprice(high: &[f64], low: &[f64], close: &[f64]) -> TAResult<Vec<f64>>
```

### 5. Math Transform (15 functions)

#### Trigonometric
```rust
pub fn sin(data: &[f64]) -> TAResult<Vec<f64>>
pub fn cos(data: &[f64]) -> TAResult<Vec<f64>>
pub fn tan(data: &[f64]) -> TAResult<Vec<f64>>
pub fn asin(data: &[f64]) -> TAResult<Vec<f64>>
pub fn acos(data: &[f64]) -> TAResult<Vec<f64>>
pub fn atan(data: &[f64]) -> TAResult<Vec<f64>>
```

#### Hyperbolic
```rust
pub fn sinh(data: &[f64]) -> TAResult<Vec<f64>>
pub fn cosh(data: &[f64]) -> TAResult<Vec<f64>>
pub fn tanh(data: &[f64]) -> TAResult<Vec<f64>>
```

#### Logarithmic
```rust
pub fn ln(data: &[f64]) -> TAResult<Vec<f64>>
pub fn log10(data: &[f64]) -> TAResult<Vec<f64>>
pub fn exp(data: &[f64]) -> TAResult<Vec<f64>>
```

#### Other
```rust
pub fn sqrt(data: &[f64]) -> TAResult<Vec<f64>>
pub fn ceil(data: &[f64]) -> TAResult<Vec<f64>>
pub fn floor(data: &[f64]) -> TAResult<Vec<f64>>
```

### 6. Math Operators (11 functions)

#### Arithmetic
```rust
pub fn add(data1: &[f64], data2: &[f64]) -> TAResult<Vec<f64>>
pub fn sub(data1: &[f64], data2: &[f64]) -> TAResult<Vec<f64>>
pub fn mult(data1: &[f64], data2: &[f64]) -> TAResult<Vec<f64>>
pub fn div(data1: &[f64], data2: &[f64]) -> TAResult<Vec<f64>>
```

#### Statistical
```rust
pub fn max(data: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn min(data: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn sum(data: &[f64], period: usize) -> TAResult<Vec<f64>>
pub fn minmax(data: &[f64], period: usize) -> TAResult<(Vec<f64>, Vec<f64>)>
```

## 🎯 Common Patterns

### 1. Single Indicator Analysis
```rust
use ta_rust::prelude::*;

fn simple_analysis(prices: &[f64]) -> TAResult<()> {
    let sma_20 = sma(prices, 20)?;
    let rsi_14 = rsi(prices, 14)?;
    
    // Get latest values
    let latest_sma = sma_20.last().unwrap();
    let latest_rsi = rsi_14.last().unwrap();
    
    println!("Latest SMA(20): {:.2}", latest_sma);
    println!("Latest RSI(14): {:.2}", latest_rsi);
    
    Ok(())
}
```

### 2. Multi-Indicator Strategy
```rust
use ta_rust::prelude::*;

fn trend_following_strategy(prices: &[f64], high: &[f64], low: &[f64], close: &[f64]) 
    -> TAResult<Vec<String>> {
    
    let sma_short = sma(prices, 10)?;
    let sma_long = sma(prices, 20)?;
    let rsi_14 = rsi(prices, 14)?;
    let atr_14 = atr(high, low, close, 14)?;
    
    let mut signals = Vec::new();
    
    for i in 20..prices.len() {
        let trend = if sma_short[i] > sma_long[i] { "UP" } else { "DOWN" };
        let momentum = if rsi_14[i] > 70.0 { "OVERBOUGHT" } 
                      else if rsi_14[i] < 30.0 { "OVERSOLD" } 
                      else { "NEUTRAL" };
        let volatility = if atr_14[i] > atr_14[i-1] { "INCREASING" } else { "DECREASING" };
        
        signals.push(format!("Day {}: Trend={}, Momentum={}, Volatility={}", 
                           i, trend, momentum, volatility));
    }
    
    Ok(signals)
}
```

### 3. OHLC Data Processing
```rust
use ta_rust::prelude::*;

fn ohlc_analysis(ohlc_data: &[OHLC]) -> TAResult<()> {
    // Extract individual arrays
    let opens: Vec<f64> = ohlc_data.iter().map(|x| x.open).collect();
    let highs: Vec<f64> = ohlc_data.iter().map(|x| x.high).collect();
    let lows: Vec<f64> = ohlc_data.iter().map(|x| x.low).collect();
    let closes: Vec<f64> = ohlc_data.iter().map(|x| x.close).collect();
    
    // Calculate indicators
    let atr_result = atr(&highs, &lows, &closes, 14)?;
    let willr_result = willr(&highs, &lows, &closes, 14)?;
    let typical_price = typprice(&highs, &lows, &closes)?;
    
    // Process results...
    Ok(())
}
```

### 4. Signal Generation
```rust
use ta_rust::prelude::*;

fn generate_signals(prices: &[f64]) -> TAResult<Vec<&'static str>> {
    let (macd_line, signal_line, _) = macd(prices, 12, 26, 9)?;
    let rsi_values = rsi(prices, 14)?;
    
    let mut signals = Vec::new();
    
    for i in 1..prices.len() {
        // MACD crossover signals
        if !macd_line[i].is_nan() && !signal_line[i].is_nan() {
            let prev_diff = macd_line[i-1] - signal_line[i-1];
            let curr_diff = macd_line[i] - signal_line[i];
            
            if prev_diff < 0.0 && curr_diff > 0.0 {
                signals.push("BUY");
            } else if prev_diff > 0.0 && curr_diff < 0.0 {
                signals.push("SELL");
            } else {
                signals.push("HOLD");
            }
        } else {
            signals.push("WAIT");
        }
    }
    
    Ok(signals)
}
```

### 5. Batch Processing
```rust
use ta_rust::prelude::*;

fn batch_analysis(datasets: &[Vec<f64>]) -> TAResult<Vec<f64>> {
    let mut results = Vec::new();
    
    for dataset in datasets {
        let sma_result = sma(dataset, 20)?;
        let latest_sma = sma_result.last().unwrap();
        results.push(*latest_sma);
    }
    
    Ok(results)
}
```

## ⚡ Performance Considerations

### 1. Memory Allocation
```rust
// Good: Pre-allocate when size is known
let mut results = Vec::with_capacity(prices.len());

// Good: Reuse vectors when possible
let mut buffer = vec![0.0; 1000];
// ... use buffer for calculations

// Avoid: Frequent reallocations
let mut results = Vec::new(); // Will reallocate as it grows
```

### 2. Function Selection
```rust
// For large datasets, prefer rolling calculations
let sma_result = sma_rolling(&prices, period)?; // O(n)
// vs
let sma_result = sma(&prices, period)?; // O(n*m) for some implementations

// Use combined operations when possible
let (min_vals, max_vals) = minmax(&data, period)?; // Single pass
// vs
let min_vals = min(&data, period)?; // Two passes
let max_vals = max(&data, period)?;
```

### 3. Error Handling Optimization
```rust
// Pre-validate once for multiple calculations
validate_input_data(&prices, max_period, "analysis")?;

// Then use unchecked versions if available (future feature)
let sma_result = sma_unchecked(&prices, 20);
let ema_result = ema_unchecked(&prices, 20);
```

### 4. Data Organization
```rust
// Organize data for cache efficiency
struct MarketData {
    prices: Vec<f64>,
    volumes: Vec<f64>,
    // ... other fields
}

// Process related data together
impl MarketData {
    fn calculate_indicators(&self) -> TAResult<Indicators> {
        let sma = sma(&self.prices, 20)?;
        let ema = ema(&self.prices, 20)?;
        // ... other calculations
        Ok(Indicators { sma, ema })
    }
}
```

## 🔗 Integration Examples

### With Serde for JSON
```rust
use serde::{Deserialize, Serialize};
use ta_rust::prelude::*;

#[derive(Serialize, Deserialize)]
struct PriceData {
    prices: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
struct IndicatorResults {
    sma: Vec<f64>,
    rsi: Vec<f64>,
    macd: Vec<f64>,
}

fn process_json_data(json_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data: PriceData = serde_json::from_str(json_str)?;
    
    let sma_result = sma(&data.prices, 20)?;
    let rsi_result = rsi(&data.prices, 14)?;
    let (macd_result, _, _) = macd(&data.prices, 12, 26, 9)?;
    
    let results = IndicatorResults {
        sma: sma_result,
        rsi: rsi_result,
        macd: macd_result,
    };
    
    Ok(serde_json::to_string(&results)?)
}
```

### With Async Processing
```rust
use ta_rust::prelude::*;
use tokio;

async fn async_analysis(prices: Vec<f64>) -> TAResult<Vec<f64>> {
    // Spawn CPU-intensive calculation on thread pool
    let result = tokio::task::spawn_blocking(move || {
        sma(&prices, 20)
    }).await.unwrap()?;
    
    Ok(result)
}
```

This API overview covers all the essential patterns and functions available in TA-Rust. For specific function details, see the individual indicator documentation or the [API documentation](https://docs.rs/ta-rust).