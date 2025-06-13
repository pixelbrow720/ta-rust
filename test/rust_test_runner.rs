use ta_rust::prelude::*;
use serde_json;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <function_name> [params...]", args[0]);
        std::process::exit(1);
    }
    
    let function_name = &args[1];
    
    match function_name.as_str() {
        "sma" => test_sma(),
        "ema" => test_ema(),
        "wma" => test_wma(),
        "rsi" => test_rsi(),
        "atr" => test_atr(),
        "macd" => test_macd(),
        "bbands" => test_bbands(),
        "sar" => test_sar(),
        "obv" => test_obv(),
        "ad" => test_ad(),
        _ => {
            eprintln!("Unknown function: {}", function_name);
            std::process::exit(1);
        }
    }
}

fn test_sma() -> Result<(), Box<dyn std::error::Error>> {
    // Sample data for testing
    let data = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93,
        46.83, 47.69, 46.49, 46.26, 47.09, 46.66, 46.80, 47.12, 45.81, 46.12
    ];
    
    let result = sma(&data, 10)?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}

fn test_ema() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93,
        46.83, 47.69, 46.49, 46.26, 47.09, 46.66, 46.80, 47.12, 45.81, 46.12
    ];
    
    let result = ema(&data, 10)?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}

fn test_wma() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93,
        46.83, 47.69, 46.49, 46.26, 47.09, 46.66, 46.80, 47.12, 45.81, 46.12
    ];
    
    let result = wma(&data, 10)?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}

fn test_rsi() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93,
        46.83, 47.69, 46.49, 46.26, 47.09, 46.66, 46.80, 47.12, 45.81, 46.12,
        45.55, 46.08, 47.00, 46.03, 46.83, 47.69, 46.49, 46.26, 47.09, 46.66
    ];
    
    let result = rsi(&data, 14)?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}

fn test_atr() -> Result<(), Box<dyn std::error::Error>> {
    let high = vec![
        48.70, 48.72, 48.90, 48.87, 48.82, 49.05, 49.20, 49.35, 49.92, 50.19,
        50.12, 49.66, 49.88, 50.19, 50.36, 50.57, 50.65, 50.43, 49.63, 50.33
    ];
    let low = vec![
        47.79, 48.14, 48.39, 48.37, 48.24, 48.64, 48.94, 49.50, 49.87, 49.20,
        49.73, 48.90, 49.43, 49.73, 49.26, 50.09, 50.18, 49.21, 48.98, 49.61
    ];
    let close = vec![
        48.16, 48.61, 48.75, 48.63, 48.74, 49.03, 49.07, 49.32, 49.91, 50.13,
        49.53, 49.50, 49.75, 50.03, 50.31, 50.52, 50.41, 49.34, 49.37, 50.23
    ];
    
    let result = atr(&high, &low, &close, 14)?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}

fn test_macd() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![
        459.99, 448.85, 446.06, 450.81, 442.80, 448.97, 444.57, 441.40, 430.47, 420.05,
        431.14, 425.66, 430.58, 431.72, 437.87, 428.43, 428.35, 432.50, 443.66, 455.72,
        454.49, 452.08, 452.73, 461.91, 463.58, 461.14, 452.08, 442.66, 428.91, 429.79
    ];
    
    let (macd_line, signal_line, histogram) = macd(&data, 12, 26, 9)?;
    println!("{}", serde_json::to_string(&macd_line)?);
    Ok(())
}

fn test_bbands() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93,
        46.83, 47.69, 46.49, 46.26, 47.09, 46.66, 46.80, 47.12, 45.81, 46.12,
        45.55, 46.08, 47.00, 46.03, 46.83, 47.69, 46.49, 46.26, 47.09, 46.66
    ];
    
    let result = bbands(&data, 20, 2.0)?;
    println!("{}", serde_json::to_string(&result.middle)?);
    Ok(())
}

fn test_sar() -> Result<(), Box<dyn std::error::Error>> {
    let high = vec![
        48.70, 48.72, 48.90, 48.87, 48.82, 49.05, 49.20, 49.35, 49.92, 50.19,
        50.12, 49.66, 49.88, 50.19, 50.36, 50.57, 50.65, 50.43, 49.63, 50.33,
        51.12, 50.89, 50.95, 51.04, 50.82, 51.33, 51.44, 51.54, 51.75, 51.38
    ];
    let low = vec![
        47.79, 48.14, 48.39, 48.37, 48.24, 48.64, 48.94, 49.50, 49.87, 49.20,
        49.73, 48.90, 49.43, 49.73, 49.26, 50.09, 50.18, 49.21, 48.98, 49.61,
        50.25, 50.20, 50.31, 50.47, 50.02, 50.66, 50.67, 50.85, 51.16, 50.53
    ];
    
    let result = sar(&high, &low, 0.02, 0.20)?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}

fn test_obv() -> Result<(), Box<dyn std::error::Error>> {
    let close = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93,
        46.83, 47.69, 46.49, 46.26, 47.09, 46.66, 46.80, 47.12, 45.81, 46.12
    ];
    let volume = vec![
        1000.0, 1500.0, 800.0, 2000.0, 1200.0, 1800.0, 1600.0, 2200.0, 1900.0, 1700.0,
        1300.0, 2100.0, 1400.0, 1100.0, 1900.0, 1600.0, 1800.0, 2000.0, 1200.0, 1500.0
    ];
    
    let result = obv(&close, &volume)?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}

fn test_ad() -> Result<(), Box<dyn std::error::Error>> {
    let high = vec![
        48.70, 48.72, 48.90, 48.87, 48.82, 49.05, 49.20, 49.35, 49.92, 50.19,
        50.12, 49.66, 49.88, 50.19, 50.36, 50.57, 50.65, 50.43, 49.63, 50.33
    ];
    let low = vec![
        47.79, 48.14, 48.39, 48.37, 48.24, 48.64, 48.94, 49.50, 49.87, 49.20,
        49.73, 48.90, 49.43, 49.73, 49.26, 50.09, 50.18, 49.21, 48.98, 49.61
    ];
    let close = vec![
        48.16, 48.61, 48.75, 48.63, 48.74, 49.03, 49.07, 49.32, 49.91, 50.13,
        49.53, 49.50, 49.75, 50.03, 50.31, 50.52, 50.41, 49.34, 49.37, 50.23
    ];
    let volume = vec![
        1000.0, 1500.0, 800.0, 2000.0, 1200.0, 1800.0, 1600.0, 2200.0, 1900.0, 1700.0,
        1300.0, 2100.0, 1400.0, 1100.0, 1900.0, 1600.0, 1800.0, 2000.0, 1200.0, 1500.0
    ];
    
    let result = ad(&high, &low, &close, &volume)?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}
