# fuzzy-date

Python module to convert various time strings into datetime objects, written in Rust.

## Date conversion

```python
import fuzzydate as fd

# If current time is April 1st 2023 12PM UTC...

fd.to_datetime('1 hour ago')             # 2023-04-01 11:00:00+00:00
fd.to_datetime('last week')              # 2023-03-20 12:00:00+00:00
fd.to_datetime('-1 week')                # 2023-03-25 12:00:00+00:00
fd.to_datetime('last week midnight')     # 2023-03-20 00:00:00+00:00
fd.to_datetime('-1d 2h 5min 10s')        # 2023-03-31 09:54:50+00:00
fd.to_datetime('tomorrow')               # 2023-04-02 00:00:00+00:00
fd.to_datetime('prev Monday')            # 2023-03-27 00:00:00+00:00
fd.to_datetime('last day of this month') # 2023-04-30 00:00:00+00:00

# Anything invalid raises a ValueError

fd.to_datetime('next Summer')
# ValueError: Unable to convert "next Summer" into datetime
```

## Time duration

```python
import fuzzydate as fd

fd.to_seconds('1h 4min') # 3840.0
fd.to_seconds('+2 days') # 172800.0
fd.to_seconds('-1 hour') # -3600.0
fd.to_seconds('1 week')  # 604800.0

# Anything other than an exact length of time raises a ValueError

fd.to_seconds('last week')
# ValueError: Unable to convert "last week" into seconds

# Because years and months have varying amount of seconds, using 
# them raises a ValueError

fd.to_seconds('1m 2w 30min')
# ValueError: Converting months into seconds is not supported
```

## Localization

```python
import fuzzydate as fd

fd.config.add_tokens({
    'måndag': fd.token.WDAY_MON,
    'dagar': fd.token.LONG_UNIT_DAY,
})

fd.config.add_patterns({
    'nästa [wday]': fd.pattern.NEXT_WDAY,
})

assert fd.to_date('next Monday') == fd.to_date('nästa Måndag')
assert fd.to_date('+5 days') == fd.to_date('+5 dagar')
assert fd.to_seconds('+5 days') == fd.to_seconds('+5 dagar')
```

## Requirements

- Python >= 3.8

## Installation

```
pip install fuzzy-date 
```

## Syntax support

### Special

- Date `now`, `today`, `tomorrow`, `yesterday`
- Time of day `midnight`

### Relative

- Adjustment `last`, `prev`, `this`, `next` or `+`, `-`
- Units `next week`, `next month`, `next year`
- Weekdays `next Mon`, `next Monday`
- Numeric `(s)ec`, `min`, `(h)r`, `(d)ay`, `(w)eek`, `(m)onth`, `(y)ear`
- Ranges `last/first day of`

### Fixed

- Unix timestamp `@1680307200`
- Dates `2023-04-01`, `04/01/2023`, `01.04.2023`
- Textual dates `April 1st 2023`, `April 1 2023`, `1 April 2023`
- Day and month `April 1st`, `April 1`, `1 April`
- Datetime formats `2023-04-01 12:00`, `2023-04-01 12:00:00`

## Methods

### Conversion

```python
fuzzydate.to_date(
    source: str,
    today: datetime.date = None,
    weekday_start_mon: bool = True) -> datetime.date

fuzzydate.to_datetime(
    source: str,
    now: datetime.datetime = None,
    weekday_start_mon: bool = True) -> datetime.datetime
    
fuzzydate.to_seconds(
    source: str) -> float
```

### Configuration

```python
# Read-only
fuzzydate.config.patterns: dict[str, str]
fuzzydate.config.tokens: dict[str, int]

fuzzydate.config.add_patterns(
    tokens: dict[str, str]) -> None

fuzzydate.config.add_tokens(
    tokens: dict[str, int]) -> None
```

## Background

This library was born out of the need to accept various user inputs for date range start and end
times, to convert user time tracking entries into exact durations etc. All very much akin to what 
[timelib](https://github.com/derickr/timelib) does. Other implementations do exist, but they did not quite work for me - usually 
missing support for some key wording I needed, or having issues with timezone handling. 

Also, I kinda wanted to learn Rust via some example project as well.

## License

MIT

