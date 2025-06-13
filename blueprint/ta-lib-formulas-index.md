# Indeks Rumusan Matematika TA-Lib untuk Implementasi Rust

## Gambaran Umum
Dokumentasi ini berisi rumusan matematika lengkap untuk mengimplementasikan library TA-Lib dalam Rust murni. Rumusan ini dirancang untuk memastikan kompatibilitas 100% dengan TA-Lib asli dari segi logika dan hasil perhitungan.

## Struktur Dokumentasi

### 📄 ta-lib-formulas.md
Berisi rumusan untuk:
- **Overlap Studies** (17 fungsi): BBANDS, DEMA, EMA, HT_TRENDLINE, KAMA, MA, MAMA, MAVP, MIDPOINT, MIDPRICE, SAR, SAREXT, SMA, T3, TEMA, TRIMA, WMA
- **Momentum Indicators** (30 fungsi): ADX, ADXR, APO, AROON, AROONOSC, BOP, CCI, CMO, DX, MACD, dll.
- **Volume Indicators** (3 fungsi): AD, ADOSC, OBV
- **Volatility Indicators** (3 fungsi): ATR, NATR, TRANGE
- **Price Transform** (4 fungsi): AVGPRICE, MEDPRICE, TYPPRICE, WCLPRICE
- **Cycle Indicators** (5 fungsi): HT_DCPERIOD, HT_DCPHASE, HT_PHASOR, HT_SINE, HT_TRENDMODE
- **Pattern Recognition** (sebagian): CDL2CROWS hingga CDLHARAMI

### 📄 ta-lib-formulas-part2.md
Berisi rumusan untuk:
- **Pattern Recognition** (lanjutan): CDLHARAMICROSS hingga CDLXSIDEGAP3METHODS (total 61 pola candlestick)
- **Statistic Functions** (9 fungsi): BETA, CORREL, LINEARREG, LINEARREG_ANGLE, LINEARREG_INTERCEPT, LINEARREG_SLOPE, STDDEV, TSF, VAR
- **Math Transform Functions** (15 fungsi): ACOS, ASIN, ATAN, CEIL, COS, COSH, EXP, FLOOR, LN, LOG10, SIN, SINH, SQRT, TAN, TANH
- **Math Operator Functions** (11 fungsi): ADD, DIV, MAX, MAXINDEX, MIN, MININDEX, MINMAX, MINMAXINDEX, MULT, SUB, SUM
- **Implementation Notes**: Panduan implementasi penting

## Total Fungsi: 158+

## Konsep Penting untuk Implementasi

### 1. Tipe Data
```rust
// Gunakan f64 untuk semua kalkulasi
type Price = f64;
type Volume = f64;
type Period = usize;
```

### 2. Moving Average Types
```rust
enum MAType {
    SMA = 0,
    EMA = 1,
    WMA = 2,
    DEMA = 3,
    TEMA = 4,
    TRIMA = 5,
    KAMA = 6,
    MAMA = 7,
    T3 = 8,
}
```

### 3. Pattern Recognition Output
```rust
const PATTERN_BULLISH: i32 = 100;
const PATTERN_BEARISH: i32 = -100;
const PATTERN_NONE: i32 = 0;
```

### 4. Unstable Period
Beberapa indikator memerlukan periode "pemanasan":
- **SMA**: `period` nilai
- **EMA**: `2 * period - 1` nilai
- **RSI/ATR/ADX**: menggunakan Wilder's smoothing
- **MACD**: `slow_period + signal_period - 1`

### 5. Error Handling
```rust
// Contoh penanganan error
fn calculate_rsi(prices: &[f64], period: usize) -> Result<Vec<f64>, TAError> {
    if prices.len() < period {
        return Err(TAError::InsufficientData);
    }
    // ... kalkulasi
}
```

### 6. Wilder's Smoothing
Untuk RSI, ATR, ADX gunakan:
```rust
alpha = 1.0 / period  // bukan 2.0 / (period + 1.0)
```

### 7. Hilbert Transform
Untuk fungsi HT_*, implementasi memerlukan:
- 4-bar FIR filter untuk smoothing
- Hilbert Transform untuk fase dan periode
- Adaptive period calculation

### 8. Candlestick Pattern Recognition
Setiap pola memiliki:
- Kondisi spesifik yang harus dipenuhi
- Threshold untuk body/shadow ratios
- Context awareness (trend direction)

### 9. Performance Tips
- Pre-allocate vectors dengan kapasitas yang diketahui
- Gunakan iterators daripada indexing jika memungkinkan
- Cache nilai yang sering digunakan
- Implementasikan rolling calculations untuk update incremental

### 10. Testing & Validation
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sma_accuracy() {
        let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sma(&prices, 3).unwrap();
        assert_eq!(result[2], 2.0); // (1+2+3)/3
        assert_eq!(result[3], 3.0); // (2+3+4)/3
        assert_eq!(result[4], 4.0); // (3+4+5)/3
    }
}
```

## Struktur Project Rust yang Disarankan
```
ta-rust/
├── src/
│   ├── lib.rs
│   ├── overlap/
│   │   ├── mod.rs
│   │   ├── sma.rs
│   │   ├── ema.rs
│   │   └── ...
│   ├── momentum/
│   │   ├── mod.rs
│   │   ├── rsi.rs
│   │   ├── macd.rs
│   │   └── ...
│   ├── volume/
│   ├── volatility/
│   ├── pattern/
│   ├── cycle/
│   ├── statistic/
│   ├── math_transform/
│   ├── math_operators/
│   └── common/
│       ├── mod.rs
│       ├── types.rs
│       └── utils.rs
├── tests/
├── benches/
└── Cargo.toml
```

## Catatan Kompatibilitas TA-Lib
1. **Parameter Default**: Gunakan default yang sama dengan TA-Lib C
2. **Output Format**: Ikuti konvensi output TA-Lib (leading NaN untuk unstable period)
3. **Precision**: Usahakan presisi hingga 8 decimal places
4. **Edge Cases**: Handle sesuai behavior TA-Lib asli

## Referensi Implementasi
Untuk setiap fungsi, ikuti pola:
1. Validasi input
2. Alokasi output dengan size yang tepat
3. Handle unstable period
4. Lakukan kalkulasi
5. Return hasil atau error
6. Dokumentasi yang lengkap untuk penggunaan di projek rust

Dengan rumusan matematika yang telah disediakan, Anda dapat mengimplementasikan TA-Lib dalam Rust dengan akurasi yang sama persis dengan versi aslinya.
