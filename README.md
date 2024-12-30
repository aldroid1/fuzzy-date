# fuzzy-date

[![PyPI version shields.io](https://img.shields.io/pypi/v/fuzzy-date.svg?color=blue)](https://pypi.python.org/pypi/fuzzy-date/) ‎
[![PyPI download month](https://img.shields.io/pypi/dm/fuzzy-date.svg?color=blue)](https://pypistats.org/packages/fuzzy-date) ‎

Python module to convert various time strings into datetime objects, written in Rust.

## Date conversion

```python
import fuzzydate as fd

# If current time is April 1st 2023 12PM UTC...

fd.to_datetime('1 hour ago')             # 2023-04-01 11:00:00+00:00
fd.to_datetime('last week')              # 2023-03-20 12:00:00+00:00
fd.to_datetime('past 2 weeks')           # 2023-03-18 12:00:00+00:00
fd.to_datetime('-1 week')                # 2023-03-25 12:00:00+00:00
fd.to_datetime('last week midnight')     # 2023-03-20 00:00:00+00:00
fd.to_datetime('-1d 2h 5min 10s')        # 2023-03-31 09:54:50+00:00
fd.to_datetime('tomorrow')               # 2023-04-02 00:00:00+00:00
fd.to_datetime('prev Monday')            # 2023-03-27 00:00:00+00:00
fd.to_datetime('prev June')              # 2022-06-01 00:00:00+00:00
fd.to_datetime('last day of this month') # 2023-04-30 00:00:00+00:00

# Anything invalid raises a ValueError

fd.to_datetime('next Summer')
# ValueError: Unable to convert "next Summer" into datetime
```

## Time duration

### Duration seconds

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

### Duration string

```python
import fuzzydate as fd

fd.to_duration(3840.0)                       # 1hr 4min
fd.to_duration(3840.0, units='long')         # 1 hour 4 minutes
fd.to_duration(3840.0, units='short')        # 1h 4min
fd.to_duration(3840.0, max='min', min='min') # 64min
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

fd.config.units = {
    fd.unit.DAY: 'dag',
    fd.unit.DAYS: 'dagar',
}

assert fd.to_duration(86400.0) == '1 dag'
```

## Requirements

- Python >= 3.9

## Installation

```
pip install fuzzy-date 
```

## Syntax support

### Special

- Date `now`, `today`, `tomorrow`, `yesterday`
- Time of day `midnight`

### Relative

- Adjustment `last`, `prev`, `past`, `this`, `next` or `+`, `-`
- Units `next week`, `next month`, `next year`
- Weekdays `next Mon`, `next Monday`
- Months `next Jan`, `next January`
- Numeric `(s)ec`, `min`, `(h)r`, `(d)ay`, `(w)eek`, `(m)onth`, `(y)ear`
- Ranges `last/first day of`

### Fixed

- Unix timestamp `@1680307200`
- Numeric dates `2023-04-01`, `20230401`, `04/01/2023`, `01.04.2023`
- Textual dates `April 1st 2023`, `April 1 2023`, `1 April 2023`, `1. April 2023`
- Combination dates `01-April-2023`, `April-01-2023`, `2023-April-01`
- Day and month `April 1st`, `April 1`, `1 April`, `1. April`, `1st of April`
- Month and year `April`, `April 2023`
- Datetime formats `2023-04-01 12:00`, `2023-04-01 12:00:00`, `2023-04-01 12:00:00.410`
- Time of day `2pm`, `2:00 pm`

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
    
fuzzydate.to_duration(
    seconds: float, 
    units: str = None, 
    max: str = 'w', 
    min: str = 's') -> str
    
fuzzydate.to_seconds(
    source: str) -> float
```

### Configuration

```python
# Read-only
fuzzydate.config.patterns: dict[str, str]
fuzzydate.config.tokens: dict[str, int]

# Read-write
fuzzydate.config.units: dict[str, str]
fuzzydate.config.units_long: dict[str, str]
fuzzydate.config.units_short: dict[str, str]

fuzzydate.config.add_patterns(
    tokens: dict[str, str]) -> None

fuzzydate.config.add_tokens(
    tokens: dict[str, int]) -> None
```

## Background

This library was born out of the need to accept various user inputs for date range start and end
times, to convert user time tracking entries into exact durations etc. All very much alike to what
[timelib](https://github.com/derickr/timelib) does.

Other implementations are available, but I did not find one that would have worked for me - usually
they were missing support for some key wording I needed, or handled user vagueness and timezones in
a different way.

Also, I kinda wanted to learn Rust via some example project as well.

## License

MIT

