# Rumusan Matematika TA-Lib (Bagian 2)

## Pattern Recognition (Lanjutan)

### CDLHARAMICROSS - Harami Cross Pattern
```
Bullish Harami Cross:
1. Long black candle in downtrend
2. Doji within first body
Pattern = 100

Bearish Harami Cross:
1. Long white candle in uptrend  
2. Doji within first body
Pattern = -100
```

### CDLHIGHWAVE - High-Wave Candle
```
Conditions:
1. Small body
2. Very long upper shadow
3. Very long lower shadow
Body/Range ratio < 0.1
Shadow/Body ratio > 3
```

### CDLHIKKAKE - Hikkake Pattern
```
Bullish:
1. Inside bar setup
2. False breakout to downside
3. Price breaks above inside bar high

Bearish:
1. Inside bar setup
2. False breakout to upside
3. Price breaks below inside bar low
```

### CDLHIKKAKEMOD - Modified Hikkake Pattern
```
Extended version of Hikkake with additional confirmation:
1. Inside bar
2. False breakout
3. Confirmation bar in opposite direction
4. Continuation in trend direction
```

### CDLHOMINGPIGEON - Homing Pigeon
```
Conditions:
1. Two black candles in downtrend
2. Second black body within first
3. Both close near their lows
Pattern = 100 (Bullish reversal)
```

### CDLIDENTICAL3CROWS - Identical Three Crows
```
Conditions:
1. Three identical black candles
2. Each opens at or near previous close
3. Each closes at or near its low
Pattern = -100 (Bearish continuation)
```

### CDLINNECK - In-Neck Pattern
```
Bearish In-Neck:
1. Black candle in downtrend
2. Small white candle
3. White closes at or slightly above black close
Pattern = -100 (Bearish continuation)
```

### CDLINVERTEDHAMMER - Inverted Hammer
```
Conditions:
1. Small body at lower range
2. Upper shadow >= 2 × body
3. Little/no lower shadow
4. Appears in downtrend
Pattern = 100 (Bullish reversal)
```

### CDLKICKING - Kicking
```
Bullish Kicking:
1. Black marubozu
2. White marubozu with gap up

Bearish Kicking:
1. White marubozu
2. Black marubozu with gap down
```

### CDLKICKINGBYLENGTH - Kicking by Length
```
Same as Kicking but:
- Direction determined by longer marubozu
- If second candle longer = stronger signal
```

### CDLLADDERBOTTOM - Ladder Bottom
```
Conditions:
1. Three consecutive black candles
2. Fourth black with upper shadow
3. Fifth white opens above fourth body
Pattern = 100 (Bullish reversal)
```

### CDLLONGLEGGEDDOJI - Long Legged Doji
```
Conditions:
1. Open ≈ Close
2. Long upper shadow
3. Long lower shadow
4. Shadows approximately equal
```

### CDLLONGLINE - Long Line Candle
```
Conditions:
Body/Range ratio > 0.6
Long body relative to recent candles
```

### CDLMARUBOZU - Marubozu
```
White Marubozu: Open = Low, Close = High
Black Marubozu: Open = High, Close = Low
No shadows
```

### CDLMATCHINGLOW - Matching Low
```
Conditions:
1. First black candle in downtrend
2. Second black candle
3. Both close at same price
Pattern = 100 (Bullish reversal)
```

### CDLMATHOLD - Mat Hold
```
Bullish Mat Hold:
1. Long white candle
2. Gap up, small candle
3. Two small candles within first range
4. White candle closing above first

Pattern = 100 (Bullish continuation)
```

### CDLMORNINGDOJISTAR - Morning Doji Star
```
Conditions:
1. Long black candle
2. Doji with gap down
3. White candle closing above first midpoint
Pattern = 100 (Bullish reversal)
```

### CDLMORNINGSTAR - Morning Star
```
Conditions:
1. Long black candle
2. Small body with gap down
3. White candle closing above first midpoint
Pattern = 100 (Bullish reversal)
```

### CDLONNECK - On-Neck Pattern
```
Bearish On-Neck:
1. Black candle in downtrend
2. Small white candle
3. White closes at black low
Pattern = -100 (Bearish continuation)
```

### CDLPIERCING - Piercing Pattern
```
Conditions:
1. Black candle in downtrend
2. White opens below first low
3. White closes above first midpoint
Pattern = 100 (Bullish reversal)
```

### CDLRICKSHAWMAN - Rickshaw Man
```
Similar to Long Legged Doji:
1. Open ≈ Close (near center)
2. Long shadows both sides
3. Small body at center of range
```

### CDLRISEFALL3METHODS - Rising/Falling Three Methods
```
Rising Three Methods:
1. Long white candle
2. Three small black candles within first range
3. White candle closing above first

Falling Three Methods:
1. Long black candle
2. Three small white candles within first range
3. Black candle closing below first
```

### CDLSEPARATINGLINES - Separating Lines
```
Bullish:
1. Black candle in downtrend
2. White opens at first open, closes higher

Bearish:
1. White candle in uptrend
2. Black opens at first open, closes lower
```

### CDLSHOOTINGSTAR - Shooting Star
```
Conditions:
1. Small body at lower range
2. Upper shadow >= 2 × body
3. Little/no lower shadow
4. Appears in uptrend after gap up
Pattern = -100 (Bearish reversal)
```

### CDLSHORTLINE - Short Line Candle
```
Conditions:
Small body relative to recent candles
Body/Average body < 0.5
```

### CDLSPINNINGTOP - Spinning Top
```
Conditions:
1. Small body
2. Upper shadow > body
3. Lower shadow > body
4. Shadows relatively equal
```

### CDLSTALLEDPATTERN - Stalled Pattern
```
Conditions:
1. Long white candle
2. White candle with smaller gain
3. Small white/spinning top near previous close
Pattern = -100 (Bearish reversal warning)
```

### CDLSTICKSANDWICH - Stick Sandwich
```
Conditions:
1. Black candle
2. White candle closing higher
3. Black candle closing at first close
Pattern = 100 (Bullish reversal)
```

### CDLTAKURI - Takuri
```
Conditions:
1. Dragonfly Doji in downtrend
2. Open = High = Close
3. Very long lower shadow (>3 × average)
Pattern = 100 (Bullish reversal)
```

### CDLTASUKIGAP - Tasuki Gap
```
Upward Tasuki Gap:
1. White candle
2. White with gap up
3. Black opens within gap, closes in gap

Downward Tasuki Gap:
1. Black candle
2. Black with gap down
3. White opens within gap, closes in gap
```

### CDLTHRUSTING - Thrusting Pattern
```
Conditions:
1. Black candle in downtrend
2. White opens below first low
3. White closes below first midpoint
Pattern = -100 (Bearish continuation)
```

### CDLTRISTAR - Tristar Pattern
```
Bullish Tristar:
1. Doji
2. Doji with gap down
3. Doji with gap up

Bearish Tristar:
1. Doji
2. Doji with gap up
3. Doji with gap down
```

### CDLUNIQUE3RIVER - Unique 3 River
```
Conditions:
1. Long black candle
2. Black hammer closing higher
3. Small white candle below second
Pattern = 100 (Bullish reversal)
```

### CDLUPSIDEGAP2CROWS - Upside Gap Two Crows
```
Conditions:
1. White candle in uptrend
2. Black with gap up
3. Black engulfing second, body in gap
Pattern = -100 (Bearish reversal)
```

### CDLXSIDEGAP3METHODS - Upside/Downside Gap Three Methods
```
Upside Gap Three Methods:
1. Two white candles with gap
2. Black candle filling gap

Downside Gap Three Methods:
1. Two black candles with gap
2. White candle filling gap
```

## Statistic Functions (Fungsi Statistik)

### BETA - Beta
**Input:** Price1 series, Price2 series, Period (n)  
**Output:** Beta coefficient

```
β = Covariance(Price1, Price2) / Variance(Price2)

Where:
Covariance = Σ((Price1[i] - Mean1) × (Price2[i] - Mean2)) / (n-1)
Variance = Σ((Price2[i] - Mean2)²) / (n-1)
```

### CORREL - Pearson's Correlation Coefficient
**Input:** Price1 series, Price2 series, Period (n)  
**Output:** Correlation coefficient

```
r = Σ((x[i] - x̄)(y[i] - ȳ)) / √(Σ(x[i] - x̄)² × Σ(y[i] - ȳ)²)

Where x̄ and ȳ are means of x and y
Range: -1 to +1
```

### LINEARREG - Linear Regression
**Input:** Price series, Period (n)  
**Output:** Linear regression line value

```
y = mx + b

Where:
m = LINEARREG_SLOPE
b = LINEARREG_INTERCEPT
Current value = m × (n-1) + b
```

### LINEARREG_ANGLE - Linear Regression Angle
**Input:** Price series, Period (n)  
**Output:** Angle in degrees

```
Slope = LINEARREG_SLOPE
Angle = arctan(Slope) × (180/π)
```

### LINEARREG_INTERCEPT - Linear Regression Intercept
**Input:** Price series, Period (n)  
**Output:** Y-intercept

```
b = ȳ - m × x̄

Where:
ȳ = mean of prices
x̄ = mean of x values (0 to n-1)
m = slope
```

### LINEARREG_SLOPE - Linear Regression Slope
**Input:** Price series, Period (n)  
**Output:** Slope

```
m = Σ((x[i] - x̄)(y[i] - ȳ)) / Σ((x[i] - x̄)²)

Where:
x[i] = i (time index)
y[i] = Price[i]
```

### STDDEV - Standard Deviation
**Input:** Price series, Period (n), Deviations  
**Output:** Standard deviation

```
σ = √(Σ(Price[i] - Mean)² / n)
Result = σ × Deviations
```

### TSF - Time Series Forecast
**Input:** Price series, Period (n)  
**Output:** Forecast value

```
TSF = LINEARREG + LINEARREG_SLOPE
(Projects linear regression one period forward)
```

### VAR - Variance
**Input:** Price series, Period (n)  
**Output:** Variance

```
σ² = Σ(Price[i] - Mean)² / n
```

## Math Transform Functions

### ACOS - Vector Trigonometric ACos
```
Output[i] = arccos(Input[i])
Range: Input must be [-1, 1]
Output in radians [0, π]
```

### ASIN - Vector Trigonometric ASin
```
Output[i] = arcsin(Input[i])
Range: Input must be [-1, 1]
Output in radians [-π/2, π/2]
```

### ATAN - Vector Trigonometric ATan
```
Output[i] = arctan(Input[i])
Output in radians [-π/2, π/2]
```

### CEIL - Vector Ceil
```
Output[i] = ⌈Input[i]⌉
(Smallest integer >= Input[i])
```

### COS - Vector Trigonometric Cos
```
Output[i] = cos(Input[i])
Input in radians
```

### COSH - Vector Trigonometric Cosh
```
Output[i] = cosh(Input[i])
= (e^Input[i] + e^-Input[i]) / 2
```

### EXP - Vector Arithmetic Exp
```
Output[i] = e^Input[i]
```

### FLOOR - Vector Floor
```
Output[i] = ⌊Input[i]⌋
(Largest integer <= Input[i])
```

### LN - Vector Log Natural
```
Output[i] = ln(Input[i])
Input must be > 0
```

### LOG10 - Vector Log10
```
Output[i] = log₁₀(Input[i])
Input must be > 0
```

### SIN - Vector Trigonometric Sin
```
Output[i] = sin(Input[i])
Input in radians
```

### SINH - Vector Trigonometric Sinh
```
Output[i] = sinh(Input[i])
= (e^Input[i] - e^-Input[i]) / 2
```

### SQRT - Vector Square Root
```
Output[i] = √Input[i]
Input must be >= 0
```

### TAN - Vector Trigonometric Tan
```
Output[i] = tan(Input[i])
Input in radians
```

### TANH - Vector Trigonometric Tanh
```
Output[i] = tanh(Input[i])
= (e^Input[i] - e^-Input[i]) / (e^Input[i] + e^-Input[i])
```

## Math Operator Functions

### ADD - Vector Arithmetic Add
```
Output[i] = Input1[i] + Input2[i]
```

### DIV - Vector Arithmetic Div
```
Output[i] = Input1[i] / Input2[i]
Note: Handle division by zero
```

### MAX - Highest value over period
```
MAX = max(Input[i]) for i in period n
```

### MAXINDEX - Index of highest value
```
Returns the index (bars ago) of highest value in period n
```

### MIN - Lowest value over period
```
MIN = min(Input[i]) for i in period n
```

### MININDEX - Index of lowest value
```
Returns the index (bars ago) of lowest value in period n
```

### MINMAX - Lowest and highest values
```
Returns both MIN and MAX over period n
```

### MINMAXINDEX - Indexes of lowest and highest
```
Returns both MININDEX and MAXINDEX over period n
```

### MULT - Vector Arithmetic Mult
```
Output[i] = Input1[i] × Input2[i]
```

### SUB - Vector Arithmetic Subtraction
```
Output[i] = Input1[i] - Input2[i]
```

### SUM - Summation
```
SUM = Σ(Input[i]) for i in period n
```

## Important Implementation Notes

### EMA Implementation
```rust
// Standard EMA
alpha = 2.0 / (period + 1.0)
ema[0] = first_value
for i in 1..len {
    ema[i] = price[i] * alpha + ema[i-1] * (1.0 - alpha)
}

// Wilder's EMA (for RSI, ATR, ADX)
alpha = 1.0 / period
```

### Handling Unstable Period
Many indicators need a "warm-up" period before producing valid results:
- SMA: needs `period` values
- EMA: needs `2 * period - 1` values for stability
- MACD: needs `slow_period + signal_period - 1`
- ADX: needs `2 * period - 1`

### NaN and Division by Zero
Always check for:
- Division by zero in ratios
- Negative values in SQRT, LOG
- Out of range [-1, 1] for ACOS, ASIN
- Zero volume in volume indicators

### Precision Considerations
- Use f64 for all calculations
- Be careful with equality comparisons for Doji patterns
- Use small epsilon for float comparisons

### Pattern Recognition Thresholds
```rust
// Example thresholds
const DOJI_BODY_RATIO: f64 = 0.1;  // Body/Range < 10%
const LONG_SHADOW_RATIO: f64 = 2.0; // Shadow >= 2 × Body
const EQUAL_THRESHOLD: f64 = 0.001;  // For "equal" prices
```

### Performance Optimization
- Pre-allocate arrays
- Use rolling calculations where possible
- Cache frequently used values (e.g., EMA for MACD)
- Implement incremental updates for real-time data

### TA-Lib Compatibility
To ensure compatibility with original TA-Lib:
1. Use same unstable period calculations
2. Match the initialization methods
3. Follow same conventions for invalid outputs (NaN)
4. Use identical parameter defaults
5. Implement same MA type enumerations
