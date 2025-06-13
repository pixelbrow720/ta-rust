# Rumusan Matematika TA-Lib

## Overlap Studies (Studi Overlap)

### BBANDS - Bollinger Bands
**Input:** Close prices, Period (n), Standard Deviations (k)  
**Output:** Upper Band, Middle Band, Lower Band

```
Middle Band = SMA(Close, n)
Standard Deviation = √(Σ(Close[i] - Middle Band)² / n)
Upper Band = Middle Band + (k × Standard Deviation)
Lower Band = Middle Band - (k × Standard Deviation)
```

### DEMA - Double Exponential Moving Average
**Input:** Price series, Period (n)  
**Output:** DEMA values

```
EMA1 = EMA(Price, n)
EMA2 = EMA(EMA1, n)
DEMA = 2 × EMA1 - EMA2
```

### EMA - Exponential Moving Average
**Input:** Price series, Period (n)  
**Output:** EMA values

```
Multiplier (α) = 2 / (n + 1)
EMA[today] = (Price[today] × α) + (EMA[yesterday] × (1 - α))

Initial EMA = SMA of first n periods
```

### HT_TRENDLINE - Hilbert Transform - Instantaneous Trendline
**Input:** Price series  
**Output:** Trendline values

```
Uses Hilbert Transform to decompose price into:
- In-phase component (I)
- Quadrature component (Q)

Trendline = WMA(Price, Period) where Period is dynamically calculated
Period = 2π / Dominant Cycle Phase
```

### KAMA - Kaufman Adaptive Moving Average
**Input:** Price series, Period (n)  
**Output:** KAMA values

```
Direction = |Close[today] - Close[n periods ago]|
Volatility = Σ|Close[i] - Close[i-1]| for n periods
Efficiency Ratio (ER) = Direction / Volatility

Fast SC = 2 / (2 + 1) = 0.6667
Slow SC = 2 / (30 + 1) = 0.0645
SC = (ER × (Fast SC - Slow SC) + Slow SC)²

KAMA[today] = KAMA[yesterday] + SC × (Price[today] - KAMA[yesterday])
```

### MA - Moving Average
**Input:** Price series, Period (n), MA Type  
**Output:** MA values

```
Supports multiple types:
- SMA: Simple Moving Average
- EMA: Exponential Moving Average
- WMA: Weighted Moving Average
- DEMA: Double Exponential Moving Average
- TEMA: Triple Exponential Moving Average
- TRIMA: Triangular Moving Average
- KAMA: Kaufman Adaptive Moving Average
- MAMA: MESA Adaptive Moving Average
- T3: Triple Exponential Moving Average (T3)
```

### MAMA - MESA Adaptive Moving Average
**Input:** Price series, Fast Limit, Slow Limit  
**Output:** MAMA, FAMA values

```
Uses Hilbert Transform to calculate adaptive period
Smoothing Factor = adaptive based on dominant cycle
MAMA = α × Price + (1 - α) × MAMA[prev]
FAMA = 0.5 × α × MAMA + (1 - 0.5 × α) × FAMA[prev]
```

### MAVP - Moving Average with Variable Period
**Input:** Price series, Variable periods array, Min period, Max period  
**Output:** MA values with variable period

```
For each point i:
    period = periods[i] (bounded by min and max)
    MA[i] = SMA(Price, period) at point i
```

### MIDPOINT - MidPoint over period
**Input:** Price series, Period (n)  
**Output:** Midpoint values

```
Midpoint = (Highest(Price, n) + Lowest(Price, n)) / 2
```

### MIDPRICE - Midpoint Price over period
**Input:** High, Low series, Period (n)  
**Output:** Midprice values

```
Midprice = (Highest(High, n) + Lowest(Low, n)) / 2
```

### SAR - Parabolic SAR
**Input:** High, Low series, Acceleration Factor (AF), Max AF  
**Output:** SAR values

```
Initial SAR = Low[0] for uptrend, High[0] for downtrend
EP (Extreme Point) = highest high in uptrend, lowest low in downtrend

SAR[tomorrow] = SAR[today] + AF × (EP - SAR[today])

AF starts at 0.02, increases by 0.02 when new EP, max at 0.20
Reverse when price crosses SAR
```

### SAREXT - Parabolic SAR Extended
**Input:** High, Low, Start value, Offset on reverse, AF init, AF increment, AF max, etc.  
**Output:** SAR values

```
Extended version of SAR with more parameters:
- Custom start value
- Offset on reverse
- Custom AF initial, increment, and max values
- Different AF for long and short
```

### SMA - Simple Moving Average
**Input:** Price series, Period (n)  
**Output:** SMA values

```
SMA = Σ(Price[i]) / n, for i = 0 to n-1
```

### T3 - Triple Exponential Moving Average (T3)
**Input:** Price series, Period (n), Volume Factor (v)  
**Output:** T3 values

```
c1 = -v³
c2 = 3v² + 3v³
c3 = -6v² - 3v - 3v³
c4 = 1 + 3v + v³ + 3v²

e1 = EMA(Price, n)
e2 = EMA(e1, n)
e3 = EMA(e2, n)
e4 = EMA(e3, n)
e5 = EMA(e4, n)
e6 = EMA(e5, n)

T3 = c1×e6 + c2×e5 + c3×e4 + c4×e3
```

### TEMA - Triple Exponential Moving Average
**Input:** Price series, Period (n)  
**Output:** TEMA values

```
EMA1 = EMA(Price, n)
EMA2 = EMA(EMA1, n)
EMA3 = EMA(EMA2, n)
TEMA = 3×EMA1 - 3×EMA2 + EMA3
```

### TRIMA - Triangular Moving Average
**Input:** Price series, Period (n)  
**Output:** TRIMA values

```
If n is odd:
    TRIMA = SMA(SMA(Price, (n+1)/2), (n+1)/2)
If n is even:
    TRIMA = SMA(SMA(Price, n/2+1), n/2)
```

### WMA - Weighted Moving Average
**Input:** Price series, Period (n)  
**Output:** WMA values

```
WMA = Σ(Price[i] × Weight[i]) / Σ(Weight[i])
Where Weight[i] = n - i, for i = 0 to n-1
```

## Momentum Indicators (Indikator Momentum)

### ADX - Average Directional Movement Index
**Input:** High, Low, Close, Period (n)  
**Output:** ADX values

```
TR = max(High - Low, |High - Close[prev]|, |Low - Close[prev]|)
+DM = High - High[prev] if > 0 and > (Low[prev] - Low), else 0
-DM = Low[prev] - Low if > 0 and > (High - High[prev]), else 0

ATR = EMA(TR, n)
+DI = 100 × EMA(+DM, n) / ATR
-DI = 100 × EMA(-DM, n) / ATR

DX = 100 × |+DI - -DI| / (+DI + -DI)
ADX = EMA(DX, n)
```

### ADXR - Average Directional Movement Index Rating
**Input:** High, Low, Close, Period (n)  
**Output:** ADXR values

```
ADXR = (ADX[today] + ADX[n periods ago]) / 2
```

### APO - Absolute Price Oscillator
**Input:** Price series, Fast Period, Slow Period, MA Type  
**Output:** APO values

```
APO = MA(Price, Fast Period) - MA(Price, Slow Period)
```

### AROON - Aroon
**Input:** High, Low, Period (n)  
**Output:** Aroon Up, Aroon Down

```
Aroon Up = 100 × (n - periods since n-period high) / n
Aroon Down = 100 × (n - periods since n-period low) / n
```

### AROONOSC - Aroon Oscillator
**Input:** High, Low, Period (n)  
**Output:** Aroon Oscillator values

```
Aroon Oscillator = Aroon Up - Aroon Down
```

### BOP - Balance Of Power
**Input:** Open, High, Low, Close  
**Output:** BOP values

```
BOP = (Close - Open) / (High - Low)
```

### CCI - Commodity Channel Index
**Input:** High, Low, Close, Period (n)  
**Output:** CCI values

```
TP (Typical Price) = (High + Low + Close) / 3
SMA = Simple Moving Average of TP over n periods
MAD = Mean Absolute Deviation = Σ|TP[i] - SMA| / n
CCI = (TP - SMA) / (0.015 × MAD)
```

### CMO - Chande Momentum Oscillator
**Input:** Price series, Period (n)  
**Output:** CMO values

```
Sum of positive changes = Σ(max(Price[i] - Price[i-1], 0))
Sum of negative changes = Σ(abs(min(Price[i] - Price[i-1], 0)))
CMO = 100 × (Sum Up - Sum Down) / (Sum Up + Sum Down)
```

### DX - Directional Movement Index
**Input:** High, Low, Close, Period (n)  
**Output:** DX values

```
+DM = High - High[prev] if > 0 and > (Low[prev] - Low), else 0
-DM = Low[prev] - Low if > 0 and > (High - High[prev]), else 0
TR = max(High - Low, |High - Close[prev]|, |Low - Close[prev]|)

+DI = 100 × EMA(+DM, n) / EMA(TR, n)
-DI = 100 × EMA(-DM, n) / EMA(TR, n)

DX = 100 × |+DI - -DI| / (+DI + -DI)
```

### MACD - Moving Average Convergence/Divergence
**Input:** Price series, Fast Period (12), Slow Period (26), Signal Period (9)  
**Output:** MACD, MACD Signal, MACD Histogram

```
MACD Line = EMA(Price, 12) - EMA(Price, 26)
Signal Line = EMA(MACD Line, 9)
MACD Histogram = MACD Line - Signal Line
```

### MACDEXT - MACD with controllable MA type
**Input:** Price, Fast Period, Fast MA Type, Slow Period, Slow MA Type, Signal Period, Signal MA Type  
**Output:** MACD, Signal, Histogram

```
MACD = MA(Price, Fast Period, Fast MA Type) - MA(Price, Slow Period, Slow MA Type)
Signal = MA(MACD, Signal Period, Signal MA Type)
Histogram = MACD - Signal
```

### MACDFIX - Moving Average Convergence/Divergence Fix 12/26
**Input:** Price series, Signal Period (9)  
**Output:** MACD, Signal, Histogram

```
MACD = EMA(Price, 12) - EMA(Price, 26)
Signal = EMA(MACD, Signal Period)
Histogram = MACD - Signal
```

### MFI - Money Flow Index
**Input:** High, Low, Close, Volume, Period (n)  
**Output:** MFI values

```
TP = (High + Low + Close) / 3
MF = TP × Volume

Positive MF = Sum of MF when TP > TP[prev]
Negative MF = Sum of MF when TP < TP[prev]

MFR = Positive MF / Negative MF
MFI = 100 - (100 / (1 + MFR))
```

### MINUS_DI - Minus Directional Indicator
**Input:** High, Low, Close, Period (n)  
**Output:** -DI values

```
-DM = Low[prev] - Low if > 0 and > (High - High[prev]), else 0
TR = max(High - Low, |High - Close[prev]|, |Low - Close[prev]|)
-DI = 100 × EMA(-DM, n) / EMA(TR, n)
```

### MINUS_DM - Minus Directional Movement
**Input:** High, Low, Period (n)  
**Output:** -DM values

```
-DM = Low[prev] - Low if > 0 and > (High - High[prev]), else 0
Smoothed -DM = EMA(-DM, n)
```

### MOM - Momentum
**Input:** Price series, Period (n)  
**Output:** Momentum values

```
MOM = Price[today] - Price[n periods ago]
```

### PLUS_DI - Plus Directional Indicator
**Input:** High, Low, Close, Period (n)  
**Output:** +DI values

```
+DM = High - High[prev] if > 0 and > (Low[prev] - Low), else 0
TR = max(High - Low, |High - Close[prev]|, |Low - Close[prev]|)
+DI = 100 × EMA(+DM, n) / EMA(TR, n)
```

### PLUS_DM - Plus Directional Movement
**Input:** High, Low, Period (n)  
**Output:** +DM values

```
+DM = High - High[prev] if > 0 and > (Low[prev] - Low), else 0
Smoothed +DM = EMA(+DM, n)
```

### PPO - Percentage Price Oscillator
**Input:** Price series, Fast Period, Slow Period, MA Type  
**Output:** PPO values

```
PPO = 100 × (MA(Price, Fast) - MA(Price, Slow)) / MA(Price, Slow)
```

### ROC - Rate of Change
**Input:** Price series, Period (n)  
**Output:** ROC values

```
ROC = ((Price[today] / Price[n periods ago]) - 1) × 100
```

### ROCP - Rate of Change Percentage
**Input:** Price series, Period (n)  
**Output:** ROCP values

```
ROCP = (Price[today] - Price[n periods ago]) / Price[n periods ago]
```

### ROCR - Rate of Change Ratio
**Input:** Price series, Period (n)  
**Output:** ROCR values

```
ROCR = Price[today] / Price[n periods ago]
```

### ROCR100 - Rate of Change Ratio 100 scale
**Input:** Price series, Period (n)  
**Output:** ROCR100 values

```
ROCR100 = (Price[today] / Price[n periods ago]) × 100
```

### RSI - Relative Strength Index
**Input:** Price series, Period (n)  
**Output:** RSI values

```
Gain = Price[today] - Price[yesterday] if > 0, else 0
Loss = |Price[today] - Price[yesterday]| if < 0, else 0

Avg Gain = EMA(Gain, n) using Wilder's smoothing (α = 1/n)
Avg Loss = EMA(Loss, n) using Wilder's smoothing (α = 1/n)

RS = Avg Gain / Avg Loss
RSI = 100 - (100 / (1 + RS))
```

### STOCH - Stochastic
**Input:** High, Low, Close, FastK Period, SlowK Period, SlowK MA Type, SlowD Period, SlowD MA Type  
**Output:** SlowK, SlowD

```
FastK = 100 × (Close - Lowest Low) / (Highest High - Lowest Low)
SlowK = MA(FastK, SlowK Period, SlowK MA Type)
SlowD = MA(SlowK, SlowD Period, SlowD MA Type)
```

### STOCHF - Stochastic Fast
**Input:** High, Low, Close, FastK Period, FastD Period, FastD MA Type  
**Output:** FastK, FastD

```
FastK = 100 × (Close - Lowest Low) / (Highest High - Lowest Low)
FastD = MA(FastK, FastD Period, FastD MA Type)
```

### STOCHRSI - Stochastic Relative Strength Index
**Input:** Price series, Period, FastK Period, FastD Period, FastD MA Type  
**Output:** FastK, FastD

```
RSI = Calculate RSI(Price, Period)
StochRSI = (RSI - Lowest RSI) / (Highest RSI - Lowest RSI)
FastK = 100 × StochRSI
FastD = MA(FastK, FastD Period, FastD MA Type)
```

### TRIX - 1-day Rate-Of-Change of Triple Smooth EMA
**Input:** Price series, Period (n)  
**Output:** TRIX values

```
EMA1 = EMA(Price, n)
EMA2 = EMA(EMA1, n)
EMA3 = EMA(EMA2, n)
TRIX = 10000 × (EMA3[today] - EMA3[yesterday]) / EMA3[yesterday]
```

### ULTOSC - Ultimate Oscillator
**Input:** High, Low, Close, Period1 (7), Period2 (14), Period3 (28)  
**Output:** Ultimate Oscillator values

```
BP (Buying Pressure) = Close - min(Low, Close[prev])
TR (True Range) = max(High, Close[prev]) - min(Low, Close[prev])

Average1 = Sum(BP, Period1) / Sum(TR, Period1)
Average2 = Sum(BP, Period2) / Sum(TR, Period2)
Average3 = Sum(BP, Period3) / Sum(TR, Period3)

UO = 100 × ((4 × Average1) + (2 × Average2) + Average3) / 7
```

### WILLR - Williams' %R
**Input:** High, Low, Close, Period (n)  
**Output:** Williams' %R values

```
%R = -100 × (Highest High - Close) / (Highest High - Lowest Low)
```

## Volume Indicators (Indikator Volume)

### AD - Chaikin A/D Line
**Input:** High, Low, Close, Volume  
**Output:** A/D Line values

```
CLV (Close Location Value) = ((Close - Low) - (High - Close)) / (High - Low)
AD = AD[prev] + (CLV × Volume)
```

### ADOSC - Chaikin A/D Oscillator
**Input:** High, Low, Close, Volume, Fast Period (3), Slow Period (10)  
**Output:** A/D Oscillator values

```
AD = Chaikin A/D Line
ADOSC = EMA(AD, Fast Period) - EMA(AD, Slow Period)
```

### OBV - On Balance Volume
**Input:** Close, Volume  
**Output:** OBV values

```
If Close > Close[prev]: OBV = OBV[prev] + Volume
If Close < Close[prev]: OBV = OBV[prev] - Volume
If Close = Close[prev]: OBV = OBV[prev]
```

## Volatility Indicators (Indikator Volatilitas)

### ATR - Average True Range
**Input:** High, Low, Close, Period (n)  
**Output:** ATR values

```
TR = max(High - Low, |High - Close[prev]|, |Low - Close[prev]|)
ATR = EMA(TR, n) using Wilder's smoothing (α = 1/n)
```

### NATR - Normalized Average True Range
**Input:** High, Low, Close, Period (n)  
**Output:** NATR values

```
ATR = Average True Range
NATR = 100 × ATR / Close
```

### TRANGE - True Range
**Input:** High, Low, Close  
**Output:** True Range values

```
TR = max(High - Low, |High - Close[prev]|, |Low - Close[prev]|)
```

## Price Transform (Transformasi Harga)

### AVGPRICE - Average Price
**Input:** Open, High, Low, Close  
**Output:** Average Price values

```
AVGPRICE = (Open + High + Low + Close) / 4
```

### MEDPRICE - Median Price
**Input:** High, Low  
**Output:** Median Price values

```
MEDPRICE = (High + Low) / 2
```

### TYPPRICE - Typical Price
**Input:** High, Low, Close  
**Output:** Typical Price values

```
TYPPRICE = (High + Low + Close) / 3
```

### WCLPRICE - Weighted Close Price
**Input:** High, Low, Close  
**Output:** Weighted Close Price values

```
WCLPRICE = (High + Low + 2 × Close) / 4
```

## Cycle Indicators (Indikator Siklus)

### HT_DCPERIOD - Hilbert Transform - Dominant Cycle Period
**Input:** Price series  
**Output:** Dominant Cycle Period

```
1. Apply Hilbert Transform to get In-phase (I) and Quadrature (Q)
2. Smooth I and Q components
3. Calculate instantaneous phase: Phase = atan(Q/I)
4. Calculate phase change: DeltaPhase = Phase - Phase[prev]
5. Smooth DeltaPhase
6. Period = 2π / DeltaPhase
```

### HT_DCPHASE - Hilbert Transform - Dominant Cycle Phase
**Input:** Price series  
**Output:** Dominant Cycle Phase

```
1. Apply Hilbert Transform to get I and Q components
2. Calculate instantaneous phase: Phase = atan(Q/I)
3. Accumulate phase change to get Dominant Cycle Phase
```

### HT_PHASOR - Hilbert Transform - Phasor Components
**Input:** Price series  
**Output:** In-phase component, Quadrature component

```
Apply Hilbert Transform:
In-phase (I) = Price
Quadrature (Q) = Hilbert Transform of Price
```

### HT_SINE - Hilbert Transform - SineWave
**Input:** Price series  
**Output:** Sine, Lead Sine

```
1. Calculate Dominant Cycle Period
2. Sine = sin(Phase)
3. Lead Sine = sin(Phase + π/4)
```

### HT_TRENDMODE - Hilbert Transform - Trend vs Cycle Mode
**Input:** Price series  
**Output:** Trend Mode (0 = Cycle, 1 = Trend)

```
1. Calculate Dominant Cycle Period
2. If Period < threshold: Mode = 1 (Trend)
3. If Period >= threshold: Mode = 0 (Cycle)
```

## Pattern Recognition (Pola Candlestick)

### Pola Candlestick Umum
**Output:** 
- 100 = Bullish pattern
- -100 = Bearish pattern  
- 0 = No pattern

### CDL2CROWS - Two Crows
```
Conditions:
1. First candle: Long white (Close[2] > Open[2])
2. Second candle: Black, opens above first (Open[1] > Close[2])
3. Third candle: Black, opens within second body, closes below first close
Pattern = -100 (Bearish reversal)
```

### CDL3BLACKCROWS - Three Black Crows
```
Conditions:
1. Three consecutive long black candles
2. Each opens within previous body
3. Each closes near its low
Pattern = -100 (Bearish reversal)
```

### CDL3INSIDE - Three Inside Up/Down
```
Three Inside Up (Bullish):
1. First: Long black candle
2. Second: White candle within first body (Harami)
3. Third: White close above second close

Three Inside Down (Bearish):
1. First: Long white candle
2. Second: Black candle within first body
3. Third: Black close below second close
```

### CDL3LINESTRIKE - Three-Line Strike
```
Bullish:
1. Three consecutive black candles
2. Fourth white candle opens below third low, closes above first open

Bearish:
1. Three consecutive white candles
2. Fourth black candle opens above third high, closes below first open
```

### CDL3OUTSIDE - Three Outside Up/Down
```
Three Outside Up:
1. First: Black candle
2. Second: White engulfing
3. Third: White close above second

Three Outside Down:
1. First: White candle
2. Second: Black engulfing
3. Third: Black close below second
```

### CDL3STARSINSOUTH - Three Stars In The South
```
Conditions:
1. First: Long black with long lower shadow
2. Second: Black, smaller body, shadow within first
3. Third: Small black marubozu within second range
Pattern = 100 (Bullish reversal)
```

### CDL3WHITESOLDIERS - Three Advancing White Soldiers
```
Conditions:
1. Three consecutive white candles
2. Each opens within previous body
3. Each closes near its high
Pattern = 100 (Bullish reversal)
```

### CDLABANDONEDBABY - Abandoned Baby
```
Bullish:
1. First: Black candle
2. Second: Doji with gap down
3. Third: White with gap up

Bearish:
1. First: White candle  
2. Second: Doji with gap up
3. Third: Black with gap down
```

### CDLADVANCEBLOCK - Advance Block
```
Conditions:
1. Three white candles, each smaller
2. Upper shadows getting longer
3. Signs of buying pressure weakening
Pattern = -100 (Bearish reversal warning)
```

### CDLBELTHOLD - Belt-hold
```
Bullish Belt-hold:
- Long white candle
- Opens at low (no lower shadow)
- Closes well above open

Bearish Belt-hold:
- Long black candle
- Opens at high (no upper shadow)
- Closes well below open
```

### CDLBREAKAWAY - Breakaway
```
Bullish Breakaway:
1. Long black candle
2. Black with gap down
3. Two more candles (any color)
4. Long white closing within first gap

Bearish Breakaway:
1. Long white candle
2. White with gap up
3. Two more candles
4. Long black closing within first gap
```

### CDLCLOSINGMARUBOZU - Closing Marubozu
```
Bullish: White candle with no upper shadow (Close = High)
Bearish: Black candle with no lower shadow (Close = Low)
```

### CDLCONCEALBABYSWALL - Concealing Baby Swallow
```
Conditions:
1. Two black marubozu candles
2. Third: Black with gap down, upper shadow
3. Fourth: Black engulfing third completely
Pattern = 100 (Bullish reversal)
```

### CDLCOUNTERATTACK - Counterattack
```
Bullish:
1. Black candle in downtrend
2. White candle opens much lower, closes at first close

Bearish:
1. White candle in uptrend
2. Black candle opens much higher, closes at first close
```

### CDLDARKCLOUDCOVER - Dark Cloud Cover
```
Conditions:
1. First: Long white candle
2. Second: Black opens above first high
3. Closes below midpoint of first body
Pattern = -100 (Bearish reversal)
```

### CDLDOJI - Doji
```
Condition: Open = Close (or very close)
Body to range ratio < 0.1
```

### CDLDOJISTAR - Doji Star
```
Bullish: White candle + gap up + Doji
Bearish: Black candle + gap down + Doji
```

### CDLDRAGONFLYDOJI - Dragonfly Doji
```
Conditions:
1. Open = High = Close
2. Long lower shadow
3. No upper shadow
Pattern context determines bullish/bearish
```

### CDLENGULFING - Engulfing Pattern
```
Bullish Engulfing:
1. Black candle
2. White candle completely engulfs first

Bearish Engulfing:
1. White candle
2. Black candle completely engulfs first
```

### CDLEVENINGDOJISTAR - Evening Doji Star
```
Conditions:
1. Long white candle
2. Doji with gap up
3. Black candle closing below first midpoint
Pattern = -100 (Bearish reversal)
```

### CDLEVENINGSTAR - Evening Star
```
Conditions:
1. Long white candle
2. Small body with gap up
3. Black candle closing below first midpoint
Pattern = -100 (Bearish reversal)
```

### CDLGAPSIDESIDEWHITE - Up/Down-gap side-by-side white lines
```
Up-gap: Gap up + two white candles at same level = 100 (Continuation)
Down-gap: Gap down + two white candles at same level = -100 (Continuation)
```

### CDLGRAVESTONEDOJI - Gravestone Doji
```
Conditions:
1. Open = Low = Close
2. Long upper shadow
3. No lower shadow
Pattern context determines bullish/bearish
```

### CDLHAMMER - Hammer
```
Conditions:
1. Small body at upper range
2. Lower shadow >= 2 × body
3. Little/no upper shadow
4. Appears in downtrend
Pattern = 100 (Bullish reversal)
```

### CDLHANGINGMAN - Hanging Man
```
Conditions:
1. Small body at upper range
2. Lower shadow >= 2 × body
3. Little/no upper shadow
4. Appears in uptrend
Pattern = -100 (Bearish reversal)
```

### CDLHARAMI - Harami Pattern
```
Bullish Harami:
1. Long black candle
2. Small white within first body

Bearish Harami:
1. Long white candle  
2. Small black within first body
```
