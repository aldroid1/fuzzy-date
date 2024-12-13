use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc};
use pyo3::types::{PyDate, PyDateTime};
use pyo3::{Bound, Py, PyErr, Python};

/// Turn optional date from Python into DateTime with a timezone,
/// setting UTC as timezone and time as midnight
pub(crate) fn into_date(py: Python, value: Option<Bound<PyDate>>) -> Result<DateTime<FixedOffset>, PyErr> {
    match value {
        Some(v) => {
            let real_value: Py<PyDate> = v.unbind();
            let date_value = real_value.extract::<NaiveDate>(py)?;
            let date_time = NaiveDateTime::from(date_value);
            Ok(Utc.from_local_datetime(&date_time).unwrap().fixed_offset())
        }
        None => {
            let system_date = NaiveDateTime::from(Utc::now().date_naive());
            Ok(Utc.from_local_datetime(&system_date).unwrap().fixed_offset())
        }
    }
}

/// Turn optional datetime from Python object into DateTime with a timezone
/// information, defaulting to UTC when missing
pub(crate) fn into_datetime(py: Python, value: Option<Bound<PyDateTime>>) -> Result<DateTime<FixedOffset>, PyErr> {
    let py_value: Py<PyDateTime> = match value {
        Some(v) => v.unbind(),
        None => return Ok(Utc::now().fixed_offset()),
    };

    let naive_value = match py_value.extract(py) {
        Ok(v) => return Ok(v),
        Err(_) => py_value.extract::<NaiveDateTime>(py)?,
    };

    Ok(Utc.from_local_datetime(&naive_value).unwrap().fixed_offset())
}

#[cfg(test)]
mod test {
    use super::*;
    use pyo3::types::PyTzInfo;
    use pyo3::{Bound, IntoPyObject, PyResult, Python};

    #[test]
    fn test_into_date() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let expect_value = Utc::now().format("%Y-%m-%d 00:00:00 +00:00").to_string();
            let result_value = into_date(py, None);
            assert_eq!(result_value.unwrap().to_string(), expect_value);
        });

        Python::with_gil(|py| {
            let test_value = PyDate::new(py, 2023, 4, 1);
            assert_date(py, test_value, "2023-04-01 00:00:00 +00:00");
        });
    }

    #[test]
    fn test_into_datetime() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let expect_value = Utc::now().format("%Y-%m-%d %H:").to_string();
            let result_value = into_datetime(py, None);
            assert!(result_value.unwrap().to_string().starts_with(expect_value.as_str()));
        });

        Python::with_gil(|py| {
            let test_value = PyDateTime::new(py, 2023, 4, 1, 15, 2, 1, 7, None);
            assert_datetime(py, test_value, "2023-04-01 15:02:01.000007 +00:00");
        });

        Python::with_gil(|py| {
            let tz_offset = FixedOffset::east_opt(5 * 60 * 60).unwrap();
            let tz_bound: Bound<PyTzInfo> = tz_offset.into_pyobject(py).unwrap();
            let test_value = PyDateTime::new(py, 2023, 4, 1, 15, 2, 1, 7, Some(&tz_bound));
            assert_datetime(py, test_value, "2023-04-01 15:02:01.000007 +05:00");
        });
    }

    fn assert_date(py: Python, test_value: PyResult<Bound<PyDate>>, expect_value: &str) {
        let date_value: Bound<PyDate> = test_value.unwrap().into_pyobject(py).unwrap();
        let result_value = into_date(py, Some(date_value));
        assert_eq!(result_value.unwrap().to_string(), expect_value);
    }

    fn assert_datetime(py: Python, test_value: PyResult<Bound<PyDateTime>>, expect_value: &str) {
        let date_value: Bound<PyDateTime> = test_value.unwrap().into_pyobject(py).unwrap();
        let result_value = into_datetime(py, Some(date_value));
        assert_eq!(result_value.unwrap().to_string(), expect_value);
    }
}
