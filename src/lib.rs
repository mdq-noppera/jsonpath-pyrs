use serde_json::Value as JsonValue;

use jsonpath_lib as jsonpath;

use std::fs::File;
use std::io::BufReader;

use pyo3::prelude::*;

#[py::modinit(_jsonpath_pyrs)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "read_json_file")]
    fn read_json_file(py: Python, path: &str, expression: &str) -> PyResult<PyObject> {
        let mut results: Vec<PyObject> = vec![];

        // Open the file in read-only mode with buffer.
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                println!("Could not read file, check permissions or path!");
                return Ok(results.to_object(py));
            }
        };

        let reader = BufReader::new(file);

        // Read the JSON contents of the file as json value.
        let v: JsonValue = match serde_json::from_reader(reader) {
            Ok(v) => v,
            Err(e) => {
                println!("JSON syntax might be corrupt.");
                return Ok(results.to_object(py));
            }
        };

        let mut selector = jsonpath::selector(&v);

        let result = match selector(expression) {
            Ok(r) => r,
            Err(e) => {
                println!("JSON path might be flawed.");
                return Ok(results.to_object(py));
            }
        };

        for r in result {
            if r.is_boolean() {
                results.push(r.as_bool().unwrap().to_object(py))
            } else if r.is_i64() {
                results.push(r.as_i64().unwrap().to_object(py))
            } else if r.is_f64() {
                results.push(r.as_f64().unwrap().to_object(py))
            } else if r.is_string() {
                results.push(r.as_str().unwrap().to_object(py))
            } else if r.is_array() {
                let mut sub_results: Vec<PyObject> = vec![];

                for r2 in r.as_array().unwrap() {
                    if r2.is_boolean() {
                        sub_results.push(r2.as_bool().unwrap().to_object(py))
                    } else if r2.is_i64() {
                        sub_results.push(r2.as_i64().unwrap().to_object(py))
                    } else if r2.is_f64() {
                        sub_results.push(r2.as_f64().unwrap().to_object(py))
                    } else if r2.is_string() {
                        sub_results.push(r2.as_str().unwrap().to_object(py))
                    }
                }
                results.push(sub_results.to_object(py))
            }
        }

        Ok(results.to_object(py))
    }

    #[pyfn(m, "read_json_string")]
    fn read_json_string(py: Python, json: &str, expression: &str) -> PyResult<PyObject> {
        let mut results: Vec<PyObject> = vec![];

        let v: JsonValue = match serde_json::from_str(json) {
            Ok(v) => v,
            Err(e) => {
                println!("JSON syntax might be corrupt.");
                return Ok(results.to_object(py));
            }
        };

        let mut selector = jsonpath::selector(&v);

        let result = match selector(expression) {
            Ok(r) => r,
            Err(e) => {
                println!("JSON path might be flawed.");
                return Ok(results.to_object(py));
            }
        };

        for r in result {
            if r.is_boolean() {
                results.push(r.as_bool().unwrap().to_object(py))
            } else if r.is_i64() {
                results.push(r.as_i64().unwrap().to_object(py))
            } else if r.is_f64() {
                results.push(r.as_f64().unwrap().to_object(py))
            } else if r.is_string() {
                results.push(r.as_str().unwrap().to_object(py))
            } else if r.is_array() {
                let mut sub_results: Vec<PyObject> = vec![];

                for r2 in r.as_array().unwrap() {
                    if r2.is_boolean() {
                        sub_results.push(r2.as_bool().unwrap().to_object(py))
                    } else if r2.is_i64() {
                        sub_results.push(r2.as_i64().unwrap().to_object(py))
                    } else if r2.is_f64() {
                        sub_results.push(r2.as_f64().unwrap().to_object(py))
                    } else if r2.is_string() {
                        sub_results.push(r2.as_str().unwrap().to_object(py))
                    }
                }
                results.push(sub_results.to_object(py))
            }
        }

        Ok(results.to_object(py))
    }

    Ok(())
}
