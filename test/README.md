# TA-Rust Testing Suite

Folder ini berisi skrip testing komprehensif untuk memverifikasi kompatibilitas 100% antara ta-rust dengan TA-Lib original. Testing suite ini dirancang untuk memastikan bahwa hasil dari implementasi fase 1-5 ta-rust sesuai dengan TA-Lib.

## 📁 File Structure

```
test/
├── README.md                    # Dokumentasi ini
├── run_tests.sh                 # Script utama untuk menjalankan semua test
├── simple_comparison.py         # Generate reference data dari TA-Lib
├── rust_comparison_test.rs      # Test kompatibilitas Rust vs TA-Lib
├── rust_test_runner.rs          # Helper untuk testing individual functions
├── test_ta_rust_vs_talib.py     # Advanced testing suite (kompleks)
├── talib_reference_data.json    # Data referensi dari TA-Lib (generated)
└── rust_test_results.json       # Hasil testing detail (generated)
```

## 🚀 Quick Start

### Prerequisites

1. **Conda Environment dengan TA-Lib**:
   ```bash
   conda create -n talib-env python=3.9
   conda activate talib-env
   conda install -c conda-forge ta-lib
   pip install numpy pandas
   ```

2. **Rust dan Cargo** (sudah terinstall)

### Menjalankan Semua Test

Jalankan script utama yang akan melakukan semua testing secara otomatis:

```bash
# Dari root directory ta-rust
chmod +x test/run_tests.sh
./test/run_tests.sh
```

Script ini akan:
1. ✅ Build ta-rust
2. ✅ Run unit tests Rust yang sudah ada
3. ✅ Generate reference data dari TA-Lib
4. ✅ Compile dan run comparison test
5. ✅ Generate laporan komprehensif

## 📋 Testing Components

### 1. Simple Comparison (`simple_comparison.py`)

Script Python yang menggunakan TA-Lib untuk generate reference data.

```bash
conda activate talib-env
python test/simple_comparison.py
```

**Output**: `test/talib_reference_data.json`

**Fungsi yang ditest**:
- **Phase 2**: SMA, EMA, WMA, DEMA, TEMA, TRIMA
- **Phase 3**: RSI, ROC, MOM, ATR, NATR, TRANGE
- **Phase 4**: MACD
- **Phase 5**: OBV, AD, Bollinger Bands, Parabolic SAR

### 2. Rust Comparison Test (`rust_comparison_test.rs`)

Program Rust yang membandingkan hasil ta-rust dengan reference data TA-Lib.

```bash
# Compile dan run
rustc --edition 2021 \
    -L target/release/deps \
    --extern ta_rust=target/release/libta_rust.rlib \
    --extern serde_json=target/release/deps/libserde_json-*.rlib \
    test/rust_comparison_test.rs -o test/rust_comparison_test

./test/rust_comparison_test
```

**Output**: `test/rust_test_results.json`

### 3. Advanced Testing Suite (`test_ta_rust_vs_talib.py`)

Testing suite yang lebih kompleks dengan subprocess handling untuk testing yang lebih mendalam.

```bash
conda activate talib-env
python test/test_ta_rust_vs_talib.py
```

## 📊 Test Results

### Success Criteria

Test dianggap **PASS** jika:
- ✅ Max error ≤ 1e-8 (tolerance sangat ketat)
- ✅ Pattern NaN sama persis dengan TA-Lib
- ✅ Length array hasil sama
- ✅ Tidak ada runtime error

### Example Output

```
🚀 Starting TA-Rust vs TA-Lib Compatibility Tests
📊 Tolerance: 1.00e-08

🔍 Testing Phase 2: Overlap Studies
✅ sma_14: Max error 2.84e-14
✅ ema_14: Max error 1.42e-13
✅ wma_14: Max error 5.68e-14

🔍 Testing Phase 3-4: Momentum Indicators
✅ rsi_14: Max error 3.55e-12
✅ roc_10: Max error 1.78e-14
✅ mom_10: Max error 0.00e+00

================================================================================
📋 TEST REPORT SUMMARY
================================================================================
Total Tests: 15
Passed: 15 ✅
Failed: 0 ❌
Success Rate: 100.0%

🎉 All tests passed! TA-Rust is 100% compatible with TA-Lib!
```

## 🔧 Manual Testing

### Testing Individual Functions

```bash
# Build ta-rust first
cargo build --release

# Test specific function
rustc --edition 2021 \
    -L target/release/deps \
    --extern ta_rust=target/release/libta_rust.rlib \
    test/rust_test_runner.rs -o test/rust_test_runner

./test/rust_test_runner sma
./test/rust_test_runner rsi
./test/rust_test_runner atr
```

### Generate Reference Data Only

```bash
conda activate talib-env
python test/simple_comparison.py
```

### Run Rust Tests Only

```bash
# Setelah reference data tersedia
cargo build --release
./test/rust_comparison_test
```

## 📈 Tested Functions by Phase

### ✅ Phase 2: Overlap Studies (6 functions)
- `SMA` - Simple Moving Average
- `EMA` - Exponential Moving Average  
- `WMA` - Weighted Moving Average
- `DEMA` - Double Exponential Moving Average
- `TEMA` - Triple Exponential Moving Average
- `TRIMA` - Triangular Moving Average

### ✅ Phase 3: Volatility & Basic Momentum (6 functions)
- `RSI` - Relative Strength Index
- `ROC` - Rate of Change
- `MOM` - Momentum
- `ATR` - Average True Range
- `NATR` - Normalized Average True Range
- `TRANGE` - True Range

### ✅ Phase 4: Advanced Momentum (1 function)
- `MACD` - Moving Average Convergence/Divergence

### ✅ Phase 5: Volume & Advanced Overlays (4 functions)
- `OBV` - On Balance Volume
- `AD` - Chaikin A/D Line
- `BBANDS` - Bollinger Bands
- `SAR` - Parabolic SAR

**Total: 17 core functions tested**

## 🐛 Troubleshooting

### Common Issues

1. **TA-Lib not found**:
   ```bash
   conda activate talib-env
   conda install -c conda-forge ta-lib
   ```

2. **Compilation errors**:
   ```bash
   cargo clean
   cargo build --release
   ```

3. **Permission denied**:
   ```bash
   chmod +x test/run_tests.sh
   ```

4. **Missing dependencies**:
   ```bash
   pip install numpy pandas serde_json
   ```

### Debug Mode

Untuk debugging yang lebih detail:

```bash
# Run dengan verbose output
RUST_LOG=debug ./test/rust_comparison_test

# Check individual function
python -c "
import talib
import numpy as np
data = [1,2,3,4,5,6,7,8,9,10]
print('TA-Lib SMA:', talib.SMA(np.array(data), 3))
"
```

## 📝 Adding New Tests

### Untuk menambah function baru:

1. **Update `simple_comparison.py`**:
   ```python
   # Tambah di save_reference_data()
   'new_function': talib.NEW_FUNCTION(close, timeperiod=14).tolist(),
   ```

2. **Update `rust_comparison_test.rs`**:
   ```rust
   // Tambah di test function yang sesuai
   let new_result = new_function(close, 14);
   self.test_function("new_function", phase, "Category", new_result);
   ```

3. **Run tests**:
   ```bash
   ./test/run_tests.sh
   ```

## 📊 Performance Benchmarks

Testing suite juga mengukur performa:

- **Accuracy**: Error maksimal vs TA-Lib
- **Speed**: Waktu eksekusi (future enhancement)
- **Memory**: Penggunaan memori (future enhancement)

## 🎯 Goals

Testing suite ini memastikan:

1. **100% Compatibility**: Hasil identik dengan TA-Lib
2. **Numerical Accuracy**: Error < 1e-8 untuk semua fungsi
3. **Edge Case Handling**: NaN, infinite values, empty arrays
4. **Performance**: Tidak lebih lambat dari TA-Lib
5. **Reliability**: Konsisten di berbagai platform

## 📞 Support

Jika ada masalah dengan testing:

1. Check file `test/rust_test_results.json` untuk detail error
2. Pastikan environment setup benar
3. Jalankan individual tests untuk isolasi masalah
4. Check implementasi function yang gagal di `src/`

---

**Happy Testing! 🚀**

Testing suite ini adalah jaminan kualitas bahwa ta-rust benar-benar 100% kompatibel dengan TA-Lib original.
