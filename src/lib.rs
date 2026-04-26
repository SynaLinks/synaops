// License Apache 2.0: (c) 2025 Yoan Sallami (Synalinks Team)

// pyo3 0.22's #[pyfunction] macro expands returns with a redundant `.into()`,
// which clippy flags on every function. Silence at module scope.
#![allow(clippy::useless_conversion)]

use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use serde_json::Value;

mod json_schema_utils;
mod json_utils;
mod nlp_utils;
mod regex_cache;

fn to_value(obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    depythonize(obj).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

fn from_value<'py>(py: Python<'py>, v: &Value) -> PyResult<Bound<'py, PyAny>> {
    pythonize(py, v).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

fn regex_err<E: ToString>(e: E) -> PyErr {
    pyo3::exceptions::PyValueError::new_err(e.to_string())
}

// --- JSON object operations ---

#[pyfunction]
#[pyo3(signature = (json, prefix))]
fn prefix_json<'py>(
    py: Python<'py>,
    json: &Bound<'py, PyAny>,
    prefix: &str,
) -> PyResult<Bound<'py, PyAny>> {
    let v = to_value(json)?;
    from_value(py, &json_utils::prefix_json(v, prefix))
}

#[pyfunction]
#[pyo3(signature = (json, suffix))]
fn suffix_json<'py>(
    py: Python<'py>,
    json: &Bound<'py, PyAny>,
    suffix: &str,
) -> PyResult<Bound<'py, PyAny>> {
    let v = to_value(json)?;
    from_value(py, &json_utils::suffix_json(v, suffix))
}

#[pyfunction]
fn concatenate_json<'py>(
    py: Python<'py>,
    json1: &Bound<'py, PyAny>,
    json2: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    let a = to_value(json1)?;
    let b = to_value(json2)?;
    from_value(py, &json_utils::concatenate_json(a, b))
}

#[pyfunction]
fn factorize_json<'py>(
    py: Python<'py>,
    json: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    let v = to_value(json)?;
    from_value(py, &json_utils::factorize_json(v))
}

#[pyfunction]
#[pyo3(signature = (json, mask=None, pattern=None, recursive=true))]
fn out_mask_json<'py>(
    py: Python<'py>,
    json: &Bound<'py, PyAny>,
    mask: Option<Vec<String>>,
    pattern: Option<String>,
    recursive: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let v = to_value(json)?;
    let mask_refs: Option<Vec<&str>> =
        mask.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect());
    let result =
        json_utils::out_mask_json(&v, mask_refs.as_deref(), pattern.as_deref(), recursive)
            .map_err(regex_err)?;
    from_value(py, &result)
}

#[pyfunction]
#[pyo3(signature = (json, mask=None, pattern=None, recursive=true))]
fn in_mask_json<'py>(
    py: Python<'py>,
    json: &Bound<'py, PyAny>,
    mask: Option<Vec<String>>,
    pattern: Option<String>,
    recursive: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let v = to_value(json)?;
    let mask_refs: Option<Vec<&str>> =
        mask.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect());
    let result =
        json_utils::in_mask_json(&v, mask_refs.as_deref(), pattern.as_deref(), recursive)
            .map_err(regex_err)?;
    from_value(py, &result)
}

// --- JSON schema operations ---

#[pyfunction]
fn standardize_schema<'py>(
    py: Python<'py>,
    schema: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    let v = to_value(schema)?;
    from_value(py, &json_schema_utils::standardize_schema(v))
}


#[pyfunction]
fn prefix_schema<'py>(
    py: Python<'py>,
    schema: &Bound<'py, PyAny>,
    prefix: &str,
) -> PyResult<Bound<'py, PyAny>> {
    let v = to_value(schema)?;
    from_value(py, &json_schema_utils::prefix_schema(v, prefix))
}

#[pyfunction]
fn suffix_schema<'py>(
    py: Python<'py>,
    schema: &Bound<'py, PyAny>,
    suffix: &str,
) -> PyResult<Bound<'py, PyAny>> {
    let v = to_value(schema)?;
    from_value(py, &json_schema_utils::suffix_schema(v, suffix))
}

#[pyfunction]
fn concatenate_schema<'py>(
    py: Python<'py>,
    schema1: &Bound<'py, PyAny>,
    schema2: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    let a = to_value(schema1)?;
    let b = to_value(schema2)?;
    from_value(py, &json_schema_utils::concatenate_schema(a, b))
}

#[pyfunction]
fn factorize_schema<'py>(
    py: Python<'py>,
    schema: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    let v = to_value(schema)?;
    from_value(py, &json_schema_utils::factorize_schema(v))
}

#[pyfunction]
#[pyo3(signature = (schema, mask=None, pattern=None, recursive=true))]
fn out_mask_schema<'py>(
    py: Python<'py>,
    schema: &Bound<'py, PyAny>,
    mask: Option<Vec<String>>,
    pattern: Option<String>,
    recursive: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let v = to_value(schema)?;
    let mask_refs: Option<Vec<&str>> =
        mask.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect());
    let result = json_schema_utils::out_mask_schema(
        v,
        mask_refs.as_deref(),
        pattern.as_deref(),
        recursive,
    )
    .map_err(regex_err)?;
    from_value(py, &result)
}

#[pyfunction]
#[pyo3(signature = (schema, mask=None, pattern=None, recursive=true))]
fn in_mask_schema<'py>(
    py: Python<'py>,
    schema: &Bound<'py, PyAny>,
    mask: Option<Vec<String>>,
    pattern: Option<String>,
    recursive: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let v = to_value(schema)?;
    let mask_refs: Option<Vec<&str>> =
        mask.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect());
    let result = json_schema_utils::in_mask_schema(
        v,
        mask_refs.as_deref(),
        pattern.as_deref(),
        recursive,
    )
    .map_err(regex_err)?;
    from_value(py, &result)
}

#[pymodule]
fn synaops(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(prefix_json, m)?)?;
    m.add_function(wrap_pyfunction!(suffix_json, m)?)?;
    m.add_function(wrap_pyfunction!(concatenate_json, m)?)?;
    m.add_function(wrap_pyfunction!(factorize_json, m)?)?;
    m.add_function(wrap_pyfunction!(out_mask_json, m)?)?;
    m.add_function(wrap_pyfunction!(in_mask_json, m)?)?;

    m.add_function(wrap_pyfunction!(standardize_schema, m)?)?;
    m.add_function(wrap_pyfunction!(prefix_schema, m)?)?;
    m.add_function(wrap_pyfunction!(suffix_schema, m)?)?;
    m.add_function(wrap_pyfunction!(concatenate_schema, m)?)?;
    m.add_function(wrap_pyfunction!(factorize_schema, m)?)?;
    m.add_function(wrap_pyfunction!(out_mask_schema, m)?)?;
    m.add_function(wrap_pyfunction!(in_mask_schema, m)?)?;

    Ok(())
}
