#![feature(proc_macro)]
#![feature(proc_macro_path_invoc)]


use serde_json::Value as JsonValue;

use jsonpath_lib as jsonpath;
use jsonpath_lib::JsonPathError;

use std::fs::File;
use std::io::BufReader;

use pyo3::prelude::*;


#[py::modinit(_jsonpath_pyrs)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "read_value_from_file")]
    pub fn read_value_from_file(py: Python, path: &str, expression: &str) -> PyResult<PyObject> {
        // Open the file in read-only mode with buffer.
        let file = File::open(path).expect("Could not read file, check permissions!");
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as json value.
        let v: JsonValue = serde_json::from_reader(reader).expect("JSON file might be corrupt.");
        let mut selector = jsonpath::selector(&v);

        Ok(String::from(selector(&expression).unwrap().get(0).unwrap().as_str().unwrap()).to_object(py))
    }

    #[pyfn(m, "read_values_from_file")]
    fn read_values_from_file(py: Python, path: &str, expression: &str) -> PyResult<PyObject> {
        let mut values = Vec::new();

        // Open the file in read-only mode with buffer.
        let file = File::open(path).expect("Could not read file, check permissions!");
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as json value.
        let v: JsonValue = serde_json::from_reader(reader).expect("JSON file might be corrupt.");
        let mut selector = jsonpath::selector(&v);

        for value in selector(expression).unwrap() {
            values.push(String::from(value.as_str().unwrap()));
        }

        Ok(values.to_object(py))
    }

    #[pyfn(m, "get_value_from_json_str")]
    fn get_value_from_json_str(py: Python, json: &str, expression: &str) -> PyResult<PyObject> {
        let v: JsonValue = serde_json::from_str(json).expect("JSON string might be corrupt.");
        let mut selector = jsonpath::selector(&v);

        Ok(String::from(selector(&expression).unwrap().get(0).unwrap().as_str().unwrap()).to_object(py))
    }

    #[pyfn(m, "get_values_from_json_str")]
    fn get_values_from_json_str(py: Python, json: &str, expression: &str) -> PyResult<PyObject> {
        let mut values = vec![];

        let v: JsonValue = serde_json::from_str(json).expect("JSON string might be corrupt.");
        let mut selector = jsonpath::selector(&v);

        for value in selector(expression).unwrap() {
            values.push(String::from(value.as_str().unwrap()));
        }

        Ok(values.to_object(py))
    }

    Ok(())
}