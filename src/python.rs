use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc};
use pyo3::types::{PyDate, PyDateTime};
use pyo3::{Py, PyErr, Python};

/// Turn optional date from Python into DateTime with a timezone,
/// setting UTC as timezone and time as midnight
pub(crate) fn into_date(
    py: Python,
    value: Option<Py<PyDate>>) -> Result<DateTime<FixedOffset>, PyErr> {
    if value.is_none() {
        let system_date = NaiveDateTime::from(Utc::now().date_naive());
        return Ok(Utc.from_local_datetime(&system_date).unwrap().fixed_offset());
    }

    let date_value: NaiveDate = value.unwrap().extract::<NaiveDate>(py)?;
    let date_time = NaiveDateTime::from(date_value);
    Ok(Utc.from_local_datetime(&date_time).unwrap().fixed_offset())
}

/// Turn optional datetime from Python object into DateTime with a timezone
/// information, defaulting to UTC when missing
pub(crate) fn into_datetime(
    py: Python,
    value: Option<Py<PyDateTime>>) -> Result<DateTime<FixedOffset>, PyErr> {
    if value.is_none() {
        return Ok(Utc::now().fixed_offset());
    }

    let real_value = value.unwrap();
    let with_timezone = real_value.extract::<DateTime<FixedOffset>>(py);

    if with_timezone.is_ok() {
        return Ok(with_timezone?);
    }

    let without_timezone = real_value.extract::<NaiveDateTime>(py)?;
    Ok(Utc.from_local_datetime(&without_timezone).unwrap().fixed_offset())
}

#[cfg(test)]
mod test {
    use super::*;
    use pyo3::types::PyTzInfo;
    use pyo3::{Bound, PyObject, PyResult, Python, ToPyObject};

    #[test]
    fn test_into_date() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let expect_value = Utc::now().format("%Y-%m-%d 00:00:00 +00:00").to_string();
            let result_value = into_date(py, Option::from(None));
            assert_eq!(result_value.unwrap().to_string(), expect_value);
        });

        Python::with_gil(|py| {
            let test_value = PyDate::new_bound(py, 2023, 4, 1);
            assert_date(py, test_value, "2023-04-01 00:00:00 +00:00");
        });
    }

    #[test]
    fn test_into_datetime() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let expect_value = Utc::now().format("%Y-%m-%d %H:").to_string();
            let result_value = into_datetime(py, Option::from(None));
            assert!(result_value.unwrap().to_string().starts_with(expect_value.as_str()));
        });

        Python::with_gil(|py| {
            let test_value = PyDateTime::new_bound(py, 2023, 4, 1, 15, 2, 1, 7, None);
            assert_datetime(py, test_value, "2023-04-01 15:02:01.000007 +00:00");
        });

        Python::with_gil(|py| {
            let tz_offset = FixedOffset::east_opt(5 * 60 * 60).unwrap();
            let tz_value: PyObject = tz_offset.to_object(py);
            let tz_bound: Bound<PyTzInfo> = tz_value.extract::<Bound<PyTzInfo>>(py).unwrap();
            let test_value = PyDateTime::new_bound(py, 2023, 4, 1, 15, 2, 1, 7, Option::from(&tz_bound));
            assert_datetime(py, test_value, "2023-04-01 15:02:01.000007 +05:00");
        });
    }

    fn assert_date(py: Python, test_value: PyResult<Bound<PyDate>>, expect_value: &str) {
        let obj_value: PyObject = test_value.unwrap().to_object(py);
        let date_value: Py<PyDate> = obj_value.extract(py).unwrap();
        let result_value = into_date(py, Option::from(date_value));
        assert_eq!(result_value.unwrap().to_string(), expect_value);
    }

    fn assert_datetime(py: Python, test_value: PyResult<Bound<PyDateTime>>, expect_value: &str) {
        let obj_value: PyObject = test_value.unwrap().to_object(py);
        let date_value: Py<PyDateTime> = obj_value.extract(py).unwrap();
        let result_value = into_datetime(py, Option::from(date_value));
        assert_eq!(result_value.unwrap().to_string(), expect_value);
    }
}