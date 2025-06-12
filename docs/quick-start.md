# Quick Start Guide

Get up and running with TA-Rust in minutes! This guide covers the most common use cases and patterns.

## 🚀 Basic Usage

### Import the Library

```rust
use ta_rust::prelude::*;
```

The prelude imports all commonly used functions and types.

### Sample Data

Let's start with some sample price data:

```rust
let prices = vec![
    44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 45.92,
    45.73, 46.16, 47.04, 46.07, 46.03, 46.83, 47.69, 46.49,
    46.26, 47.09, 46.66, 46.80, 47.12, 45.81, 46.12, 45.55,
];
```

## 📊 Moving Averages

### Simple Moving Average (SMA)

```rust
let sma_result = sma(&prices, 10)?;
println!("SMA(10): {:?}", sma_result);
```

### Exponential Moving Average (EMA)

```rust
let ema_result = ema(&prices, 10)?;
println!("EMA(10): {:?}", ema_result);
```

### Weighted Moving Average (WMA)

```rust
let wma_result = wma(&prices, 10)?;
println!("WMA(10): {:?}", wma_result);
```

### Generic Moving Average

```rust
// Use different MA types
let sma_result = ma(&prices, 10, MAType::SMA)?;
let ema_result = ma(&prices, 10, MAType::EMA)?;
let wma_result = ma(&prices, 10, MAType::WMA)?;
```

## 📈 Momentum Indicators

### RSI (Relative Strength Index)

```rust
let rsi_result = rsi(&prices, 14)?;
println!("RSI(14): {:?}", rsi_result);

// Values above 70 indicate overbought, below 30 indicate oversold
for (i, &rsi_val) in rsi_result.iter().enumerate() {
    if !rsi_val.is_nan() {
        if rsi_val > 70.0 {
            println!("Day {}: Overbought (RSI: {:.2})", i, rsi_val);
        } else if rsi_val < 30.0 {
            println!("Day {}: Oversold (RSI: {:.2})", i, rsi_val);
        }
    }
}
```

### MACD (Moving Average Convergence/Divergence)

```rust
let (macd_line, signal_line, histogram) = macd(&prices, 12, 26, 9)?;

println!("MACD Line: {:?}", macd_line);
println!("Signal Line: {:?}", signal_line);
println!("Histogram: {:?}", histogram);

// Check for MACD crossovers
for i in 1..macd_line.len() {
    if !macd_line[i].is_nan() && !signal_line[i].is_nan() {
        let prev_diff = macd_line[i-1] - signal_line[i-1];
        let curr_diff = macd_line[i] - signal_line[i];
        
        if prev_diff < 0.0 && curr_diff > 0.0 {
            println!("Day {}: Bullish MACD crossover", i);
        } else if prev_diff > 0.0 && curr_diff < 0.0 {
            println!("Day {}: Bearish MACD crossover", i);
        }
    }
}
```

### Williams %R

```rust
// For Williams %R, we need OHLC data
let high = vec![45.0, 46.0, 47.0, 48.0, 49.0, 48.5, 47.5, 46.5, 45.5, 44.5];
let low = vec![43.0, 44.0, 45.0, 46.0, 47.0, 46.5, 45.5, 44.5, 43.5, 42.5];
let close = vec![44.0, 45.0, 46.0, 47.0, 48.0, 47.0, 46.0, 45.0, 44.0, 43.0];

let willr_result = willr(&high, &low, &close, 14)?;
println!("Williams %R: {:?}", willr_result);
```

## 📊 Volatility Indicators

### Average True Range (ATR)

```rust
let high = vec![45.0, 46.0, 47.0, 48.0, 49.0, 48.5, 47.5, 46.5, 45.5, 44.5];
let low = vec![43.0, 44.0, 45.0, 46.0, 47.0, 46.5, 45.5, 44.5, 43.5, 42.5];
let close = vec![44.0, 45.0, 46.0, 47.0, 48.0, 47.0, 46.0, 45.0, 44.0, 43.0];

let atr_result = atr(&high, &low, &close, 14)?;
println!("ATR(14): {:?}", atr_result);

// ATR can be used for position sizing
let current_atr = atr_result.last().unwrap();
let position_size = 1000.0 / current_atr; // Risk $1000 per ATR unit
println!("Suggested position size: {:.2}", position_size);
```

### True Range

```rust
let tr_result = trange(&high, &low, &close)?;
println!("True Range: {:?}", tr_result);
```

## 🔄 Price Transforms

### Typical Price

```rust
let typical_price = typprice(&high, &low, &close)?;
println!("Typical Price: {:?}", typical_price);
```

### Weighted Close Price

```rust
let weighted_close = wclprice(&high, &low, &close)?;
println!("Weighted Close: {:?}", weighted_close);
```

### Average Price

```rust
let open = vec![43.5, 44.5, 45.5, 46.5, 47.5, 46.5, 45.5, 44.5, 43.5, 42.5];
let avg_price = avgprice(&open, &high, &low, &close)?;
println!("Average Price: {:?}", avg_price);
```

## 🧮 Math Operations

### Basic Operations

```rust
let data1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let data2 = vec![2.0, 3.0, 4.0, 5.0, 6.0];

// Vector operations
let sum_result = add(&data1, &data2)?;
let diff_result = sub(&data1, &data2)?;
let mult_result = mult(&data1, &data2)?;
let div_result = div(&data1, &data2)?;

println!("Addition: {:?}", sum_result);
println!("Subtraction: {:?}", diff_result);
println!("Multiplication: {:?}", mult_result);
println!("Division: {:?}", div_result);
```

### Statistical Operations

```rust
let data = vec![1.0, 5.0, 3.0, 9.0, 2.0, 8.0, 4.0, 7.0, 6.0];

let max_vals = max(&data, 3)?;
let min_vals = min(&data, 3)?;
let sum_vals = sum(&data, 3)?;

println!("Max(3): {:?}", max_vals);
println!("Min(3): {:?}", min_vals);
println!("Sum(3): {:?}", sum_vals);
```

## ���� Error Handling

TA-Rust uses Result types for error handling:

```rust
use ta_rust::prelude::*;

fn analyze_data(prices: &[f64]) -> Result<(), TAError> {
    // This will return an error if insufficient data
    let sma_result = sma(prices, 20)?;
    
    // This will return an error if period is 0
    let rsi_result = rsi(prices, 0)?;
    
    Ok(())
}

// Handle errors gracefully
match analyze_data(&prices) {
    Ok(()) => println!("Analysis completed successfully"),
    Err(TAError::InsufficientData { required, provided }) => {
        println!("Need {} data points, got {}", required, provided);
    }
    Err(TAError::InvalidPeriod { period, name }) => {
        println!("Invalid period {} for {}", period, name);
    }
    Err(e) => println!("Error: {}", e),
}
```

## 📊 Working with OHLC Data

### Using OHLC Struct

```rust
use ta_rust::common::OHLC;

let ohlc_data = vec![
    OHLC { open: 43.5, high: 45.0, low: 43.0, close: 44.0 },
    OHLC { open: 44.0, high: 46.0, low: 44.0, close: 45.0 },
    OHLC { open: 45.0, high: 47.0, low: 45.0, close: 46.0 },
    // ... more data
];

// Extract individual arrays
let opens: Vec<f64> = ohlc_data.iter().map(|x| x.open).collect();
let highs: Vec<f64> = ohlc_data.iter().map(|x| x.high).collect();
let lows: Vec<f64> = ohlc_data.iter().map(|x| x.low).collect();
let closes: Vec<f64> = ohlc_data.iter().map(|x| x.close).collect();

// Use with indicators
let atr_result = atr(&highs, &lows, &closes, 14)?;
```

## 🎯 Common Patterns

### Trend Analysis

```rust
fn analyze_trend(prices: &[f64]) -> Result<String, TAError> {
    let sma_short = sma(prices, 10)?;
    let sma_long = sma(prices, 20)?;
    
    let latest_short = sma_short.last().unwrap();
    let latest_long = sma_long.last().unwrap();
    
    if latest_short > latest_long {
        Ok("Uptrend".to_string())
    } else if latest_short < latest_long {
        Ok("Downtrend".to_string())
    } else {
        Ok("Sideways".to_string())
    }
}
```

### Momentum Analysis

```rust
fn analyze_momentum(prices: &[f64]) -> Result<String, TAError> {
    let rsi_result = rsi(prices, 14)?;
    let latest_rsi = rsi_result.last().unwrap();
    
    if *latest_rsi > 70.0 {
        Ok("Overbought".to_string())
    } else if *latest_rsi < 30.0 {
        Ok("Oversold".to_string())
    } else {
        Ok("Neutral".to_string())
    }
}
```

### Volatility Analysis

```rust
fn analyze_volatility(high: &[f64], low: &[f64], close: &[f64]) -> Result<String, TAError> {
    let atr_result = atr(high, low, close, 14)?;
    let latest_atr = atr_result.last().unwrap();
    
    // Compare with historical average
    let atr_avg = atr_result.iter()
        .filter(|x| !x.is_nan())
        .sum::<f64>() / atr_result.iter().filter(|x| !x.is_nan()).count() as f64;
    
    if *latest_atr > atr_avg * 1.5 {
        Ok("High Volatility".to_string())
    } else if *latest_atr < atr_avg * 0.5 {
        Ok("Low Volatility".to_string())
    } else {
        Ok("Normal Volatility".to_string())
    }
}
```

## 🚀 Performance Tips

### 1. Reuse Calculations

```rust
// Instead of calculating SMA multiple times
let sma_10 = sma(&prices, 10)?;
let sma_20 = sma(&prices, 20)?;

// Use the generic MA function for consistency
let sma_10 = ma(&prices, 10, MAType::SMA)?;
let sma_20 = ma(&prices, 20, MAType::SMA)?;
```

### 2. Batch Processing

```rust
// Process multiple indicators at once
fn full_analysis(prices: &[f64], high: &[f64], low: &[f64], close: &[f64]) 
    -> Result<(), TAError> {
    
    let sma_result = sma(prices, 20)?;
    let ema_result = ema(prices, 20)?;
    let rsi_result = rsi(prices, 14)?;
    let atr_result = atr(high, low, close, 14)?;
    let (macd, signal, histogram) = macd(prices, 12, 26, 9)?;
    
    // Process all results together
    Ok(())
}
```

## 📚 Next Steps

Now that you've learned the basics:

1. Explore [Indicator Documentation](indicators/) for detailed function references
2. Read the [Performance Guide](performance.md) for optimization tips
3. Check out [Error Handling Guide](error-handling.md) for robust error management
4. Browse [Advanced Examples](examples/) for complex use cases

## 💡 Pro Tips

1. **Always handle errors** - Financial calculations can fail in many ways
2. **Validate your data** - Check for NaN values and sufficient data points
3. **Use appropriate periods** - Different timeframes need different parameters
4. **Combine indicators** - No single indicator tells the whole story
5. **Test thoroughly** - Validate your logic with known data sets

## 🔗 Useful Links

- [API Documentation](https://docs.rs/ta-rust)
- [GitHub Repository](https://github.com/pixelbrow720/ta-rust)
- [Issue Tracker](https://github.com/pixelbrow720/ta-rust/issues)
- [Original TA-Lib Documentation](https://ta-lib.org/)