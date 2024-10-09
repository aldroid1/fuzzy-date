# fuzzy-date

Python module to convert various time strings into datetime objects, written in Rust.

```python
import fuzzydate as fd

# If current time is April 1st 2023 12PM UTC...

fd.to_datetime('1 hour ago')             # 2023-04-01 11:00:00+00:00
fd.to_datetime('last week')              # 2023-03-20 12:00:00+00:00
fd.to_datetime('last week midnight')     # 2023-03-20 00:00:00+00:00
fd.to_datetime('-1d 2h 5min 10s')        # 2023-03-31 09:54:50+00:00
fd.to_datetime('tomorrow midnight')      # 2023-04-02 00:00:00+00:00
fd.to_datetime('prev Monday')            # 2023-03-27 12:00:00+00:00
fd.to_datetime('last day of this month') # 2023-04-30 00:00:00+00:00

# Anything invalid raises a ValueError

fd.to_datetime('next Summer')
# ValueError: Unable to convert "next Summer" into datetime
```

## Requirements

- Python >= 3.8

## Installation

```
pip install fuzzy-date 
```

## Methods

```python
fuzzydate.to_date(
    source: str,
    today: datetime.date = None,
    weekday_start_mon: bool = True) -> datetime.date

fuzzydate.to_datetime(
    source: str,
    now: datetime.datetime = None,
    weekday_start_mon: bool = True) -> datetime.datetime
```

## Syntax support

### Special

- Date `now`, `today`, `tomorrow`, `yesterday`
- Time of day `midnight`

### Relative

- Adjustment `last`, `prev`, `next` or `+`, `-`
- Units `next week`, `next month`, `next year`
- Weekdays `next Mon`, `next Monday`
- Numeric `(s)ec`, `min`, `(h)r`, `(d)ay`, `(w)eek`, `(m)onth`, `(y)ear`
- Ranges `last/first day of`

### Fixed

- Unix timestamp `@1680307200` 
- Dates `2023-04-01`, `04/01/2023`, `01.04.2023`
- Textual dates `April 1st 2023`, `April 1 2023`, `1 April 2023`
- Datetime formats `2023-04-01 12:00`, `2023-04-01 12:00:00`

## Background

This library was born out of the need to accept various user inputs for date range start and end
times, very much like [timelib](https://github.com/derickr/timelib) does. While other
implementations do exist, they did not quite work for me - usually missing support for some
key wording, or having timezone handling issues. Also, I kinda wanted to learn Rust via some
example project as well.

## License

MIT

