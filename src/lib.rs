use serde_json::Value as JsonValue;

use jsonpath_lib as jsonpath;
use jsonpath_lib::JsonPathError;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread::sleep;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;



#[pyfunction]
pub fn read_value_from_file(path: String, expression: String) -> PyResult<String> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).expect("Could not read file, check permissions!");
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as json value.
    let v: JsonValue = serde_json::from_reader(reader).expect("JSON file might be corrupt.");
    let mut selector = jsonpath::selector(&v);

    Ok(String::from(selector(&expression).unwrap().get(0).unwrap().as_str().unwrap()))
}

#[pymodule]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_value_from_file))?;

    Ok(())
}


/*pub fn read_values_from_file(path: String, expression: String) -> Vec<String> {
    let mut values = Vec::new();

    // Open the file in read-only mode with buffer.
    let file = File::open(path).expect("Could not read file, check permissions!");
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as json value.
    let v: JsonValue = serde_json::from_reader(reader).expect("JSON file might be corrupt.");
    let mut selector = jsonpath::selector(&v);

    for value in selector(&expression).unwrap() {
        values.push(String::from(value.as_str().unwrap()));
    }

    values
}


fn get_value_from_json_str(json: String, expression: String) -> String {
    let v: JsonValue = serde_json::from_str(&json).expect("JSON string might be corrupt.");
    let mut selector = jsonpath::selector(&v);

    String::from(selector(&expression).unwrap().get(0).unwrap().as_str().unwrap())
}

fn get_values_from_json_str(json: String, expression: String) -> Vec<String> {
    let mut values = vec![String];

    let v: JsonValue = serde_json::from_str(&json).expect("JSON string might be corrupt.");
    let mut selector = jsonpath::selector(&v);

    for value in selector(&expression).unwrap() {
        values.push(String::from(value.as_str().unwrap()));
    }

    values
}*/

#[cfg(test)]
mod tests {
    use crate::read_value_from_file;
    use crate::read_values_from_file;

    #[test]
    fn read_value_from_file_test() {
        let path = String::from("vda4938_2.txt_out.json");
        let expression = String::from("$.segments[0].segment_tag");
        let v = read_value_from_file(path, expression);
        println!("{:?}", &v);
        assert_eq!(&v, "UNB");
    }

    #[test]
    fn read_values_from_file_test() {
        let path = String::from("vda4938_2.txt_out.json");
        let expression = String::from("$.segments[*].segment_tag");
        let v = read_values_from_file(path, expression);
        println!("{:?}", &v);
        assert_eq!("UNB", "UNB");
    }
}
